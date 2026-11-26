use core::any::Any;
use core::marker::PhantomData;

use crate::container::provider_container::ContainerError;

pub(crate) trait ErasedListener {
    fn notify(&mut self, previous: Option<&dyn Any>, next: &dyn Any) -> Result<(), ContainerError>;
}

pub(crate) struct Listener<T, F> {
    callback: F,
    marker: PhantomData<fn(&T)>,
}

impl<T, F> Listener<T, F> {
    pub(crate) fn new(callback: F) -> Self {
        Self {
            callback,
            marker: PhantomData,
        }
    }
}

impl<T, F> ErasedListener for Listener<T, F>
where
    T: 'static,
    F: FnMut(Option<&T>, &T),
{
    fn notify(&mut self, previous: Option<&dyn Any>, next: &dyn Any) -> Result<(), ContainerError> {
        let next = next
            .downcast_ref::<T>()
            .ok_or(ContainerError::TypeMismatch)?;

        let previous = match previous {
            Some(value) => Some(
                value
                    .downcast_ref::<T>()
                    .ok_or(ContainerError::TypeMismatch)?,
            ),

            None => None,
        };

        (self.callback)(previous, next);

        Ok(())
    }
}
