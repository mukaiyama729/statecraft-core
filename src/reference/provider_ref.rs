use crate::{
    container::provider_container::ContainerError,
    provider::{base::ProviderId, functional::Provider},
    reference::build_context::BuildContext,
};

pub struct Ref<'a> {
    context: BuildContext<'a>,
}

impl<'a> Ref<'a> {
    pub(crate) fn new(context: BuildContext<'a>) -> Self {
        Self { context }
    }

    pub fn provider_id(&self) -> ProviderId {
        self.context.frame.owner()
    }

    pub fn read<T: Clone + PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
    ) -> Result<T, ContainerError> {
        self.context
            .container
            .read_with_parent(provider, self.context.frame)
    }
}
