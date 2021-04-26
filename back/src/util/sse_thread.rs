use crate::db::model::{Post, ThreadWithPosts};
use actix_web::web::{Bytes, Data};
use actix_web::Error;
use futures::{Stream, StreamExt};
use serde_json::json;
use std::collections::HashMap;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;
use tokio::time::{interval_at, Instant};

#[derive(Clone)]
pub enum Event<'a> {
    Thread(&'a Option<ThreadWithPosts>),
    Post(&'a Post),
    Ping,
}

impl<'a> Event<'a> {
    pub fn to_message(self) -> Bytes {
        let message = match self {
            Self::Thread(thread) => format!(
                "event: thread\ndata: {}\n\n",
                serde_json::to_string(thread).unwrap()
            ),
            Self::Post(post) => format!(
                "event: post\ndata: {}\n\n",
                serde_json::to_string(post).unwrap()
            ),
            Self::Ping => "event: ping\n\n".to_owned(),
        };
        Bytes::from(message)
    }
}

type Subscriber = Sender<Bytes>;

#[derive(Clone, Debug)]
pub struct Broadcaster(HashMap<i32, Vec<Subscriber>>);

impl Broadcaster {
    pub fn create() -> Data<Mutex<Broadcaster>> {
        let me = Data::new(Mutex::new(Self::new()));
        Self::spawn_ping(me.clone());
        me
    }

    fn new() -> Self {
        Broadcaster(HashMap::new())
    }
    // spawn a cleanup task that pings every subscriber once a minute and removes the dead ones
    fn spawn_ping(me: Data<Mutex<Broadcaster>>) {
        actix_rt::spawn(async move {
            let mut task = interval_at(Instant::now(), Duration::from_secs(60));
            while let Some(_) = task.next().await {
                me.lock().await.remove_dead();
            }
        })
    }

    fn remove_dead(&mut self) {
        let mut thread_subscribers = HashMap::with_capacity(self.0.capacity());

        for (thread, subscribers) in self.0.iter() {
            let mut live_subscribers = Vec::with_capacity(subscribers.capacity());

            for subscriber in subscribers.iter() {
                let mut subscriber = subscriber.clone();
                let result = subscriber.try_send(Event::Ping.to_message());
                if let Ok(_) = result {
                    live_subscribers.push(subscriber);
                }
            }
            if live_subscribers.len() > 0 {
                thread_subscribers.insert(*thread, live_subscribers);
            }
        }
        self.0 = thread_subscribers;
    }

    pub fn new_subscriber(&mut self, thread: Option<ThreadWithPosts>) -> Option<Client> {
        let (tx, rx) = channel(100);

        let res = tx.clone().try_send(Event::Thread(&thread).to_message());

        if let Err(_) = res {
            return None;
        }
        if let Some(thread) = thread {
            self.0.entry(thread.id).or_insert(Vec::new()).push(tx);
        }

        Some(Client(rx))
    }

    pub fn send(&self, thread_id: i32, event: Event) {
        if let Some(subscribers) = self.0.get(&thread_id) {
            for subscriber in subscribers.iter() {
                subscriber.clone().try_send(event.clone().to_message());
            }
        }
    }
}

pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_next(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
