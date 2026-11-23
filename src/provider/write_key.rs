use crate::provider::{ProviderId, ReadKey};

pub struct WriteKey<'a, S: 'static, T: 'static> {
    read_key: &'a ReadKey<S, T>,
    getter_mut: fn(&mut S) -> &mut T,
}

impl<'a, S: 'static, T: 'static> WriteKey<'a, S, T> {
    pub const fn new(read_key: &'a ReadKey<S, T>, getter_mut: fn(&mut S) -> &mut T) -> Self {
        Self {
            read_key,
            getter_mut,
        }
    }
    pub const fn id(&self) -> ProviderId {
        self.read_key.id()
    }
    pub const fn read_key(&self) -> &'a ReadKey<S, T> {
        self.read_key
    }
    pub fn get_mut<'b>(&self, state: &'b mut S) -> &'b mut T {
        (self.getter_mut)(state)
    }
}
