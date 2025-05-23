// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `com.atproto.repo.createRecord` namespace.
pub const NSID: &str = "com.atproto.repo.createRecord";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///The NSID of the record collection.
    pub collection: atrium_api::types::string::Nsid,
    ///The record itself. Must contain a $type field.
    pub record: atrium_api::types::Unknown,
    ///The handle or DID of the repo (aka, current account).
    pub repo: atrium_api::types::string::AtIdentifier,
    ///The Record Key.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub rkey: core::option::Option<atrium_api::types::string::RecordKey>,
    ///Compare and swap with the previous commit by CID.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub swap_commit: core::option::Option<atrium_api::types::string::Cid>,
    ///Can be set to 'false' to skip Lexicon schema validation of record data, 'true' to require it, or leave unset to validate only for known Lexicons.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub validate: core::option::Option<bool>,
}
pub type Input = atrium_api::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub cid: atrium_api::types::string::Cid,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub commit: core::option::Option<crate::com::atproto::repo::defs::CommitMeta>,
    pub uri: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub validation_status: core::option::Option<String>,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    ///Indicates that 'swapCommit' didn't match current repo commit.
    InvalidSwap(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidSwap(msg) => {
                write!(_f, "InvalidSwap")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
