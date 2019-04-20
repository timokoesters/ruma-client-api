//! Helper module for the Serialize / Deserialize impl's for the User struct
//! in the parent module.

use serde::{Deserialize, Serialize};

use super::{DeprecatedMedium, Medium};

// The following three structs could just be used in place of the one in the parent module, but
// that one is arguably much easier to deal with.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum User<'a> {
    DeprecatedUserIdentifier(DeprecatedUserIdentifier<'a>),
    UserIdentifier {
        #[serde(borrow)]
        identifier: UserIdentifier<'a>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum UserIdentifier<'a> {
    #[serde(rename = "m.id.user")]
    MatrixId { user: &'a str },
    #[serde(rename = "m.id.thirdparty")]
    ThirdPartyId { medium: Medium, address: &'a str },
    #[serde(rename = "m.id.phone")]
    PhoneNumber { country: &'a str, phone: &'a str },
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum DeprecatedUserIdentifier<'a> {
    MatrixId {
        user: &'a str,
    },
    ThirdPartyId {
        address: &'a str,
        medium: DeprecatedMedium,
    },
}

impl<'a> From<&'a super::User> for User<'a> {
    fn from(su: &'a super::User) -> Self {
        use super::User::*;

        match su {
            MatrixId(user) => User::UserIdentifier {
                identifier: UserIdentifier::MatrixId { user },
            },
            DeprecatedMatrixId(user) => {
                User::DeprecatedUserIdentifier(DeprecatedUserIdentifier::MatrixId { user })
            }
            ThirdPartyId { address, medium } => User::UserIdentifier {
                identifier: UserIdentifier::ThirdPartyId {
                    address,
                    medium: *medium,
                },
            },
            DeprecatedThirdPartyId { address, medium } => {
                User::DeprecatedUserIdentifier(DeprecatedUserIdentifier::ThirdPartyId {
                    address,
                    medium: *medium,
                })
            }
            PhoneNumber { country, phone } => User::UserIdentifier {
                identifier: UserIdentifier::PhoneNumber { country, phone },
            },
        }
    }
}

impl Into<super::User> for User<'_> {
    fn into(self) -> super::User {
        use super::User::*;

        match self {
            User::UserIdentifier {
                identifier: UserIdentifier::MatrixId { user },
            } => MatrixId(user.to_owned()),
            User::UserIdentifier {
                identifier: UserIdentifier::ThirdPartyId { address, medium },
            } => ThirdPartyId {
                address: address.to_owned(),
                medium: medium.to_owned(),
            },
            User::UserIdentifier {
                identifier: UserIdentifier::PhoneNumber { country, phone },
            } => PhoneNumber {
                country: country.to_owned(),
                phone: phone.to_owned(),
            },
            User::DeprecatedUserIdentifier(DeprecatedUserIdentifier::MatrixId { user }) => {
                DeprecatedMatrixId(user.to_owned())
            }
            User::DeprecatedUserIdentifier(DeprecatedUserIdentifier::ThirdPartyId {
                address,
                medium,
            }) => DeprecatedThirdPartyId {
                address: address.to_owned(),
                medium: medium.to_owned(),
            },
        }
    }
}
