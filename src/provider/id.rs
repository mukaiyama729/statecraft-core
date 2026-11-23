#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProviderId(usize);

impl ProviderId {
    pub const fn new(id: usize) -> Self {
        Self(id)
    }

    pub const fn index(self) -> usize {
        self.0
    }
}
