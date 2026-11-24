#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProviderId(usize);

impl ProviderId {
    pub fn as_usize(self) -> usize {
        self.0
    }
}

pub struct ProviderMeta {
    name: &'static str,
    auto_dispose: bool,
}

impl ProviderMeta {
    pub const fn new(name: &'static str) -> Self {
        Self {
            name,
            auto_dispose: false,
        }
    }

    pub const fn with_auto_dispose(mut self, enabled: bool) -> Self {
        self.auto_dispose = enabled;
        self
    }

    pub fn id(&'static self) -> ProviderId {
        ProviderId(self as *const Self as usize)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn is_auto_dispose(&self) -> bool {
        self.auto_dispose
    }
}

pub trait ProviderBase: Send + Sync + 'static {
    type State: 'static;

    fn meta(&'static self) -> &'static ProviderMeta;

    fn id(&'static self) -> ProviderId {
        self.meta().id()
    }

    fn name(&'static self) -> &'static str {
        self.meta().name()
    }

    fn is_auto_dispose(&'static self) -> bool {
        self.meta().is_auto_dispose()
    }
}
