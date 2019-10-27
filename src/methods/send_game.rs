use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, NotificationState},
    },
    token,
};
use serde::Serialize;

/// Sends a game.
///
/// Reflects the [`sendGame`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendgame
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendGame<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    game_short_name: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> SendGame<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        game_short_name: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            game_short_name,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures which message this game is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C: Connector> SendGame<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "sendGame",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
