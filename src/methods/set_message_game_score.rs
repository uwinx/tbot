use super::call_method;
use crate::{
    connectors::Client,
    errors, token,
    types::{
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};
use serde::Serialize;

/// Sets a user's new high score in a game sent by the bot itself.
///
/// Reflects the [`setGameScore`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setgamescore
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMessageGameScore<'a> {
    #[serde(skip)]
    client: &'a Client,
    #[serde(skip)]
    token: token::Ref<'a>,
    user_id: user::Id,
    score: u32,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_edit_message: Option<bool>,
}

impl<'a> SetMessageGameScore<'a> {
    pub(crate) fn new(
        client: &'a Client,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        user_id: user::Id,
        score: u32,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            score,
            chat_id: chat_id.into(),
            message_id,
            force: None,
            disable_edit_message: None,
        }
    }

    /// Configures if the score may go down. Reflects the `force` parameter.
    pub fn force(mut self, is_forced: bool) -> Self {
        self.force = Some(is_forced);
        self
    }

    /// Configures if the message should not be edited immediately.
    /// Reflects the `disable_edit_message` parameter.
    pub fn disable_edit_message(mut self, is_disabled: bool) -> Self {
        self.disable_edit_message = Some(is_disabled);
        self
    }
}

impl SetMessageGameScore<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.client,
            self.token,
            "setGameScore",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
