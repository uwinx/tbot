use super::*;

/// Represents the origin of the callback.
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub enum CallbackOrigin {
    /// The callback comes from this message.
    Message(Box<Message>),
    /// The callback comes from an inline message with this ID.
    Inline(String),
}

/// Represents the kind of the callback.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
// todo: #[non_exhaustive]
pub enum CallbackKind {
    /// The callback is sent with some data.
    Data(String),
    /// The callback is sent to open a game.
    Game(String),
}

/// Represents a [`CallbackQuery`].
///
/// [`CallbackQuery`]: https://core.telegram.org/bots/api#callbackquery
#[derive(Debug, PartialEq, Clone)]
// todo: #[non_exhaustive]
pub struct CallbackQuery {
    /// The ID of the callback.
    pub id: String,
    /// The user who initiated the callback.
    pub from: User,
    /// The origin of the query.
    pub origin: CallbackOrigin,
    /// The identifier of the chat.
    pub chat_instance: String,
    /// The kind of the callback.
    pub kind: CallbackKind,
}

const ID: &str = "id";
const FROM: &str = "from";
const MESSAGE: &str = "message";
const INLINE_MESSAGE_ID: &str = "inline_message_id";
const CHAT_INSTANCE: &str = "chat_instance";
const DATA: &str = "data";
const GAME_SHORT_NAME: &str = "game_short_name";

struct CallbackQueryVisitor;

impl<'v> serde::de::Visitor<'v> for CallbackQueryVisitor {
    type Value = CallbackQuery;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "struct CallbackQuery")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
    where
        V: serde::de::MapAccess<'v>,
    {
        let mut id = None;
        let mut from = None;
        let mut message = None;
        let mut inline_message_id = None;
        let mut chat_instance = None;
        let mut data = None;
        let mut game_short_name = None;

        while let Some(key) = map.next_key()? {
            match key {
                ID => id = Some(map.next_value()?),
                FROM => from = Some(map.next_value()?),
                MESSAGE => message = Some(map.next_value()?),
                INLINE_MESSAGE_ID => {
                    inline_message_id = Some(map.next_value()?)
                }
                CHAT_INSTANCE => chat_instance = Some(map.next_value()?),
                DATA => data = Some(map.next_value()?),
                GAME_SHORT_NAME => game_short_name = Some(map.next_value()?),
                _ => {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
            }
        }

        let origin = if let Some(message) = message {
            CallbackOrigin::Message(message)
        } else if let Some(inline_message_id) = inline_message_id {
            CallbackOrigin::Inline(inline_message_id)
        } else {
            return Err(serde::de::Error::custom("Neither `message` nor `inline_message_id` was present on `CallbackQuery`"));
        };

        let kind = if let Some(data) = data {
            CallbackKind::Data(data)
        } else if let Some(game_short_name) = game_short_name {
            CallbackKind::Game(game_short_name)
        } else {
            return Err(serde::de::Error::custom("Neither `callback_data` nor `game_short_name` was present on `CallbackQuery`"));
        };

        Ok(CallbackQuery {
            id: id.ok_or_else(|| serde::de::Error::missing_field(ID))?,
            from: from.ok_or_else(|| serde::de::Error::missing_field(FROM))?,
            origin,
            chat_instance: chat_instance.ok_or_else(|| {
                serde::de::Error::missing_field(CHAT_INSTANCE)
            })?,
            kind,
        })
    }
}

impl<'de> serde::Deserialize<'de> for CallbackQuery {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_struct(
            "CallbackQuery",
            &[
                ID,
                FROM,
                MESSAGE,
                INLINE_MESSAGE_ID,
                CHAT_INSTANCE,
                DATA,
                GAME_SHORT_NAME,
            ],
            CallbackQueryVisitor,
        )
    }
}
