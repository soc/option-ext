use OptionExt;

impl<T> OptionExt<T> for Option<T> {
    fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T> {
        match *self {
            Some(ref y) => x == y,
            None => false,
        }
    }
}
