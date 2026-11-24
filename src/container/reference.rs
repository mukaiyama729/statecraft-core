use alloc::vec::Vec;

use crate::key::ProviderKey;

pub struct ProviderRef<'key> {
    //pub(crate) container: &'a mut ProviderContainer,
    pub(crate) key: ProviderKey<'key>,
    pub(crate) dependencies: Vec<ProviderKey<'key>>,
}
