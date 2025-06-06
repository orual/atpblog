// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `com.atproto.server.createSession` namespace.
pub const NSID: &str = "com.atproto.server.createSession";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///When true, instead of throwing error for takendown accounts, a valid response with a narrow scoped token will be returned
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub allow_takendown: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub auth_factor_token: core::option::Option<String>,
    ///Handle or other identifier supported by the server for the authenticating user.
    pub identifier: String,
    pub password: String,
}
pub type Input = atrium_api::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub access_jwt: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub active: core::option::Option<bool>,
    pub did: atrium_api::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub did_doc: core::option::Option<atrium_api::types::Unknown>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub email: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub email_auth_factor: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub email_confirmed: core::option::Option<bool>,
    pub handle: atrium_api::types::string::Handle,
    pub refresh_jwt: String,
    ///If active=false, this optional field indicates a possible reason for why the account is not active. If active=false and no status is supplied, then the host makes no claim for why the repository is no longer being hosted.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub status: core::option::Option<String>,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    AccountTakedown(Option<String>),
    AuthFactorTokenRequired(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::AccountTakedown(msg) => {
                write!(_f, "AccountTakedown")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::AuthFactorTokenRequired(msg) => {
                write!(_f, "AuthFactorTokenRequired")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
