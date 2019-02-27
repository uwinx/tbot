use super::*;

// This is a false positive as it's used in `into_future`'s signature
#[allow(dead_code)]
type Photos = Vec<Vec<types::UserProfilePhotos>>;

/// Represents the [`getUserProfilePhotos`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getuserprofilephotos
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetUserProfilePhotos<'a> {
    #[serde(skip)]
    token: &'a str,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl<'a> GetUserProfilePhotos<'a> {
    /// Constructs a new `GetUserProfilePhotos`.
    pub fn new<'b: 'a>(token: &'b str, user_id: i64) -> Self {
        Self {
            token,
            user_id,
            offset: None,
            limit: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Configures `offset`.
    pub fn offset(mut self, offset: u64) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Configures `limit`.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use = "futures do nothing unless polled"]
    pub fn into_future(
        self,
    ) -> impl Future<Item = Photos, Error = DeliveryError> {
        send_method(
            self.token,
            "getUserProfilePhotos",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        )
    }
}

#[cfg(feature = "proxy")]
impl<'a> ProxyMethod for GetUserProfilePhotos<'a> {
    /// Configures `proxy`.
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
