use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The authenticated principal received from the trusted ingress layer.
///
/// This value is sourced exclusively from the `X-Authenticated-Subject` header,
/// which is set by ingress-nginx from the oauth2-proxy auth response. It
/// corresponds to the JWT `sub` claim, which is the Kratos Identity ID.
///
/// The Account Service never processes the original JWT. It operates solely on
/// this extracted, pre-validated principal.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthenticatedPrincipal {
    /// The OAuth `sub` claim value, equal to the Kratos Identity ID.
    pub subject: String,
}

/// Account domain entity.
///
/// Represents the account information for an authenticated identity, sourced
/// from the Ory Kratos Admin API. Contains only fields defined in the DiTA
/// identity schema.
///
/// This entity belongs strictly to the account domain and must not carry
/// user-domain or institution-domain concepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Kratos Identity ID (equal to the OAuth `sub` claim).
    pub id: String,

    /// Primary account email address.
    pub email: String,

    /// First name from identity traits, if present.
    pub first_name: Option<String>,

    /// Last name from identity traits, if present.
    pub last_name: Option<String>,

    /// Username from identity traits, if present.
    pub username: Option<String>,

    /// Avatar URL from identity traits, if present.
    pub avatar_url: Option<String>,

    /// Kratos identity state (e.g. "active").
    pub state: Option<String>,
}

/// Kratos identity traits as defined in the DiTA identity schema.
#[derive(Debug, Deserialize)]
pub struct KratosTraits {
    pub email: String,
    pub name: Option<KratosName>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
}

/// Name sub-object from Kratos identity traits.
#[derive(Debug, Deserialize)]
pub struct KratosName {
    pub first: Option<String>,
    pub last: Option<String>,
}

/// Kratos Admin API identity response.
#[derive(Debug, Deserialize)]
pub struct KratosIdentity {
    pub id: String,
    pub state: Option<String>,
    pub traits: Value,
}

impl KratosIdentity {
    /// Maps a Kratos identity response into the `Account` domain entity.
    pub fn into_account(self) -> Account {
        let traits: Option<KratosTraits> = serde_json::from_value(self.traits.clone()).ok();

        let (email, first_name, last_name, username, avatar_url) = match traits {
            Some(t) => {
                let (first_name, last_name) = match t.name {
                    Some(n) => (n.first, n.last),
                    None => (None, None),
                };
                (t.email, first_name, last_name, t.username, t.avatar_url)
            }
            None => (String::new(), None, None, None, None),
        };

        Account {
            id: self.id,
            email,
            first_name,
            last_name,
            username,
            avatar_url,
            state: self.state,
        }
    }
}
