use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        keyboard, message,
        parameters::{ChatId, ImplicitChatId},
    },
};

/// Represents the [`stopPoll`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#stoppoll
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct StopPoll<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a, C> StopPoll<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            reply_markup: None,
        }
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for StopPoll<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Poll;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "stopPoll",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
