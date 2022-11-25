use std::collections::HashMap;
use async_trait::async_trait;
use crate::models::{AutoproxySettings, Group, Member, MemberGuildSettings, Message, Switch, System, SystemGuildSettings, SystemSettings};
use crate::traits::provider::{Provider, ProviderResult};
use crate::traits::notifier::Notifier;

pub(crate) struct InMemoryCache {
    systems: HashMap<String, System>,
    system_settings: HashMap<String, SystemSettings>,
    system_guild_settings: HashMap<(String, String), SystemGuildSettings>,
    system_autoproxy: HashMap<String, AutoproxySettings>,
    system_members: HashMap<String, Vec<Member>>,
    members: HashMap<String, Member>,
    member_groups: HashMap<String, Vec<Group>>,
    member_guild_settings: HashMap<(String, String), MemberGuildSettings>,
    system_groups: HashMap<String, Vec<Group>>,
    groups: HashMap<String, Group>,
    group_members: HashMap<String, Vec<Member>>,
    /* system_switches: HashMap<String, Vec<Switch>>, */
    system_active_switch: HashMap<(String, String), Switch>,
    switches: HashMap<(String, String), Switch>,
    messages: HashMap<String, Message>,
}

impl InMemoryCache {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            system_settings: HashMap::new(),
            system_guild_settings: HashMap::new(),
            system_autoproxy: HashMap::new(),
            system_members: HashMap::new(),
            members: HashMap::new(),
            member_groups: HashMap::new(),
            member_guild_settings: HashMap::new(),
            system_groups: HashMap::new(),
            groups: HashMap::new(),
            group_members: HashMap::new(),
            /* system_switches: HashMap::new(), */
            system_active_switch: HashMap::new(),
            switches: HashMap::new(),
            messages: HashMap::new(),
        }
    }
}

#[async_trait]
impl Notifier for InMemoryCache {
    async fn notify_system(&mut self, system: &System) {
        self.systems.insert(system.id.clone(), system.clone());
    }

    async fn notify_system_settings(&mut self, system: &str, settings: &SystemSettings) {
        self.system_settings.insert(system.to_string(), settings.clone());
    }

    async fn notify_system_guild_settings(&mut self, system: &str, guild: &str, settings: &SystemGuildSettings) {
        self.system_guild_settings.insert((system.to_string(), guild.to_string()), settings.clone());
    }

    async fn notify_system_autoproxy(&mut self, system: &str, settings: &AutoproxySettings) {
        self.system_autoproxy.insert(system.to_string(), settings.clone());
    }

    async fn notify_system_members(&mut self, system: &str, members: &Vec<Member>) {
        self.system_members.insert(system.to_string(), members.clone());

        for member in members {
            self.members.insert(member.id.clone(), member.clone());
        }
    }

    async fn notify_member(&mut self, member: &Member) {
        self.members.insert(member.id.clone(), member.clone());
    }

    async fn notify_member_groups(&mut self, member: &str, groups: &Vec<Group>) {
        self.member_groups.insert(member.to_string(), groups.clone());

        for group in groups {
            self.groups.insert(group.id.clone(), group.clone());
        }
    }

    async fn notify_member_guild_settings(&mut self, member: &str, guild: &str, settings: &MemberGuildSettings) {
        self.member_guild_settings.insert((member.to_string(), guild.to_string()), settings.clone());
    }

    async fn notify_system_groups(&mut self, system: &str, groups: &Vec<Group>) {
        self.system_groups.insert(system.to_string(), groups.clone());

        for group in groups {
            self.groups.insert(group.id.clone(), group.clone());
        }
    }

    async fn notify_group(&mut self, group: &Group) {
        self.groups.insert(group.id.clone(), group.clone());
    }

    async fn notify_group_members(&mut self, group: &str, members: &Vec<Member>) {
        self.group_members.insert(group.to_string(), members.clone());

        for member in members {
            self.members.insert(member.id.clone(), member.clone());
        }
    }

    async fn notify_system_switches(&mut self, system: &str, switches: &Vec<Switch>) {
        // TODO: Build a good switch history awareness

        for switch in switches {
            self.switches.insert((system.to_string(), switch.id.clone()), switch.clone());
        }
    }

    async fn notify_system_active_switch(&mut self, system: &str, switch: &Switch) {
        self.system_active_switch.insert((system.to_string(), switch.id.clone()), switch.clone());
    }

    async fn notify_switch(&mut self, system: &str, switch: &Switch) {
        self.switches.insert((system.to_string(), switch.id.clone()), switch.clone());
    }

    async fn notify_message(&mut self, message: &Message) {
        self.messages.insert(message.id.clone(), message.clone());
    }
}

#[async_trait]
impl Provider for InMemoryCache {
    async fn get_system(&mut self, id: &str) -> ProviderResult<System> {
        if let Some(system) = self.systems.get(id) {
            ProviderResult::Ok(system.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_settings(&mut self, id: &str) -> ProviderResult<SystemSettings> {
        if let Some(settings) = self.system_settings.get(id) {
            ProviderResult::Ok(settings.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<SystemGuildSettings> {
        if let Some(settings) = self.system_guild_settings.get(&(id.to_string(), guild.to_string())) {
            ProviderResult::Ok(settings.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_autoproxy(&mut self, id: &str) -> ProviderResult<AutoproxySettings> {
        if let Some(settings) = self.system_autoproxy.get(id) {
            ProviderResult::Ok(settings.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        if let Some(members) = self.system_members.get(id) {
            ProviderResult::Ok(members.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_member(&mut self, id: &str) -> ProviderResult<Member> {
        if let Some(member) = self.members.get(id) {
            ProviderResult::Ok(member.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_member_groups(&mut self, id: &str) -> ProviderResult<Vec<Group>> {
        if let Some(groups) = self.member_groups.get(id) {
            ProviderResult::Ok(groups.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_member_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<MemberGuildSettings> {
        if let Some(settings) = self.member_guild_settings.get(&(id.to_string(), guild.to_string())) {
            ProviderResult::Ok(settings.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_groups(&mut self, id: &str, with_member: bool) -> ProviderResult<Vec<Group>> {
        if let Some(groups) = self.system_groups.get(id) {
            if with_member {
                // Check that we have member information for each group
                if !groups.into_iter().all(|g| g.members != None) {
                    return ProviderResult::Failed;
                }
            }

            let mut new_groups = groups.clone();

            if !with_member {
                for group in new_groups.iter_mut() {
                    group.members = None;
                }
            }

            ProviderResult::Ok(new_groups)
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_group(&mut self, id: &str) -> ProviderResult<Group> {
        if let Some(group) = self.groups.get(id) {
            ProviderResult::Ok(group.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_group_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        if let Some(members) = self.group_members.get(id) {
            ProviderResult::Ok(members.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_system_switches(&mut self, _id: &str, _before: &str, _limit: u64) -> ProviderResult<Vec<Switch>> {
        ProviderResult::NotImplemented
    }

    async fn get_system_active_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        if let Some(switch) = self.system_active_switch.get(&(id.to_string(), switch_id.to_string())) {
            ProviderResult::Ok(switch.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        if let Some(switch) = self.switches.get(&(id.to_string(), switch_id.to_string())) {
            ProviderResult::Ok(switch.clone())
        } else {
            ProviderResult::Failed
        }
    }

    async fn get_message(&mut self, id: &str) -> ProviderResult<Message> {
        if let Some(message) = self.messages.get(id) {
            ProviderResult::Ok(message.clone())
        } else {
            ProviderResult::Failed
        }
    }
}