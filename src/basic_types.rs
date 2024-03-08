use std::fmt::Debug;

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