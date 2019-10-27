use super::ChatMethods;
use crate::methods::PinChatMessage;

/// Provides methods for pinnable messages.
pub trait Pinnable<'a, C: 'static>: ChatMethods<'a, C> {
    /// Pins this message.
    fn pin_this_message(&'a self) -> PinChatMessage<'a, C> {
        self.bot()
            .pin_chat_message(self.chat().id, self.message_id())
    }
}
