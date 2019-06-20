//! Types representing an inline query ID.

use serde::{Deserialize, Serialize};

/// Represents an inline query ID.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
#[serde(transparent)]
pub struct Id(pub String);

/// Contains a reference to an inline query ID.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[serde(transparent)]
pub struct Ref<'a>(pub &'a str);

impl Id {
    /// Constructs an inline query ID [`Ref`] based on `self`.
    ///
    /// [`IdRef`]: ./struct.Ref.html
    pub fn as_ref(&self) -> Ref<'_> {
        Ref(&self.0)
    }
}

impl<'a> Ref<'a> {
    /// Constructs an inline query [`Id`] based on `self`.
    ///
    /// [`Id`]: ./struct.Id.html
    pub fn to_owned(&self) -> Id {
        Id(self.0.into())
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl<'a> From<&'a str> for Ref<'a> {
    fn from(id: &'a str) -> Self {
        Self(id)
    }
}

impl<'a> PartialEq<Ref<'a>> for Id {
    fn eq(&self, other: &Ref<'a>) -> bool {
        self.0 == other.0
    }
}

impl<'a> PartialEq<Id> for Ref<'a> {
    fn eq(&self, other: &Id) -> bool {
        self.0 == other.0
    }
}