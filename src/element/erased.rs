use core::any::Any;

use crate::provider::base::ProviderId;

use super::element::{ElementStatus, ProviderElement};

pub trait ErasedElement {
    fn provider_id(&self) -> ProviderId;

    fn status(&self) -> ElementStatus;

    fn invalidate(&mut self);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ErasedElement for ProviderElement<T> {
    fn provider_id(&self) -> ProviderId {
        ProviderElement::provider_id(self)
    }

    fn status(&self) -> ElementStatus {
        ProviderElement::status(self)
    }

    fn invalidate(&mut self) {
        ProviderElement::invalidate(self);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
