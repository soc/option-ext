mod contains;

/// Extension trait providing additional methods for `Option`.
pub trait OptionExt<T> {
    /// Returns `true` if the option is a [`Some`] value containing the given value.
    ///
    /// # Examples
    ///
    /// ```
    /// use option_ext::OptionExt;
    ///
    /// let x: Option<u32> = Some(2);
    /// assert_eq!(x.contains(&2), true);
    ///
    /// let x: Option<u32> = Some(3);
    /// assert_eq!(x.contains(&2), false);
    ///
    /// let x: Option<u32> = None;
    /// assert_eq!(x.contains(&2), false);
    /// ```
    #[must_use]
    fn contains<U>(&self, x: &U) -> bool where U: PartialEq<T>;
}

