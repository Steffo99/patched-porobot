pub enum LoadingError {
    Checking,
    Loading(std::io::Error),
    Parsing(serde_json::Error),
    Using,
}

pub type LoadingResult<T> = Result<T, LoadingError>;
