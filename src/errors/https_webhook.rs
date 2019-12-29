use super::MethodCall;
use is_macro::Is;
use std::{
    error::Error,
    fmt::{self, Display, Formatter},
};
use tokio::time::Elapsed;

/// Represents possible errors that an HTTPS webhook server may return.
#[derive(Debug, Is)]
pub enum HttpsWebhook {
    /// An error during setting the webhook.
    SetWebhook(MethodCall),
    /// Calling the `setWebhook` method timed out.
    SetWebhookTimeout(Elapsed),
    /// An error during initializing TLS.
    Tls(
        #[cfg(feature = "tls")] native_tls::Error,
        #[cfg(feature = "rustls")] tokio_rustls::rustls::TLSError,
    ),
    /// An error during port binding.
    Bind(std::io::Error),
    /// An error while running the server.
    Server(hyper::Error),
}

impl Display for HttpsWebhook {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        match self {
            Self::SetWebhook(error) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 failed with an error: {}",
                error,
            ),
            Self::SetWebhookTimeout(timeout) => write!(
                formatter,
                "The webhook event loop failed because a call to `setWebhook` \
                 timed out: {}",
                timeout,
            ),
            Self::Tls(timeout) => write!(
                formatter,
                "The webhook event loop failed because TLS initialization \
                 failed with an error: {}",
                timeout,
            ),
            Self::Bind(timeout) => write!(
                formatter,
                "The webhook event loop failed because failed to bind to a \
                 port: {}",
                timeout,
            ),
            Self::Server(error) => write!(
                formatter,
                "The webhook event loop failed because the server returned \
                 with an error: {}",
                error,
            ),
        }
    }
}

impl Error for HttpsWebhook {}

impl From<MethodCall> for HttpsWebhook {
    #[must_use]
    fn from(error: MethodCall) -> Self {
        Self::SetWebhook(error)
    }
}

impl From<Elapsed> for HttpsWebhook {
    #[must_use]
    fn from(error: Elapsed) -> Self {
        Self::SetWebhookTimeout(error)
    }
}

#[cfg(feature = "tls")]
impl From<native_tls::Error> for HttpsWebhook {
    #[must_use]
    fn from(error: native_tls::Error) -> Self {
        Self::Tls(error)
    }
}

#[cfg(feature = "rustls")]
impl From<tokio_rustls::rustls::TLSError> for HttpsWebhook {
    #[must_use]
    fn from(error: tokio_rustls::rustls::TLSError) -> Self {
        Self::Tls(error)
    }
}

impl From<std::io::Error> for HttpsWebhook {
    #[must_use]
    fn from(error: std::io::Error) -> Self {
        Self::Bind(error)
    }
}

impl From<hyper::Error> for HttpsWebhook {
    #[must_use]
    fn from(error: hyper::Error) -> Self {
        Self::Server(error)
    }
}
