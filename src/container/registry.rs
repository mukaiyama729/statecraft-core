use core::any::Any;

use crate::{container::error::ContainerError, provider::ReadKey};

// N: コンパイル時に決定されるRegistryに格納できるProvider(ReadKey)の数
// Sync: 異なったthread間で安全に参照(&)を共有するためのトレイト
// これによって別スレッドからでもRegistryにあるProviderへReadアクセスが可能となる
// Send:スレッド間で所有権の受け渡しを可能にするトレーと
pub(super) struct ProviderRegistry<'a, const N: usize> {
    entries: [Option<&'a (dyn Any + Sync + Send)>; N],
}

impl<'a, const N: usize> ProviderRegistry<'a, N> {
    pub const fn new() -> Self {
        Self { entries: [None; N] }
    }

    pub fn register<S: 'static, T: 'static>(
        &mut self,
        provider: &'a ReadKey<S, T>,
    ) -> Result<(), ContainerError> {
        let id = provider.id();

        let entry = self
            .entries
            .get_mut(id.index())
            .ok_or(ContainerError::OutOfRange(id))?;
        if entry.is_some() {
            return Err(ContainerError::AlreadyRegistered(id));
        }

        *entry = Some(provider);

        Ok(())
    }

    pub fn validate<S: 'static, T: 'static>(
        &self,
        provider: &ReadKey<S, T>,
    ) -> Result<(), ContainerError> {
        let id = provider.id();

        let entry = self
            .entries
            .get(id.index())
            .ok_or(ContainerError::OutOfRange(id))?;

        let registered = (*entry).ok_or(ContainerError::NotRegistered(id))?;

        let typed = registered
            .downcast_ref::<ReadKey<S, T>>()
            .ok_or(ContainerError::TypeMismatch(id))?;

        if !core::ptr::eq(typed, provider) {
            return Err(ContainerError::KeyMismatch(id));
        }

        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entries.iter().filter(|item| item.is_some()).count()
    }

    pub const fn capacity(&self) -> usize {
        N
    }
}
