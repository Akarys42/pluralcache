use std::sync::Arc;
use async_trait::async_trait;
use tokio::sync::Mutex;
use crate::models::{AutoproxySettings, Group, Member, MemberGuildSettings, Message, Switch, System, SystemGuildSettings, SystemSettings};
use crate::traits::notifier::Notifier;
use crate::traits::provider::{Provider, ProviderResult};

trait ProviderAndNotifier: Provider + Notifier {}

pub(crate) struct Controller {
    providers: Vec<Arc<Mutex<dyn Provider + Send + Sync>>>,
    notifiers: Vec<Arc<Mutex<dyn Notifier + Send + Sync>>>,
}

impl Controller {
    pub fn new() -> Self {
        Self {
            providers: Vec::new(),
            notifiers: Vec::new(),
        }
    }

    pub fn add_provider(&mut self, provider: Arc<Mutex<dyn Provider + Send + Sync>>) {
        self.providers.push(provider);
    }

    pub fn add_notifier(&mut self, notifier: Arc<Mutex<dyn Notifier + Send + Sync>>) {
        self.notifiers.push(notifier);
    }
}

#[async_trait]
impl Provider for Controller {
    async fn get_system(&mut self, id: &str) -> ProviderResult<System> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system(id).await;
            }

            if let ProviderResult::Ok(system) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system(&system).await;
                }

                return ProviderResult::Ok(system);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_settings(&mut self, id: &str) -> ProviderResult<SystemSettings> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_settings(id).await;
            }

            if let ProviderResult::Ok(system_settings) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_settings(&id, &system_settings).await;
                }

                return ProviderResult::Ok(system_settings);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<SystemGuildSettings> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_guild_settings(id, guild).await;
            }

            if let ProviderResult::Ok(system_guild_settings) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_guild_settings(&id, &guild, &system_guild_settings).await;
                }

                return ProviderResult::Ok(system_guild_settings);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_autoproxy(&mut self, id: &str) -> ProviderResult<AutoproxySettings> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_autoproxy(id).await;
            }

            if let ProviderResult::Ok(autoproxy_settings) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_autoproxy(&id, &autoproxy_settings).await;
                }

                return ProviderResult::Ok(autoproxy_settings);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_members(id).await;
            }

            if let ProviderResult::Ok(members) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_members(&id, &members).await;
                }

                return ProviderResult::Ok(members);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_member(&mut self, id: &str) -> ProviderResult<Member> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_member(id).await;
            }

            if let ProviderResult::Ok(member) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_member(&member).await;
                }

                return ProviderResult::Ok(member);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_member_groups(&mut self, id: &str) -> ProviderResult<Vec<Group>> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_member_groups(id).await;
            }

            if let ProviderResult::Ok(groups) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_member_groups(&id, &groups).await;
                }

                return ProviderResult::Ok(groups);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_member_guild_settings(&mut self, id: &str, guild: &str) -> ProviderResult<MemberGuildSettings> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_member_guild_settings(id, guild).await;
            }

            if let ProviderResult::Ok(member_guild_settings) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_member_guild_settings(&id, &guild, &member_guild_settings).await;
                }

                return ProviderResult::Ok(member_guild_settings);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_groups(&mut self, id: &str, with_member: bool) -> ProviderResult<Vec<Group>> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_groups(id, with_member).await;
            }

            if let ProviderResult::Ok(groups) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_groups(&id, &groups).await;
                }

                return ProviderResult::Ok(groups);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_group(&mut self, id: &str) -> ProviderResult<Group> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_group(id).await;
            }

            if let ProviderResult::Ok(group) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_group(&group).await;
                }

                return ProviderResult::Ok(group);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_group_members(&mut self, id: &str) -> ProviderResult<Vec<Member>> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_group_members(id).await;
            }

            if let ProviderResult::Ok(members) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_group_members(&id, &members).await;
                }

                return ProviderResult::Ok(members);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_switches(&mut self, id: &str, before: &str, limit: u64) -> ProviderResult<Vec<Switch>> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_switches(id, before, limit).await;
            }

            if let ProviderResult::Ok(switches) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_switches(&id, &switches).await;
                }

                return ProviderResult::Ok(switches);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_system_active_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_system_active_switch(id, switch_id).await;
            }

            if let ProviderResult::Ok(switch) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_system_active_switch(&id, &switch).await;
                }

                return ProviderResult::Ok(switch);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_switch(&mut self, id: &str, switch_id: &str) -> ProviderResult<Switch> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_switch(id, switch_id).await;
            }

            if let ProviderResult::Ok(switch) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_switch(&id, &switch).await;
                }

                return ProviderResult::Ok(switch);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }

    async fn get_message(&mut self, id: &str) -> ProviderResult<Message> {
        for provider in &mut self.providers {
            let result;
            {
                let mut provider = provider.lock().await;
                result = provider.get_message(id).await;
            }

            if let ProviderResult::Ok(message) = result {
                for notifier in &mut self.notifiers {
                    let mut notifier = notifier.lock().await;
                    notifier.notify_message(&message).await;
                }

                return ProviderResult::Ok(message);
            }

            if [ProviderResult::NotFound, ProviderResult::NotImplemented].contains(&result) {
                return result;
            }

        }
        ProviderResult::Failed
    }
}