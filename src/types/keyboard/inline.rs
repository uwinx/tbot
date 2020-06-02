//! Types representing inline keyboards.

use crate::types::{callback::Game, LoginUrl};
use is_macro::Is;
use serde::{ser::SerializeMap, Serialize};
use std::borrow::Cow;

/// A shorthand for inline markup.
pub type Markup<'a> = Cow<'a, [Cow<'a, [Button<'a>]>]>;

/// Represents different types an inline button can be.
///
/// Complete descriptions can be found in [Bots API docs][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
#[non_exhaustive]
#[must_use]
pub enum ButtonKind<'a> {
    /// Represents a URL button.
    Url(Cow<'a, str>),
    /// Represents a login button.
    LoginUrl(LoginUrl<'a>),
    /// Represents callback data.
    CallbackData(Cow<'a, str>),
    /// Represents query inserted when switched to inline.
    SwitchInlineQuery(Cow<'a, str>),
    /// Represents query inserted when switched to inline in the curent chat.
    SwitchInlineQueryCurrentChat(Cow<'a, str>),
    /// Represent a description of the game to be laucnhed.
    CallbackGame(Game),
    /// If `true`, a pay button is sent.
    Pay(bool),
}

impl<'a> ButtonKind<'a> {
    /// Create an owned `ButtonKind` from a reference without cloning.
    fn from_ref(&'a self) -> Self {
        match self {
            ButtonKind::Url(data) => ButtonKind::Url(Cow::Borrowed(data)),
            ButtonKind::LoginUrl(data) => ButtonKind::LoginUrl(data.from_ref()),
            ButtonKind::CallbackData(data) => {
                ButtonKind::CallbackData(Cow::Borrowed(data))
            }
            ButtonKind::SwitchInlineQuery(data) => {
                ButtonKind::SwitchInlineQuery(Cow::Borrowed(data))
            }
            ButtonKind::SwitchInlineQueryCurrentChat(data) => {
                ButtonKind::SwitchInlineQueryCurrentChat(Cow::Borrowed(data))
            }
            ButtonKind::CallbackGame(Game) => ButtonKind::CallbackGame(Game),
            ButtonKind::Pay(data) => ButtonKind::Pay(*data),
        }
    }

    /// Constructs a `ButtonKind` of type `Url`.
    pub fn with_url(url: impl Into<Cow<'a, str>>) -> Self {
        Self::Url(url.into())
    }

    /// Constructs a `ButtonKind` of type `LoginUrl`.
    pub fn with_login_url(login_url: impl Into<Cow<'a, str>>) -> Self {
        Self::LoginUrl(LoginUrl::new(login_url))
    }

    /// Constructs a `ButtonKind` of type `CallbackData`.
    pub fn with_callback_data(callback_data: impl Into<Cow<'a, str>>) -> Self {
        Self::CallbackData(callback_data.into())
    }

    /// Constructs a `ButtonKind` of type `SwitchInlineQuery`.
    pub fn with_switch_inline_query(
        switch_inline_query: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self::SwitchInlineQuery(switch_inline_query.into())
    }

    /// Constructs a `ButtonKind` of type `SwitchInlineQueryCurrentChat`.
    pub fn with_switch_inline_query_current_chat(
        switch_inline_query_current_chat: impl Into<Cow<'a, str>>,
    ) -> Self {
        Self::SwitchInlineQueryCurrentChat(
            switch_inline_query_current_chat.into(),
        )
    }

    /// Constructs a `ButtonKind` of type `CallbackGame`.
    pub const fn with_callback_game() -> Self {
        Self::CallbackGame(Game)
    }

    /// Constructs a `ButtonKind` of type `Pay`.
    pub const fn with_pay(pay: bool) -> Self {
        Self::Pay(pay)
    }
}

/// Represents an [`InlineKeyboardButton`].
///
/// [`InlineKeyboardButton`]: https://core.telegram.org/bots/api#inlinekeyboardbutton
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[must_use]
#[must_use]
pub struct Button<'a> {
    text: Cow<'a, str>,
    kind: ButtonKind<'a>,
}

/// Represents an [`InlineKeyboardMarkup`].
///
/// [`InlineKeyboardMarkup`]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct Keyboard<'a> {
    inline_keyboard: Markup<'a>,
}

impl<'a> Button<'a> {
    /// Constructs an inline `Button`.
    pub fn new(text: impl Into<Cow<'a, str>>, kind: ButtonKind<'a>) -> Self {
        Self {
            text: text.into(),
            kind,
        }
    }

    /// Create an owned `Button` from a reference without cloning.
    pub fn from_ref(&'a self) -> Self {
        Self {
            text: Cow::Borrowed(&self.text),
            kind: self.kind.from_ref(),
        }
    }
}

impl Serialize for Button<'_> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(Some(2))?;

        map.serialize_entry("text", &self.text)?;

        match &self.kind {
            ButtonKind::Url(url) => map.serialize_entry("url", url),
            ButtonKind::LoginUrl(login_url) => {
                map.serialize_entry("login_url", &login_url)
            }
            ButtonKind::CallbackData(callback_data) => {
                map.serialize_entry("callback_data", callback_data)
            }
            ButtonKind::SwitchInlineQuery(query) => {
                map.serialize_entry("switch_inline_query", query)
            }
            ButtonKind::SwitchInlineQueryCurrentChat(query) => {
                map.serialize_entry("switch_inline_query_current_chat", query)
            }
            ButtonKind::CallbackGame(game) => {
                map.serialize_entry("callback_game", &game)
            }
            ButtonKind::Pay(pay) => map.serialize_entry("pay", &pay),
        }?;

        map.end()
    }
}

impl<'a> Keyboard<'a> {
    /// Constructs an inline `Keyboard`.
    pub const fn new(buttons: Markup<'a>) -> Self {
        Self {
            inline_keyboard: buttons,
        }
    }
}

impl<'a> From<&'a [&'a [Button<'a>]]> for Keyboard<'a> {
    fn from(buttons: &'a [&'a [Button<'a>]]) -> Self {
        let markup: Markup<'a> = buttons
            .iter()
            .map(|button_row| button_row.iter().map(Button::from_ref).collect())
            .collect();
        Self::new(markup)
    }
}

impl<'a> From<Vec<Vec<Button<'a>>>> for Keyboard<'a> {
    fn from(buttons: Vec<Vec<Button<'a>>>) -> Self {
        Self::new(
            buttons
                .into_iter()
                .map(|x| x.into_iter().collect())
                .collect(),
        )
    }
}
