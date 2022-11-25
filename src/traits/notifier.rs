use async_trait::async_trait;
use crate::models::*;

#[async_trait]
pub trait Notifier {
    async fn notify_system(&mut self, system: &System);
    async fn notify_system_settings(&mut self, system: &str, settings: &SystemSettings);
    async fn notify_system_guild_settings(&mut self, system: &str, guild: &str, settings: &SystemGuildSettings);
    async fn notify_system_autoproxy(&mut self, system: &str, settings: &AutoproxySettings);
    async fn notify_system_members(&mut self, system: &str, members: &Vec<Member>);
    async fn notify_member(&mut self, member: &Member);
    async fn notify_member_groups(&mut self, member: &str, groups: &Vec<Group>);
    async fn notify_member_guild_settings(&mut self, member: &str, guild: &str, settings: &MemberGuildSettings);
    async fn notify_system_groups(&mut self, system: &str, groups: &Vec<Group>);
    async fn notify_group(&mut self, group: &Group);
    async fn notify_group_members(&mut self, group: &str, members: &Vec<Member>);
    async fn notify_system_switches(&mut self, system: &str, switches: &Vec<Switch>);
    async fn notify_system_active_switch(&mut self, system: &str, switch: &Switch);
    async fn notify_switch(&mut self, system: &str, switch: &Switch);
    async fn notify_message(&mut self, message: &Message);
}

#[async_trait]
impl<'a, T: Notifier + Send> Notifier for &'a mut T {
    async fn notify_system(&mut self, system: &System) {
        self.notify_system(system).await;
    }

    async fn notify_system_settings(&mut self, system: &str, settings: &SystemSettings) {
        self.notify_system_settings(system, settings).await;
    }

    async fn notify_system_guild_settings(&mut self, system: &str, guild: &str, settings: &SystemGuildSettings) {
        self.notify_system_guild_settings(system, guild, settings).await;
    }

    async fn notify_system_autoproxy(&mut self, system: &str, settings: &AutoproxySettings) {
        self.notify_system_autoproxy(system, settings).await;
    }

    async fn notify_system_members(&mut self, system: &str, members: &Vec<Member>) {
        self.notify_system_members(system, members).await;
    }

    async fn notify_member(&mut self, member: &Member) {
        self.notify_member(member).await;
    }

    async fn notify_member_groups(&mut self, member: &str, groups: &Vec<Group>) {
        self.notify_member_groups(member, groups).await;
    }

    async fn notify_member_guild_settings(&mut self, member: &str, guild: &str, settings: &MemberGuildSettings) {
        self.notify_member_guild_settings(member, guild, settings).await;
    }

    async fn notify_system_groups(&mut self, system: &str, groups: &Vec<Group>) {
        self.notify_system_groups(system, groups).await;
    }

    async fn notify_group(&mut self, group: &Group) {
        self.notify_group(group).await;
    }

    async fn notify_group_members(&mut self, group: &str, members: &Vec<Member>) {
        self.notify_group_members(group, members).await;
    }

    async fn notify_system_switches(&mut self, system: &str, switches: &Vec<Switch>) {
        self.notify_system_switches(system, switches).await;
    }

    async fn notify_system_active_switch(&mut self, system: &str, switch: &Switch) {
        self.notify_system_active_switch(system, switch).await;
    }

    async fn notify_switch(&mut self, system: &str, switch: &Switch) {
        self.notify_switch(system, switch).await;
    }

    async fn notify_message(&mut self, message: &Message) {
        self.notify_message(message).await;
    }
}