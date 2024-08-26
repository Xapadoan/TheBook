use std::{error::Error, fmt::Display, fs, io, path::PathBuf};

use shared::{auth::Session, player::PlayerBuildError};

const SESSION_STORAGE_DIR: &'static str = "local";
const SESSION_STORAGE_PATH: &'static str = "local/session.txt";

pub fn store_session(session: &Session) -> Result<(), SessionError> {
    let path = PathBuf::from(SESSION_STORAGE_PATH);
    if !path.try_exists()? {
        fs::create_dir_all(SESSION_STORAGE_DIR)?;
    }
    let serialized_session = serde_json::to_string(session)?;
    fs::write(&path, serialized_session)?;
    Ok(())
}

pub fn read_session() -> Result<Option<Session>, SessionError> {
    let path = PathBuf::from(SESSION_STORAGE_PATH);
    if !path.try_exists()? {
        Ok(None)
    } else {
        let session_file = fs::File::open(path)?;
        let buf = io::BufReader::new(session_file);
        let session: Session = serde_json::from_reader(buf)?;
        Ok(Some(session))
    }
}

#[derive(Debug)]
pub struct SessionError {
    message: String,
}

impl SessionError {
    fn new(message: String) -> Self {
        Self { message }
    }
}

impl Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for SessionError {}

impl From<serde_json::Error> for SessionError {
    fn from(value: serde_json::Error) -> Self {
        Self::new(format!("Serialization Error:\n{value}"))
    }
}

impl From<io::Error> for SessionError {
    fn from(value: io::Error) -> Self {
        Self::new(format!("io Error:\n{value}"))
    }
}

impl From<SessionError> for PlayerBuildError {
    fn from(value: SessionError) -> Self {
        eprintln!("[WARN] Should remove impl From<SessionError> for PlayerBuildError");
        Self::new(format!("Session Error:\n{value}"))
    }
}
