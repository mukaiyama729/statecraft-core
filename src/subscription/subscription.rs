use alloc::rc::{Rc, Weak};

use crate::provider::base::ProviderId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubscriptionId(u64);

impl SubscriptionId {
    pub(crate) const fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug)]
pub struct Subscription {
    id: SubscriptionId,
    provider_id: ProviderId,
    owner: Weak<()>,
}

impl Subscription {
    pub(crate) fn new(id: SubscriptionId, provider_id: ProviderId, owner: Weak<()>) -> Self {
        Self {
            id,
            provider_id,
            owner,
        }
    }

    pub fn id(&self) -> SubscriptionId {
        self.id
    }

    pub fn provider_id(&self) -> ProviderId {
        self.provider_id
    }

    pub(crate) fn belongs_to(&self, owner: &Rc<()>) -> bool {
        self.owner
            .upgrade()
            .map_or(false, |token| Rc::ptr_eq(&token, owner))
    }
}
