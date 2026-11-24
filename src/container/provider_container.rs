use crate::{
    container::pointer_manager::ProviderPointerManager,
    provider::{base::ProviderBase, functional::Provider},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    TypeMismatch,
    MissingState,
}

pub struct ProviderContainer {
    pointer_manager: ProviderPointerManager,
}

impl ProviderContainer {
    pub const fn new() -> Self {
        Self {
            pointer_manager: ProviderPointerManager::new(),
        }
    }

    pub fn read_ref<T: PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
    ) -> Result<&T, ContainerError> {
        let pointer = self.pointer_manager.upsert(provider);

        let element = pointer
            .typed_mut::<T>()
            .ok_or(ContainerError::TypeMismatch)?;

        let _ = element.flush();

        element.read_cached().ok_or(ContainerError::MissingState)
    }

    pub fn read<T: Clone + PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
    ) -> Result<T, ContainerError> {
        self.read_ref(provider).map(Clone::clone)
    }

    pub fn invalidate<P: ProviderBase>(&mut self, provider: &'static P) -> bool {
        match self.pointer_manager.get_mut(provider.id()) {
            Some(pointer) => {
                pointer.invalidate();
                true
            }

            None => false,
        }
    }

    pub fn refresh<T: Clone + PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
    ) -> Result<T, ContainerError> {
        self.invalidate(provider);

        self.read(provider)
    }

    pub fn exists<P: ProviderBase>(&self, provider: &'static P) -> bool {
        self.pointer_manager.contains(provider.id())
    }

    pub fn remove<P: ProviderBase>(&mut self, provider: &'static P) -> bool {
        self.pointer_manager.remove(provider.id()).is_some()
    }

    pub fn clear(&mut self) {
        self.pointer_manager.clear();
    }

    pub fn len(&self) -> usize {
        self.pointer_manager.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for ProviderContainer {
    fn default() -> Self {
        Self::new()
    }
}
