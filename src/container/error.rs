use crate::provider::ProviderId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerError {
    OutOfRange(ProviderId), //ID が Container の管理容量を超えている場合のエラー
    AlreadyRegistered(ProviderId), // 既にProviderが保存されている場合
    NotRegistered(ProviderId),
    TypeMismatch(ProviderId),
    KeyMismatch(ProviderId),
}
