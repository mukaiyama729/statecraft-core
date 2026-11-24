use crate::provider::base::ProviderId;

pub(crate) struct BuildContext {
    owner: ProviderId,
}

impl BuildContext {
    pub(crate) fn new(owner: ProviderId) -> Self {
        Self { owner }
    }
}

pub struct Ref<'a> {
    context: &'a mut BuildContext,
}

impl<'a> Ref<'a> {
    pub(crate) fn new(context: &'a mut BuildContext) -> Self {
        Self { context }
    }

    pub fn provider_id(&self) -> ProviderId {
        self.context.owner
    }
}
