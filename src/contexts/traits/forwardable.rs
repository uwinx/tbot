use super::*;
use crate::types::parameters::ImplicitChatId;

/// Provides methods for forwardable messages.
pub trait Forwardable<'a, C: 'static>: ChatMethods<'a, C> {
    /// Forwards this message to another chat.
    fn forward_to(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> ForwardMessage<'a, C> {
        self.bot().forward_message(chat_id, self.chat_id(), self.message_id())
    }
}
