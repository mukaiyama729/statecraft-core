use crate::{
    container::{error::ContainerError, registry::ProviderRegistry},
    provider::ReadKey,
};

/// Owns state and borrows registered provider keys for `'a`.
///
/// A registered key must remain alive while the container is used:
///
/// ```compile_fail,E0597
/// use statecraft_core::{container::ProviderContainer, provider::ReadKey};
///
/// let mut container = ProviderContainer::<u8, 1>::new(82);
/// {
///     let provider = ReadKey::new(0, |state: &u8| state);
///     container.register(&provider).unwrap();
/// }
/// assert_eq!(container.registered_count(), 1);
/// ```
pub struct ProviderContainer<'a, S, const N: usize> {
    state: S,
    registry: ProviderRegistry<'a, N>,
}

impl<'a, S: 'static, const N: usize> ProviderContainer<'a, S, N> {
    pub const fn new(state: S) -> Self {
        Self {
            state,
            registry: ProviderRegistry::new(),
        }
    }

    pub fn register<T: 'static>(
        &mut self,
        provider: &'a ReadKey<S, T>,
    ) -> Result<(), ContainerError> {
        self.registry.register(provider)
    }

    /// Returns a reference tied to the borrow of this container, not of the key.
    ///
    /// The reference cannot outlive the container that owns the state:
    ///
    /// ```compile_fail,E0597
    /// use statecraft_core::{container::ProviderContainer, provider::ReadKey};
    ///
    /// static PROVIDER: ReadKey<String, String> = ReadKey::new(0, |state| state);
    /// let value;
    /// {
    ///     let mut container = ProviderContainer::<_, 1>::new(String::from("state"));
    ///     container.register(&PROVIDER).unwrap();
    ///     value = container.read(&PROVIDER).unwrap();
    /// }
    /// assert_eq!(value, "state");
    /// ```
    pub fn read<T: 'static>(&self, provider: &ReadKey<S, T>) -> Result<&T, ContainerError> {
        self.registry.validate(provider)?;

        Ok(provider.get(&self.state))
    }

    pub fn registered_count(&self) -> usize {
        self.registry.len()
    }

    pub const fn capacity(&self) -> usize {
        self.registry.capacity()
    }
}
