use super::*;

/// Representation of the [`editMessageLiveLocation`] method for the case if
/// the message was sent via the inline mode.
///
/// [`editMessageLiveLocation`]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize)]
pub struct EditInlineLocation<'a> {
    #[serde(skip)]
    token: &'a str,
    inline_message_id: u64,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::InlineKeyboard<'a>>,
}

impl<'a> EditInlineLocation<'a> {
    /// Constructs a new `EditInlineLocation`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        inline_message_id: u64,
        (latitude, longitude): (f64, f64),
    ) -> EditInlineLocation<'a> {
        EditInlineLocation {
            token,
            inline_message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Sets `reply_markup` to `Some(markup)`.
    #[must_use]
    pub fn reply_markup(
        mut self,
        markup: types::InlineKeyboard<'a>,
    ) -> Self {
        self.reply_markup = Some(markup);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = bool, Error = DeliveryError> {
        send_method::<bool>(
            self.token,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}