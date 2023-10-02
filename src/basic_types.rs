use std::{borrow::Cow, fmt::Debug};

#[derive(Debug, Clone)]
pub struct BasicCowStr<'text> {
    inner: Cow<'text, str>,
}

impl<'text> BasicCowStr<'text> {
    pub fn new(inner: Cow<'text, str>) -> Self {
        assert!(
            u32::try_from(inner.len()).is_ok(),
            "We only support string up to 4GB in size, which is the maximum size of the u32."
        );
        Self { inner }
    }

    pub fn len(&self) -> u32 {
        // We can safely do converting here because we have already asserted that
        // the length of the string is less than or equal `u32::MAX`
        self.inner.len() as u32
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_ref()
    }
}

impl<'text> std::ops::Deref for BasicCowStr<'text> {
    type Target = Cow<'text, str>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'text, T: Into<Cow<'text, str>>> From<T> for BasicCowStr<'text> {
    fn from(value: T) -> Self {
        Self::new(value.into())
    }
}

// This is basically doing the same thing as `TryInto<u32>`.
// If we use `TryInto<u32>`, we need to put `where <T as TryInto<u32>>::Error: Debug` everywhere.
pub trait AssertIntoU32 {
    fn assert_into_u32(self) -> u32;
}

impl<T: TryInto<u32>> AssertIntoU32 for T
where
    <T as TryInto<u32>>::Error: Debug,
{
    fn assert_into_u32(self) -> u32 {
        self.try_into().unwrap()
    }
}