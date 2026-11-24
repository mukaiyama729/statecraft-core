use crate::{
    provider::{base::ProviderId, functional::Provider},
    reference::{build_context::BuildContext, provider_ref::Ref},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementStatus {
    Uninitialized,
    Clean,
    Dirty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateKind {
    Initialized,
    Changed,
    Unchanged,
}

pub struct Update<T> {
    pub kind: UpdateKind,
    pub previous: Option<T>,
}

// 状態管理を担う。
// Providerはあくまで状態を提供するもの
pub struct ProviderElement<T: 'static> {
    provider: &'static Provider<T>,
    value: Option<T>,
    dirty: bool,
    should_notify: fn(&T, &T) -> bool,
}

impl<T: PartialEq + 'static> ProviderElement<T> {
    pub fn new(provider: &'static Provider<T>) -> Self {
        Self::with_comparator(provider, |previous, next| previous != next)
    }
}

impl<T: 'static> ProviderElement<T> {
    pub fn with_comparator(
        provider: &'static Provider<T>,
        should_notify: fn(&T, &T) -> bool,
    ) -> Self {
        Self {
            provider,
            value: None,
            dirty: false,
            should_notify,
        }
    }

    pub fn provider_id(&self) -> ProviderId {
        self.provider.id()
    }

    pub fn status(&self) -> ElementStatus {
        if self.value.is_none() {
            ElementStatus::Uninitialized
        } else if self.dirty {
            ElementStatus::Dirty
        } else {
            ElementStatus::Clean
        }
    }

    pub fn read_cached(&self) -> Option<&T> {
        self.value.as_ref()
    }

    pub(crate) fn set_value(&mut self, next: T) -> Update<T> {
        let kind = match self.value.as_ref() {
            None => UpdateKind::Initialized,

            Some(previous) => {
                if (self.should_notify)(previous, &next) {
                    UpdateKind::Changed
                } else {
                    UpdateKind::Unchanged
                }
            }
        };

        let previous = self.value.replace(next);

        self.dirty = false;

        Update { kind, previous }
    }

    pub fn invalidate(&mut self) {
        self.dirty = true;
    }

    pub(crate) fn prepare_build(&self) -> Option<&'static Provider<T>> {
        match self.status() {
            ElementStatus::Clean => None,

            ElementStatus::Uninitialized | ElementStatus::Dirty => Some(self.provider),
        }
    }
}
