//! [POST /_matrix/client/r0/login](https://matrix.org/docs/spec/client_server/r0.4.0.html#post-matrix-client-r0-login)

use ruma_api_macros::ruma_api;
use ruma_identifiers::UserId;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

ruma_api! {
    metadata {
        description: "Login to the homeserver.",
        method: POST,
        name: "login",
        path: "/_matrix/client/r0/login",
        rate_limited: true,
        requires_authentication: false,
    }

    request {
        /// Identification information for the user.
        #[serde(flatten)]
        pub user: User,
        /// The authentication mechanism.
        #[serde(flatten)]
        pub login_type: LoginType,
        /// ID of the client device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub device_id: Option<String>,
        /// A display name to assign to the newly-created device. Ignored if device_id corresponds
        /// to a known device.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initial_device_display_name: Option<String>,
    }

    response {
        /// An access token for the account.
        pub access_token: String,
        /// The hostname of the homeserver on which the account has been registered.
        ///
        /// Deprecated: Clients should extract the server_name from user_id (by splitting at the
        /// first colon) if they require it.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub home_server: Option<String>,
        /// The fully-qualified Matrix ID that has been registered.
        pub user_id: UserId,
        /// ID of the logged-in device.
        ///
        /// Will be the same as the corresponging parameter in the request, if one was
        /// specified.
        pub device_id: String,
    }
}

/// The authentication mechanism.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum LoginType {
    /// A password is supplied to authenticate.
    #[serde(rename = "m.login.password")]
    Password {
        /// The password.
        password: String,
    },
    /// Token-based login.
    #[serde(rename = "m.login.token")]
    Token {
        /// The token.
        token: String,
    },
}

/// Identification information for the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum User {
    /// Either a fully qualified Matrix user ID, or just the localpart (as part of the 'identifier'
    /// field, recommended since r0.4.0).
    MatrixId(String),
    /// Either a fully qualified Matrix user ID, or just the localpart
    /// (as 'user' field, deprecated since r0.4.0)
    DeprecatedMatrixId(String),
    /// Third party identifier (as part of the 'identifier' field, recommended since r0.4.0).
    ThirdPartyId {
        /// Third party identifier for the user.
        address: String,
        /// The medium of the identifier.
        medium: Medium,
    },
    /// Third party identifier (as separate fields, deprecated since r0.4.0)
    DeprecatedThirdPartyId {
        /// Third party identifier for the user.
        address: String,
        /// The medium of the identifier.
        medium: DeprecatedMedium,
    },
    /// Same as third-party identification with medium == msisdn, but with a non-canonicalised
    /// phone number.
    PhoneNumber {
        /// The country that the phone number is from.
        country: String,
        /// The phone number.
        phone: String,
    },
}

mod user_serde;

impl Serialize for User {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        user_serde::User::from(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for User {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        user_serde::User::deserialize(deserializer).map(Into::into)
    }
}

// https://matrix.org/docs/spec/appendices.html#pid-types
// TODO: Move to ruma-identifiers (?)
/// The medium of a third party identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Medium {
    /// An email address.
    Email,
    /// A phone number represented as a Mobile Station International Subscriber Directory Number
    /// (as defined by the E.164 numbering plan)
    Msisdn,
}

/// The medium of a third party identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeprecatedMedium {
    /// An email address.
    Email,
}

#[cfg(test)]
mod tests {
    use serde_json;

    use super::{LoginType, User};

    #[test]
    fn deserialize_login_type() {
        assert_eq!(
            serde_json::from_str::<LoginType>(
                r#"
                {
                    "type": "m.login.password",
                    "password": "ilovebananas"
                }
                "#,
            )
            .unwrap(),
            LoginType::Password {
                password: "ilovebananas".into()
            }
        );

        assert_eq!(
            serde_json::from_str::<LoginType>(
                r#"
                {
                    "type": "m.login.token",
                    "token": "1234567890abcdef"
                }
                "#,
            )
            .unwrap(),
            LoginType::Token {
                token: "1234567890abcdef".into()
            }
        );
    }

    #[test]
    fn deserialize_user() {
        assert_eq!(
            serde_json::from_str::<User>(
                r#"
                {
                    "identifier": {
                        "type": "m.id.user",
                        "user": "cheeky_monkey"
                    }
                }
                "#,
            )
            .unwrap(),
            User::MatrixId("cheeky_monkey".into())
        );

        assert_eq!(
            serde_json::from_str::<User>(
                r#"
                {
                    "user": "cheeky_monkey"
                }
                "#,
            )
            .unwrap(),
            User::DeprecatedMatrixId("cheeky_monkey".into())
        );
    }
}
