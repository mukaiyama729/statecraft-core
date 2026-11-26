use crate::provider::ProviderId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DependencyEdge {
    pub(crate) subscriber: ProviderId,
    pub(crate) dependency: ProviderId,
}

impl DependencyEdge {
    pub(crate) const fn new(subscriber: ProviderId, dependency: ProviderId) -> Self {
        Self {
            subscriber,
            dependency,
        }
    }
}
