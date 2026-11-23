pub trait Field<S> {
    type Value;

    fn get(state: &S) -> &Self::Value;

    fn get_mut(state: &mut S) -> &mut Self::Value;
}
