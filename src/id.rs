// src/id.rs
//! Core ID type and associated operations

use std::{fmt, str::FromStr};

use rand::Rng;

use crate::{
    alphabet::{self, CHECK_ALPHABET},
    error::IdError,
};

/// A user-friendly identifier with check bit validation
///
/// # Example
/// ```no_run
/// use human_friendly_ids::Id;
/// use std::str::FromStr;
///
/// let id = Id::from_str("abc-").unwrap();
/// assert_eq!(id.as_str(), "abc-");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Id(pub(crate) String);

impl Id {
    /// Get string slice representation
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Calculate maximum valid ID length for current configuration
    #[allow(
        clippy::arithmetic_side_effects,
        clippy::cast_possible_truncation,
        reason = "assert macro"
    )]
    #[must_use]
    pub const fn max_length() -> usize {
        const _: () = assert!(
            CHECK_ALPHABET.len() > 2,
            "CHECK_ALPHABET length must be greater than 2"
        );
        const _: () = assert!(std::mem::size_of::<usize>() == 8,);
        let max_value = u64::MAX / (CHECK_ALPHABET.len() - 1) as u64;
        (max_value + 1) as usize
    }

    /// Generate a new ID with a given length
    ///
    /// See: [`Id::new`] if you want to use the default RNG.
    ///
    #[allow(
        clippy::missing_panics_doc,
        reason = "Internal invariant - won't generate a string that would panic."
    )]
    #[must_use]
    pub fn new_with_rng<R: Rng>(len: usize, rng: &mut R) -> Self {
        let mut body = String::with_capacity(len.saturating_sub(1));
        let mut last_char = None;

        while body.len() < len.saturating_sub(1) {
            let idx = rng.random_range(0..alphabet::GEN_ALPHABET.len());
            #[allow(clippy::indexing_slicing, reason = "index is generated within bounds")]
            let c = alphabet::GEN_ALPHABET[idx];
            // Avoid ambiguous sequences
            match (last_char, c) {
                (Some('r'), 'n') | (Some('v'), 'v') => {}
                // Don't end with 'r' or 'v', because the check-bit could create an ambiguous sequence
                (_, 'r' | 'v') if body.len() == len.saturating_sub(2) => {}
                _ => {
                    body.push(c);
                    last_char = Some(c);
                }
            }
        }

        let check_char = alphabet::calculate_check_char(&body)
            .expect("Generated body should be valid for check calculation");

        Id(format!("{}{}", body, check_char))
    }

    /// Generate a new ID with a given length
    ///
    /// This method uses the default RNG from the `rand` crate.
    #[must_use]
    pub fn new(len: usize) -> Self {
        let mut rng = rand::rng();
        Self::new_with_rng(len, &mut rng)
    }
}

#[cfg_attr(test, mutants::skip)]
impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg_attr(test, mutants::skip)]
impl std::ops::Deref for Id {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

#[cfg_attr(test, mutants::skip)]
impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0
    }
}

#[cfg_attr(test, mutants::skip)]
impl From<Id> for Box<str> {
    fn from(id: Id) -> Self {
        id.0.into_boxed_str()
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = alphabet::normalize_string(s);

        if normalized.len() <= 3 {
            return Err(IdError::TooShort);
        }

        let (body, check_char) = normalized
            .split_at_checked(normalized.len().checked_sub(1).expect("checked above"))
            .ok_or(IdError::InvalidCharacter)?;
        let expected_check = alphabet::calculate_check_char(body)?;

        if check_char != expected_check.to_string() {
            return Err(IdError::InvalidCheckBit);
        }

        for c in body.chars() {
            alphabet::validate_char(c)?;
        }

        Ok(Self(normalized))
    }
}

impl TryFrom<String> for Id {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(&value)
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(feature = "serde")]
/// This module provides custom implementations for the `Serialize` and `Deserialize` traits
/// for the `UploadId` type. These implementations allow `UploadId` to be serialized as a string
/// and deserialized from a string using Serde.
///
/// # Examples
///
/// ```
/// use serde::{Serialize, Deserialize};
/// use human_friendly_ids::Id;
///
/// #[derive(Serialize, Deserialize)]
/// struct MyStruct {
///     id: Id,
/// }
/// ```
mod serde_impl {
    use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error};

    use super::Id;

    impl Serialize for Id {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(self.as_str())
        }
    }

    impl<'de> Deserialize<'de> for Id {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            s.parse().map_err(D::Error::custom)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_serde_roundtrip() {
            let id = Id::try_from("wcfytxww4opin4jmjjes4ccfd".to_string())
                .expect("Failed to decode UploadId");
            let serialized = serde_json::to_string(&id).expect("Failed to serialize UploadId");

            insta::assert_json_snapshot!(serialized);

            let deserialized: Id =
                serde_json::from_str(&serialized).expect("Failed to deserialize UploadId");
            assert_eq!(id, deserialized);

            insta::assert_debug_snapshot!(deserialized);
        }
    }
}
