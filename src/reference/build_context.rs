use crate::{container::provider_container::ProviderContainer, provider::base::ProviderId};

pub(crate) struct BuildFrame<'a> {
    owner: ProviderId,
    parent: Option<&'a BuildFrame<'a>>,
}

impl<'a> BuildFrame<'a> {
    pub(crate) fn new(owner: ProviderId, parent: Option<&'a BuildFrame<'a>>) -> Self {
        Self { owner, parent }
    }

    pub(crate) fn owner(&self) -> ProviderId {
        self.owner
    }

    pub(crate) fn contains(&self, id: ProviderId) -> bool {
        let mut current = Some(self);

        while let Some(frame) = current {
            if frame.owner == id {
                return true;
            }

            current = frame.parent;
        }

        false
    }
}

pub(crate) struct BuildContext<'a> {
    pub(crate) container: &'a mut ProviderContainer,
    pub(crate) frame: &'a BuildFrame<'a>,
}

impl<'a> BuildContext<'a> {
    pub(crate) fn new(container: &'a mut ProviderContainer, frame: &'a BuildFrame<'a>) -> Self {
        Self { container, frame }
    }
}
