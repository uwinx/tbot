use super::send_method;
use crate::{
    connectors::Connector,
    errors,
    internal::Client,
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
    },
    token,
};
use serde::Serialize;

/// Sets a group's global permissions.
///
/// Reflects the [`setChatPermissions`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatpermissions
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatPermissions<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: token::Ref<'a>,
    chat_id: ChatId<'a>,
    permissions: chat::Permissions,
}

impl<'a, C> SetChatPermissions<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: token::Ref<'a>,
        chat_id: impl ImplicitChatId<'a>,
        permissions: chat::Permissions,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            permissions,
        }
    }
}

impl<C: Connector> SetChatPermissions<'_, C> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        send_method::<bool, _>(
            self.client,
            self.token,
            "setChatPermissions",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
