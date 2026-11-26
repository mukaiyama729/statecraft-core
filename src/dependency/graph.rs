use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec::Vec,
};

use crate::provider::base::ProviderId;

pub(crate) struct DependencyGraph {
    // keyが依存するvalue
    dependencies: BTreeMap<ProviderId, BTreeSet<ProviderId>>,

    // keyに依存するvalue
    dependents: BTreeMap<ProviderId, BTreeSet<ProviderId>>,
}

impl DependencyGraph {
    pub(crate) const fn new() -> Self {
        Self {
            dependencies: BTreeMap::new(),
            dependents: BTreeMap::new(),
        }
    }

    // idが依存するprovider_idたちの取得
    pub(crate) fn dependencies_of(&self, id: ProviderId) -> Vec<ProviderId> {
        self.dependencies
            .get(&id)
            .map(|value| value.iter().cloned().collect())
            .unwrap_or_default()
    }

    // idに依存するprovider_idたちの取得
    pub(crate) fn dependents_of(&self, id: ProviderId) -> Vec<ProviderId> {
        self.dependents
            .get(&id)
            .map(|ids| ids.iter().cloned().collect())
            .unwrap_or_default()
    }
}
