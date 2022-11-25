use crate::models::*;
use async_trait::async_trait;

#[derive(Debug, Clone, PartialEq)]
pub enum ProviderResult<T> {
    Ok(T),
    NotFound,
    Unauthorized,
    Failed,
    NotImplemented,
}

#[async_trait]
pub trait Provider {
    async fn get_system(&mut self, id: &str) -> ProviderResult<System>;
    async fn get_system_settings(&mut self, id: &str) -> ProviderResult<SystemSettings>;
    async fn get_system_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<SystemGuildSettings>;
    async fn get_system_autoproxy(&mut self, id: &str) -> ProviderResult<AutoproxySettings>;
    async fn get_system_members(&mut self, id: &str) -> ProviderResult<Vec<Member>>;
    async fn get_member(&mut self, id: &str) -> ProviderResult<Member>;
    async fn get_member_groups(&mut self, id: &str) -> ProviderResult<Vec<Group>>;
    async fn get_member_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<MemberGuildSettings>;
    async fn get_system_groups(&mut self, id: &str, with_member: bool) -> ProviderResult<Vec<Group>>;
    async fn get_group(&mut self, id: &str) -> ProviderResult<Group>;
    async fn get_group_members(&mut self, id: &str) -> ProviderResult<Vec<Member>>;
    async fn get_system_switches(&mut self, id: &str, before: &str, limit: u64) -> ProviderResult<Vec<Switch>>;
    async fn get_system_active_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch>;
    async fn get_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch>;
    async fn get_message(&mut self, id: &str) -> ProviderResult<Message>;
}

#[async_trait]
impl<T: Provider + Send + Sync> Provider for &'static mut T {
    async fn get_system(&mut self, id: &str) -> ProviderResult<System> {
        self.get_system(id).await
    }

    async fn get_system_settings(&mut self, id: &str) -> ProviderResult<SystemSettings> {
        self.get_system_settings(id).await
    }

    async fn get_system_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<SystemGuildSettings> {
        self.get_system_guild_settings(id, guild).await
    }

    async fn get_system_autoproxy(&mut self, id: &str) -> ProviderResult<AutoproxySettings> {
        self.get_system_autoproxy(id).await
    }

    async fn get_system_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        self.get_system_members(id).await
    }

    async fn get_member(&mut self, id: &str) -> ProviderResult<Member> {
        self.get_member(id).await
    }

    async fn get_member_groups(&mut self, id: &str) -> ProviderResult<Vec<Group>> {
        self.get_member_groups(id).await
    }

    async fn get_member_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<MemberGuildSettings> {
        self.get_member_guild_settings(id, guild).await
    }

    async fn get_system_groups(&mut self, id: &str, with_member: bool) -> ProviderResult<Vec<Group>> {
        self.get_system_groups(id, with_member).await
    }

    async fn get_group(&mut self, id: &str) -> ProviderResult<Group> {
        self.get_group(id).await
    }

    async fn get_group_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        self.get_group_members(id).await
    }

    async fn get_system_switches(&mut self, id: &str, before: &str, limit: u64) -> ProviderResult<Vec<Switch>> {
        self.get_system_switches(id, before, limit).await
    }

    async fn get_system_active_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        self.get_system_active_switch(id, switch_id).await
    }

    async fn get_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        self.get_switch(id, switch_id).await
    }

    async fn get_message(&mut self, id: &str) -> ProviderResult<Message> {
        self.get_message(id).await
    }
}