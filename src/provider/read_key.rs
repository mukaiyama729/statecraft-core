use crate::provider::id::ProviderId;

// S: Containerが保持する状態全体の方
// T: この Provider が参照する個別の状態の型
// Containerという大きな状態の箱をProviderを通して必要な値だけReadする
pub struct ReadKey<S, T> {
    id: ProviderId,
    getter: fn(&S) -> &T,
}

impl<S, T> ReadKey<S, T> {
    pub const fn new(id: usize, getter: fn(&S) -> &T) -> Self {
        Self {
            id: ProviderId::new(id),
            getter: getter,
        }
    }

    pub const fn id(&self) -> ProviderId {
        self.id
    }

    pub fn get<'a>(&self, state: &'a S) -> &'a T {
        (self.getter)(state)
    }
}

impl<S, T> Copy for ReadKey<S, T> {}

impl<S, T> Clone for ReadKey<S, T> {
    fn clone(&self) -> Self {
        *self
    }
}
