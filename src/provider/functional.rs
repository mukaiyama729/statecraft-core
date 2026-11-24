use crate::{
    provider::base::{ProviderBase, ProviderId, ProviderMeta},
    reference::provider_ref::Ref,
};

pub type ProviderFactory<T> = for<'a> fn(&mut Ref<'a>) -> T;

pub struct Provider<T: 'static> {
    meta: ProviderMeta,
    factory: ProviderFactory<T>,
}

impl<T: 'static> Provider<T> {
    pub const fn new(name: &'static str, factory: ProviderFactory<T>) -> Self {
        Self {
            meta: ProviderMeta::new(name),
            factory,
        }
    }

    pub const fn auto_dispose(mut self) -> Self {
        self.meta = self.meta.with_auto_dispose(true);
        self
    }

    pub(crate) fn create(&self, ref_: &mut Ref<'_>) -> T {
        (self.factory)(ref_)
    }

    pub fn id(&'static self) -> ProviderId {
        self.meta.id()
    }
}

impl<T: 'static> ProviderBase for Provider<T> {
    type State = T;

    fn meta(&'static self) -> &'static ProviderMeta {
        &self.meta
    }
}
