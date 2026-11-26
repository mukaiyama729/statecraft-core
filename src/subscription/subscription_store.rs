use core::{any::Any, task::ready};

use alloc::{boxed::Box, collections::BTreeMap, rc::Rc};

use crate::{
    container::provider_container::ContainerError,
    provider::{base::ProviderId, functional::Provider},
    subscription::{
        listener::{ErasedListener, Listener},
        subscription::{Subscription, SubscriptionId},
    },
};

type ListenerMap = BTreeMap<SubscriptionId, Box<dyn ErasedListener>>;

pub(crate) struct SubscriptionStore {
    next_id: u64,
    owner: Option<Rc<()>>,
    listeners: BTreeMap<ProviderId, ListenerMap>,
}

impl SubscriptionStore {
    pub(crate) const fn new() -> Self {
        Self {
            next_id: 0u64,
            owner: None, // 初回registerの段階でownerである()Union型が作られる
            listeners: BTreeMap::new(),
        }
    }

    pub fn register<T: 'static, F>(
        &mut self,
        provider: &'static Provider<T>,
        mut callback: F,
        initial: Option<&T>,
    ) -> Result<Subscription, ContainerError>
    where
        T: 'static,
        F: FnMut(Option<&T>, &T) + 'static,
    {
        let id = SubscriptionId::new(self.next_id);
        self.next_id = self
            .next_id
            .checked_add(1)
            .ok_or(ContainerError::SubscriptionIdExhausted)?;

        if let Some(value) = initial {
            callback(None, value);
        }

        let provider_id = provider.id();
        let listener = Listener::<T, F>::new(callback);
        self.listeners
            .entry(provider_id)
            .or_default()
            .insert(id, Box::new(listener));
        let owner = self.owner.get_or_insert_with(|| Rc::new(()));
        let subscription = Subscription::new(id, provider_id, Rc::downgrade(owner));
        Ok(subscription)
    }

    pub fn notify<T: 'static>(
        &mut self,
        provider_id: ProviderId,
        previous: Option<&T>,
        next: &T,
    ) -> Result<(), ContainerError> {
        let Some(listeners) = self.listeners.get_mut(&provider_id) else {
            return Ok(());
        };
        let previous = previous.map(|v| v as &dyn Any);
        let next = next as &dyn Any;
        for listen in listeners.values_mut() {
            listen.notify(previous, next)?;
        }
        Ok(())
    }
    pub(crate) fn remove(&mut self, subscription: &Subscription) -> bool {
        let Some(owner) = self.owner.as_ref() else {
            return false;
        };

        if !subscription.belongs_to(owner) {
            return false;
        }

        let provider_id = subscription.provider_id();

        let (removed, empty) = {
            let Some(listeners) = self.listeners.get_mut(&provider_id) else {
                return false;
            };

            let removed = listeners.remove(&subscription.id()).is_some();

            (removed, listeners.is_empty())
        };

        if empty {
            self.listeners.remove(&provider_id);
        }

        removed
    }

    pub(crate) fn remove_provider(&mut self, provider_id: ProviderId) {
        self.listeners.remove(&provider_id);
    }

    pub(crate) fn listener_count(&self, provider_id: ProviderId) -> usize {
        self.listeners
            .get(&provider_id)
            .map_or(0, |listeners| listeners.len())
    }

    pub(crate) fn clear(&mut self) {
        self.listeners.clear();
    }
}
