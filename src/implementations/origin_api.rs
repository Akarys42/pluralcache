use std::time::Duration;
use async_trait::async_trait;
use serde::Deserialize;
use crate::models::{AutoproxySettings, Group, Member, MemberGuildSettings, Message, Switch, System, SystemGuildSettings, SystemSettings};
use crate::traits::provider::{Provider, ProviderResult};

pub(crate) struct OriginApi {
    client: reqwest::Client,
    base_url: String,
}

impl OriginApi {
    async fn get<T: for<'de> Deserialize<'de>>(&mut self, path: String) -> ProviderResult<T> {
        match self.client.get(&format!("{}/{}", self.base_url, path)).send().await {
            Ok(response) => {
                if (500..599).contains(&response.status().as_u16()) || response.status().as_u16() == 400 {
                    return ProviderResult::Failed
                }
                else if response.status() == 404 {
                    return ProviderResult::NotFound
                }
                else if [401, 403].contains(&response.status().as_u16()) {
                    return ProviderResult::Unauthorized
                }

                match response.json::<T>().await {
                    Ok(json) => ProviderResult::Ok(json),
                    Err(_) => ProviderResult::Failed,
                }
            },
            Err(_) => ProviderResult::Failed,
        }
    }

    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("pluralcache")
            .gzip(true)
            .brotli(true)
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();

        Self {
            client,
            base_url,
        }
    }
}

#[async_trait]
impl Provider for OriginApi {
    async fn get_system(&mut self, id: &str) -> ProviderResult<System> {
        self.get(format!("/systems/{}", id)).await
    }

    async fn get_system_settings(&mut self, id: &str) -> ProviderResult<SystemSettings> {
        self.get(format!("/systems/{}/settings", id)).await
    }

    async fn get_system_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<SystemGuildSettings> {
        self.get(format!("/systems/{}/guilds/{}/settings", id, guild)).await
    }

    async fn get_system_autoproxy(&mut self, id: &str) -> ProviderResult<AutoproxySettings> {
        self.get(format!("/systems/{}/autoproxy", id)).await
    }

    async fn get_system_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        self.get(format!("/systems/{}/members", id)).await
    }

    async fn get_member(&mut self, id: &str) -> ProviderResult<Member> {
        self.get(format!("/members/{}", id)).await
    }

    async fn get_member_groups(&mut self, id: &str) -> ProviderResult<Vec<Group>> {
        self.get(format!("/members/{}/groups", id)).await
    }

    async fn get_member_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<MemberGuildSettings> {
        self.get(format!("/members/{}/guilds/{}/settings", id, guild)).await
    }

    async fn get_system_groups(&mut self, id: &str, with_member: bool) -> ProviderResult<Vec<Group>> {
        self.get(format!("/systems/{}/groups?with_member={}", id, with_member)).await
    }

    async fn get_group(&mut self, id: &str) -> ProviderResult<Group> {
        self.get(format!("/groups/{}", id)).await
    }

    async fn get_group_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        self.get(format!("/groups/{}/members", id)).await
    }

    async fn get_system_switches(&mut self, id: &str, before: &str, limit: u64) -> ProviderResult<Vec<Switch>> {
        self.get(format!("/systems/{}/switches?before={}&limit={}", id, before, limit)).await
    }

    async fn get_system_active_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        self.get(format!("/systems/{}/switches/{}/active", id, switch_id)).await
    }

    async fn get_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        self.get(format!("/systems/{}/switches/{}", id, switch_id)).await
    }

    async fn get_message(&mut self, id: &str) -> ProviderResult<Message> {
        self.get(format!("/messages/{}", id)).await
    }
}