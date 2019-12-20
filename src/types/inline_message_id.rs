//! Types representing an inline message ID.

use serde::{Deserialize, Serialize};

/// Represents an inline message ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
#[must_use]
pub struct InlineMessageId(pub String);

/// Contains a reference to an inline message ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
#[must_use]
pub struct Ref<'a>(pub &'a str);

impl InlineMessageId {
    /// Constructs an inline message ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(&self.0)
    }
}

impl<'a> Ref<'a> {
    /// Constructs an inline message [`Id`] based on `self`.
    ///
    /// [`Id`]: ./struct.Id.html
    pub fn to_owned(&self) -> InlineMessageId {
        InlineMessageId(self.0.into())
    }
}

impl From<String> for InlineMessageId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a str> for Ref<'a> {
    fn from(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> PartialEq<Ref<'a>> for InlineMessageId {
    #[must_use]
    fn eq(&self, other: &Ref<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialEq<InlineMessageId> for Ref<'a> {
    #[must_use]
    fn eq(&self, other: &InlineMessageId) -> bool {
        self.0 == other.0
    }
}
