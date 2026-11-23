use core::marker::PhantomData;

/// Associates notifier logic with a field type, without borrowing a provider key.
pub struct NotifierProvider<F, L> {
    field: PhantomData<fn() -> F>,
    logic: L,
}

impl<F, L> NotifierProvider<F, L> {
    pub const fn new(logic: L) -> Self {
        Self {
            field: PhantomData,
            logic,
        }
    }

    pub fn logic(&self) -> &L {
        &self.logic
    }
}
