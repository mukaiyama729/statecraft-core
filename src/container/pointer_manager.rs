use alloc::collections::BTreeMap;

use crate::{
    container::pointer::ProviderPointer,
    element::element::ProviderElement,
    provider::{base::ProviderId, functional::Provider},
};

pub(crate) struct ProviderPointerManager {
    pointers: BTreeMap<ProviderId, ProviderPointer>,
}

impl ProviderPointerManager {
    pub(crate) const fn new() -> Self {
        Self {
            pointers: BTreeMap::new(),
        }
    }

    pub(crate) fn get(&self, id: ProviderId) -> Option<&ProviderPointer> {
        self.pointers.get(&id)
    }

    pub(crate) fn get_mut(&mut self, id: ProviderId) -> Option<&mut ProviderPointer> {
        self.pointers.get_mut(&id)
    }

    // ここでstaticなProviderインスタンスを以下のように保存
    // provider_id: ProviderPointer
    // ProviderPointerはProviderElementをヒープ上におくための入れ物
    pub(crate) fn upsert<T: PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
    ) -> &mut ProviderPointer {
        self.pointers.entry(provider.id()).or_insert_with(|| {
            let element = ProviderElement::new(provider);

            ProviderPointer::new(element)
        })
    }

    pub(crate) fn contains(&self, id: ProviderId) -> bool {
        self.pointers.contains_key(&id)
    }

    pub(crate) fn remove(&mut self, id: ProviderId) -> Option<ProviderPointer> {
        self.pointers.remove(&id)
    }

    pub(crate) fn clear(&mut self) {
        self.pointers.clear();
    }

    pub(crate) fn len(&self) -> usize {
        self.pointers.len()
    }
}
