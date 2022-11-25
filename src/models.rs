use serde::{Deserialize, Serialize};
use serde;

/* Models */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct System {
    pub id: String,
    pub uuid: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub created: Option<String>,
    pub privacy: Option<SystemPrivacy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Member {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub display_name: Option<String>,
    pub color: Option<String>,
    pub birthday: Option<String>,
    pub pronouns: Option<String>,
    pub avatar_url: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub created: Option<String>,
    pub proxy_tags: Vec<ProxyTag>,
    pub keep_proxy: bool,
    pub privacy: Option<MemberPrivacy>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Group {
    pub id: String,
    pub uuid: String,
    pub name: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub color: Option<String>,
    pub privacy: Option<GroupPrivacy>,
    pub members: Option<Vec<Member>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct ProxyTag {
    prefix: Option<String>,
    suffix: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Switch {
    pub id: String,
    pub timestamp: String,
    pub members: Vec<MemberOrId>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Message {
    pub timestamp: String,
    pub id: String,
    pub original: String,
    pub sender: String,
    pub channel: String,
    pub guild: String,
    pub system: Option<System>,
    pub member: Option<Member>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SystemSettings {
    pub timezone: String,
    pub pings_enabled: bool,
    pub latch_timeout: Option<i64>,
    pub member_default_privacy: bool,
    pub group_default_privacy: bool,
    pub show_private_info: bool,
    pub member_limit: i64,
    pub group_limit: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SystemGuildSettings {
    pub guild_id: Option<String>,
    pub proxying_enabled: bool,
    pub tag: Option<String>,
    pub tag_enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AutoproxySettings {
    pub autoproxy_mode: AutoproxyMode,
    pub autoproxy_member: Option<String>,
    pub last_latch_timestamp: Option<String>,
}

/* Privacy */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SystemPrivacy {
    pub description_privacy: Option<PrivacyKey>,
    pub pronoun_privacy: Option<PrivacyKey>,
    pub member_list_privacy: Option<PrivacyKey>,
    pub group_list_privacy: Option<PrivacyKey>,
    pub front_privacy: Option<PrivacyKey>,
    pub front_history_privacy: Option<PrivacyKey>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemberPrivacy {
    pub visibility: Option<PrivacyKey>,
    pub name_privacy: Option<PrivacyKey>,
    pub description_privacy: Option<PrivacyKey>,
    pub birthday_privacy: Option<PrivacyKey>,
    pub pronoun_privacy: Option<PrivacyKey>,
    pub avatar_privacy: Option<PrivacyKey>,
    pub metadata_privacy: Option<PrivacyKey>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct GroupPrivacy {
    pub name_privacy: Option<PrivacyKey>,
    pub description_privacy: Option<PrivacyKey>,
    pub icon_privacy: Option<PrivacyKey>,
    pub list_privacy: Option<PrivacyKey>,
    pub metadata_privacy: Option<PrivacyKey>,
    pub visibility: Option<PrivacyKey>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MemberGuildSettings {
    pub guild_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

/* Enums */

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AutoproxyMode {
    OFF,
    FRONT,
    LATCH,
    MEMBER,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrivacyKey {
    PRIVATE,
    PUBLIC,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MemberOrId {
    Member(Member),
    Id(String),
}