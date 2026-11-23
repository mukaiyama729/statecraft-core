use crate::provider::{ProviderId, ReadKey, write_key::WriteKey};

pub struct StateProvider<'a, S: 'static, T: 'static> {
    writer: WriteKey<'a, S, T>,
}

impl<'a, S: 'static, T: 'static> StateProvider<'a, S, T> {
    pub const fn new(read_key: &'a ReadKey<S, T>, getter_mut: fn(&mut S) -> &mut T) -> Self {
        Self {
            writer: WriteKey::new(read_key, getter_mut),
        }
    }

    pub const fn id(&self) -> ProviderId {
        self.writer.id()
    }

    pub const fn read_key(&self) -> &'a ReadKey<S, T> {
        self.writer.read_key()
    }

    pub const fn write_key(&self) -> &WriteKey<S, T> {
        &self.writer
    }
}
