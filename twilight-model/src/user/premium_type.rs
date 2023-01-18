use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter, Result as FmtResult};

/// Type of premium tier for a [`User`].
///
/// [`User`]: super::User
#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PremiumType(u8);

impl PremiumType {
    /// User doesn't have premium.
    pub const NONE: Self = Self::new(0);

    /// User has Nitro Classic.
    pub const NITRO_CLASSIC: Self = Self::new(1);

    /// User has the standard Nitro.
    pub const NITRO: Self = Self::new(2);

    /// User has Nitro Basic.
    pub const NITRO_BASIC: Self = Self::new(3);

    /// Create a new premium type from a dynamic value.
    ///
    /// The provided value isn't validated. Known valid values are associated
    /// constants such as [`NITRO`][`Self::NITRO`].
    pub const fn new(premium_type: u8) -> Self {
        Self(premium_type)
    }

    /// Retrieve the value of the premium type.
    ///
    /// # Examples
    ///
    /// ```
    /// use twilight_model::user::PremiumType;
    ///
    /// assert_eq!(2, PremiumType::NITRO.get());
    /// ```
    pub const fn get(&self) -> u8 {
        self.0
    }

    /// Name of the associated constant.
    ///
    /// Returns `None` if the value doesn't have a defined constant.
    pub const fn name(self) -> Option<&'static str> {
        Some(match self {
            Self::NITRO_BASIC => "NITRO_BASIC",
            Self::NITRO_CLASSIC => "NITRO_CLASSIC",
            Self::NITRO => "NITRO",
            Self::NONE => "NONE",
            _ => return None,
        })
    }
}

impl Debug for PremiumType {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        if let Some(name) = self.name() {
            f.debug_struct("PremiumType")
                .field("name", &name)
                .field("value", &self.0)
                .finish()
        } else {
            f.debug_tuple("PremiumType").field(&self.0).finish()
        }
    }
}

impl From<u8> for PremiumType {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<PremiumType> for u8 {
    fn from(value: PremiumType) -> Self {
        value.get()
    }
}

#[cfg(test)]
mod tests {
    use super::PremiumType;
    use serde::{Deserialize, Serialize};
    use serde_test::Token;
    use static_assertions::assert_impl_all;
    use std::{fmt::Debug, hash::Hash};

    assert_impl_all!(
        PremiumType: Clone,
        Copy,
        Debug,
        Deserialize<'static>,
        Eq,
        Hash,
        PartialEq,
        Send,
        Serialize,
        Sync
    );

    const MAP: &[(PremiumType, u8)] = &[
        (PremiumType::NONE, 0),
        (PremiumType::NITRO_CLASSIC, 1),
        (PremiumType::NITRO, 2),
        (PremiumType::NITRO_BASIC, 3),
    ];

    #[test]
    fn variants() {
        for (kind, num) in MAP {
            serde_test::assert_tokens(
                kind,
                &[
                    Token::NewtypeStruct {
                        name: "PremiumType",
                    },
                    Token::U8(*num),
                ],
            );
            assert_eq!(*kind, PremiumType::from(*num));
            assert_eq!(*num, kind.get());
        }
    }
}
