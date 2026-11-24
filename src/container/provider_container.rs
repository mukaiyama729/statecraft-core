use crate::{
    container::pointer_manager::ProviderPointerManager,
    provider::{
        base::{ProviderBase, ProviderId},
        functional::Provider,
    },
    reference::{
        build_context::{BuildContext, BuildFrame},
        provider_ref::Ref,
    },
};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    TypeMismatch,
    MissingState,
    CircularDependency(ProviderId),
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

        let _ = element.prepare_build();

        element.read_cached().ok_or(ContainerError::MissingState)
    }

    pub(crate) fn read_with_parent<T: Clone + PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
        parent: &BuildFrame<'_>,
    ) -> Result<T, ContainerError> {
        self.read_ref_with_parent(provider, Some(parent))
            .map(Clone::clone)
    }

    fn read_ref_with_parent<T: PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
        parent: Option<&BuildFrame<'_>>,
    ) -> Result<&T, ContainerError> {
        self.ensure_initialized(provider, parent)?;

        let pointer = self
            .pointer_manager
            .get(provider.id())
            .ok_or(ContainerError::MissingState)?;

        let element = pointer.typed::<T>().ok_or(ContainerError::TypeMismatch)?;

        element.read_cached().ok_or(ContainerError::MissingState)
    }

    fn ensure_initialized<T: PartialEq + 'static>(
        &mut self,
        provider: &'static Provider<T>,
        parent: Option<&BuildFrame<'_>>,
    ) -> Result<(), ContainerError> {
        let id = provider.id();

        if let Some(parent) = parent {
            if parent.contains(id) {
                return Err(ContainerError::CircularDependency(id));
            }
        }

        // ① Prepare
        let build_provider = {
            let pointer = self.pointer_manager.upsert(provider);

            let element = pointer.typed::<T>().ok_or(ContainerError::TypeMismatch)?;

            element.prepare_build()
        };

        // Cleanの場合は何もしない
        let Some(build_provider) = build_provider else {
            return Ok(());
        };

        // ② Build
        let next = {
            let frame = BuildFrame::new(id, parent);

            let context = BuildContext::new(self, &frame);

            let mut ref_ = Ref::new(context);

            build_provider.create(&mut ref_)?
        };

        // ③ Commit
        let pointer = self
            .pointer_manager
            .get_mut(id)
            .ok_or(ContainerError::MissingState)?;

        let element = pointer
            .typed_mut::<T>()
            .ok_or(ContainerError::TypeMismatch)?;

        let _ = element.set_value(next);

        Ok(())
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
