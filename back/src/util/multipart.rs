use actix_multipart::{Field, Multipart};
use actix_web::web::{block, Bytes};
use serde::de::DeserializeOwned;
use std::ffi::OsStr;
use std::io::Write;
use std::path::Path;
use tokio::stream::StreamExt;

#[derive(Debug)]
pub enum MultipartError {
    Decode,
    Internal(String),
    BadRequest,
    InvalidField(String),
}
impl std::fmt::Display for MultipartError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Self::Decode => "couldn't decode multipart/form-data payload".to_owned(),
            Self::Internal(info) => format!(
                "an internal error occured while processing multipart request: {}",
                info
            ),
            Self::BadRequest => "bad request".to_owned(),
            Self::InvalidField(info) => format!("invalid payload structure: {}", info),
        };
        write!(f, "{}", message)
    }
}

impl std::error::Error for MultipartError {}

async fn field_to_string(field: Field) -> Result<String, MultipartError> {
    let res = field
        .collect::<Result<Bytes, actix_multipart::MultipartError>>()
        .await
        .map_err(|_| MultipartError::Decode)?;
    std::str::from_utf8(&res)
        .map(|s| s.to_owned())
        .map_err(|_| MultipartError::BadRequest)
}

pub struct SavedFile {
    pub name: String,
    pub path: String,
}

async fn field_to_file(mut field: Field, directory: &str) -> Result<SavedFile, MultipartError> {
    let filename = field
        .content_disposition()
        .map(|cd| cd.get_filename().map(|str| str.to_owned()))
        .flatten()
        .ok_or(MultipartError::BadRequest)?;
    let filename = sanitize_filename::sanitize(filename);
    let extension = Path::new(&filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap_or_else(|| "bin");

    let filepath = format!(
        "{}/{}.{}",
        directory,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("timey wimey stuff in multipart::field_to_file")
            .as_millis()
            .to_string(),
        extension
    );
    let cln = filepath.clone();

    let mut f = block(|| std::fs::File::create(cln))
        .await
        .map_err(|err| MultipartError::Internal(err.to_string()))?;

    while let Some(chunk) = field.next().await {
        let data = chunk.map_err(|_| MultipartError::Decode)?;
        f = block(move || f.write_all(&data).map(|_| f))
            .await
            .map_err(|err| MultipartError::Internal(err.to_string()))?;
    }

    Ok(SavedFile {
        name: filename,
        path: filepath,
    })
}

pub async fn to_payload<T: DeserializeOwned>(
    mut mp: Multipart,
) -> Result<(T, Vec<SavedFile>), MultipartError> {
    let mut payload: Option<String> = None;
    let mut files: Vec<SavedFile> = Vec::new();

    while let Ok(Some(field)) = mp.try_next().await {
        let disposition = field.content_disposition().ok_or(MultipartError::Decode)?;
        let field_name = disposition.get_name().ok_or(MultipartError::Decode)?;
        match field_name {
            "payload" => {
                let value = field_to_string(field).await?;
                payload = Some(value);
            }
            "file" => {
                let file = field_to_file(field, &crate::CONFIG.static_dir).await?;
                files.push(file);
            }
            _ => {}
        }
    }

    let payload = payload.ok_or(MultipartError::BadRequest)?;
    let deserialized: T = serde_json::from_str(&payload)
        .map_err(|err| MultipartError::InvalidField(err.to_string()))?;

    Ok((deserialized, files))
}
