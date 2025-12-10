#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("user error: {0}")]
    User(String),
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<serde_json::Error> for CliError {
    fn from(value: serde_json::Error) -> Self {
        CliError::Internal(format!("serialization failed with: {value}"))
    }
}

impl CliError {
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::User(_) => 8,
            CliError::Internal(_) => 128,
        }
    }
}
