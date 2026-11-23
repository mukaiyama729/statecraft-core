use crate::{container::ContainerError, state::Field};

/// Owns state and accesses its values through [`Field`] implementations.
/// Fields do not need keys or registration.
///
/// Accessors retain the `Result` return type, but currently always return `Ok`.
pub struct ProviderContainer<S> {
    state: S,
}

impl<S> ProviderContainer<S> {
    pub const fn new(state: S) -> Self {
        Self { state }
    }

    /// Returns a reference tied to the borrow of this container.
    ///
    /// The reference cannot outlive the container that owns the state:
    ///
    /// ```compile_fail,E0597
    /// use statecraft_core::{container::ProviderContainer, state::Field};
    ///
    /// struct Text;
    /// impl Field<String> for Text {
    ///     type Value = String;
    ///     fn get(state: &String) -> &String { state }
    ///     fn get_mut(state: &mut String) -> &mut String { state }
    /// }
    /// let value;
    /// {
    ///     let container = ProviderContainer::new(String::from("state"));
    ///     value = container.read::<Text>().unwrap();
    /// }
    /// assert_eq!(value, "state");
    /// ```
    pub fn read<F: Field<S>>(&self) -> Result<&F::Value, ContainerError> {
        Ok(F::get(&self.state))
    }

    /// Updates a field and returns the closure's result.
    pub fn update<F: Field<S>, R>(
        &mut self,
        f: impl FnOnce(&mut F::Value) -> R,
    ) -> Result<R, ContainerError> {
        Ok(f(F::get_mut(&mut self.state)))
    }

    pub fn set<F: Field<S>>(&mut self, value: F::Value) -> Result<(), ContainerError> {
        *F::get_mut(&mut self.state) = value;
        Ok(())
    }
}
