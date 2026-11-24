use alloc::boxed::Box;

use crate::{
    element::{element::ProviderElement, erased::ErasedElement},
    provider::base::ProviderId,
};

pub(crate) struct ProviderPointer {
    element: Box<dyn ErasedElement>,
}

impl ProviderPointer {
    pub(crate) fn new<T: 'static>(element: ProviderElement<T>) -> Self {
        Self {
            element: Box::new(element),
        }
    }

    pub(crate) fn provider_id(&self) -> ProviderId {
        self.element.provider_id()
    }

    pub(crate) fn invalidate(&mut self) {
        self.element.invalidate();
    }

    pub(crate) fn typed<T: 'static>(&self) -> Option<&ProviderElement<T>> {
        self.element.as_any().downcast_ref::<ProviderElement<T>>()
    }

    pub(crate) fn typed_mut<T: 'static>(&mut self) -> Option<&mut ProviderElement<T>> {
        self.element
            .as_any_mut()
            .downcast_mut::<ProviderElement<T>>()
    }
}
