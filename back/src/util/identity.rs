use actix_identity::Identity;
use rand::distributions::Alphanumeric;
use rand::Rng;

pub trait GetIdentity {
    fn get(&self) -> String;
}
impl GetIdentity for Identity {
    // create a new identity if the user doesn't have one
    fn get(&self) -> String {
        if let Some(id) = self.identity() {
            id
        } else {
            let mut rng = rand::thread_rng();
            let new_id: String = std::iter::repeat(())
                .map(|_| rng.sample(Alphanumeric))
                .take(16)
                .collect();
            self.remember(new_id.clone());
            new_id
        }
    }
}
