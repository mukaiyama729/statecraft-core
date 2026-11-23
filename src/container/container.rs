use crate::{
    container::{error::ContainerError, registry::ProviderRegistry},
    provider::{ReadKey, StateProvider},
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
/// stateの所有権はここで移動させられる
/// registryも
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

    pub fn register_state<T: 'static>(
        &mut self,
        provider: &'a StateProvider<S, T>,
    ) -> Result<(), ContainerError> {
        self.register(provider.read_key())
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

    pub fn read_state<'b, T: 'static>(
        &self,
        provider: &'b StateProvider<S, T>,
    ) -> Result<&T, ContainerError> {
        self.read(provider.read_key())
    }

    pub fn update<T: 'static, R>(
        &mut self,
        provider: &'a StateProvider<S, T>,
        f: impl FnOnce(&mut T) -> R,
    ) -> Result<R, ContainerError> {
        self.registry.validate(provider.read_key())?;
        Ok(f(provider.write_key().get_mut(&mut self.state)))
    }

    pub fn set<T: 'static>(
        &mut self,
        provider: &'a StateProvider<S, T>,
        value: T,
    ) -> Result<(), ContainerError> {
        self.update(provider, |slot| {
            *slot = value;
        })
    }

    pub fn registered_count(&self) -> usize {
        self.registry.len()
    }

    pub const fn capacity(&self) -> usize {
        self.registry.capacity()
    }
}
