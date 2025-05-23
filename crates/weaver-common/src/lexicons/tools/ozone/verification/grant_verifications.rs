// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `tools.ozone.verification.grantVerifications` namespace.
pub const NSID: &str = "tools.ozone.verification.grantVerifications";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///Array of verification requests to process
    pub verifications: Vec<VerificationInput>,
}
pub type Input = atrium_api::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub failed_verifications: Vec<GrantError>,
    pub verifications: Vec<crate::tools::ozone::verification::defs::VerificationView>,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
///Error object for failed verifications.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GrantErrorData {
    ///Error message describing the reason for failure.
    pub error: String,
    ///The did of the subject being verified
    pub subject: atrium_api::types::string::Did,
}
pub type GrantError = atrium_api::types::Object<GrantErrorData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VerificationInputData {
    ///Timestamp for verification record. Defaults to current time when not specified.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub created_at: core::option::Option<String>,
    ///Display name of the subject the verification applies to at the moment of verifying.
    pub display_name: String,
    ///Handle of the subject the verification applies to at the moment of verifying.
    pub handle: atrium_api::types::string::Handle,
    ///The did of the subject being verified
    pub subject: atrium_api::types::string::Did,
}
pub type VerificationInput = atrium_api::types::Object<VerificationInputData>;
