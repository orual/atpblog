// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.unspecced.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonSearchActorData {
    pub did: atrium_api::types::string::Did,
}
pub type SkeletonSearchActor = atrium_api::types::Object<SkeletonSearchActorData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonSearchPostData {
    pub uri: String,
}
pub type SkeletonSearchPost = atrium_api::types::Object<SkeletonSearchPostData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonSearchStarterPackData {
    pub uri: String,
}
pub type SkeletonSearchStarterPack = atrium_api::types::Object<
    SkeletonSearchStarterPackData,
>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SkeletonTrendData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub category: core::option::Option<String>,
    pub dids: Vec<atrium_api::types::string::Did>,
    pub display_name: String,
    pub link: String,
    pub post_count: i64,
    pub started_at: atrium_api::types::string::Datetime,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub status: core::option::Option<String>,
    pub topic: String,
}
pub type SkeletonTrend = atrium_api::types::Object<SkeletonTrendData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TrendViewData {
    pub actors: Vec<crate::app::bsky::actor::defs::ProfileViewBasic>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub category: core::option::Option<String>,
    pub display_name: String,
    pub link: String,
    pub post_count: i64,
    pub started_at: atrium_api::types::string::Datetime,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub status: core::option::Option<String>,
    pub topic: String,
}
pub type TrendView = atrium_api::types::Object<TrendViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TrendingTopicData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub description: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub display_name: core::option::Option<String>,
    pub link: String,
    pub topic: String,
}
pub type TrendingTopic = atrium_api::types::Object<TrendingTopicData>;
