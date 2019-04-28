use super::*;
use std::sync::Arc;

/// Context for the [`text`] handler.
///
/// [`text`]: ../struct.Bot.html#method.text
#[derive(Clone)]
pub struct TextContext {
    private: (),
    /// A mock bot with all API methods.
    pub bot: Arc<MockBot>,
    /// Id of the message.
    pub message_id: u32,
    /// The sender of the message.
    pub from: Option<types::User>,
    /// The time the message was sent at.
    pub date: i64,
    /// The chat where the message was sent.
    pub chat: types::raw::Chat,
    /// The origin of the message if it is a forward.
    pub forward: Option<types::Forward>,
    /// If `Some`, the original message.
    pub reply_to: Option<types::Message>,
    /// The message's text.
    pub text: String,
    /// Entities in the message (links, formatting, etc).
    pub entities: Vec<types::MessageEntity>,
}

impl TextContext {
    // https://github.com/rust-lang/rust-clippy/issues/4041
    #[allow(clippy::missing_const_for_fn)]
    pub(crate) fn new(
        bot: Arc<MockBot>,
        message_id: u32,
        from: Option<types::User>,
        date: i64,
        chat: types::raw::Chat,
        forward: Option<types::Forward>,
        reply_to: Option<types::Message>,
        text: types::Text,
    ) -> Self {
        Self {
            private: (),
            bot,
            message_id,
            from,
            date,
            chat,
            forward,
            reply_to,
            text: text.text,
            entities: text.entities,
        }
    }
}

impl<'a> traits::ChatMethods<'a> for TextContext {
    fn bot(&self) -> &MockBot {
        &self.bot
    }

    fn chat_id(&self) -> i64 {
        self.chat.id
    }

    fn message_id(&self) -> u32 {
        self.message_id
    }
}

impl<'a> Forwardable<'a> for TextContext {}
