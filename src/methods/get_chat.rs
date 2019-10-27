use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    token,
    types::{
        parameters::{ChatId, ImplicitChatId},
        Chat,
    },
};
use serde::Serialize;

/// Gets information about a chat.
///
/// Reflects the [`getChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchat
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChat<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
}

impl<'a, C> GetChat<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
        }
    }
}

impl<C: Connector> GetChat<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<Chat, errors::MethodCall> {
        send_method(
            self.client,
            self.token,
            "getChat",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
