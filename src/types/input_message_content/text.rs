use crate::types::{
    parameters::{self, ParseMode},
    InteriorBorrow,
};
use serde::Serialize;
use std::borrow::Cow;

/// Represents an [`InputTextMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Text<'a> {
    message_text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl<'a> Text<'a> {
    /// Constructs a new `Text`.
    pub fn new(message_text: impl Into<parameters::Text<'a>>) -> Self {
        let message_text = message_text.into();

        Self {
            message_text: message_text.text,
            parse_mode: message_text.parse_mode,
            disable_web_page_preview: None,
        }
    }

    /// Configures whether the web page preview is disabled.
    pub const fn is_web_page_preview_disabled(
        mut self,
        is_disabled: bool,
    ) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }
}

impl<'a> InteriorBorrow<'a> for Text<'a> {
    fn borrow_inside(&'a self) -> Self {
        Self {
            message_text: self.message_text.borrow_inside(),
            ..*self
        }
    }
}
