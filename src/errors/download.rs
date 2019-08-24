use hyper::StatusCode;

/// Represents possible errors whic may occur while downloading a file.
#[derive(Debug)]
pub enum Download {
    /// The provided file had the `path` field set to `None`.
    NoPath,
    /// A network error.
    Network(hyper::Error),
    /// Telegram returned a different from 200 status code.
    InvalidStatusCode(StatusCode),
}

impl Download {
    /// Checks if `self` is `NoPath`.
    pub fn is_no_path(&self) -> bool {
        match self {
            Download::NoPath => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Network`.
    pub fn is_network(&self) -> bool {
        match self {
            Download::Network(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `InvalidStatusCode`.
    pub fn is_invalid_status_code(&self) -> bool {
        match self {
            Download::InvalidStatusCode(..) => true,
            _ => false,
        }
    }
}

impl From<hyper::Error> for Download {
    fn from(error: hyper::Error) -> Self {
        Download::Network(error)
    }
}

impl From<StatusCode> for Download {
    fn from(error: StatusCode) -> Self {
        Download::InvalidStatusCode(error)
    }
}
