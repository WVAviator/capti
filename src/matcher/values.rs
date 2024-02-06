use crate::matcher::{MatchCmp, MatchResult};

macro_rules! impl_value_mismatch {
    ($type:ty) => {
        impl MatchCmp for $type {
            fn match_cmp(&self, other: &Self) -> MatchResult {
                if self == other {
                    MatchResult::Matches
                } else {
                    MatchResult::ValueMismatch {
                        expected: self.to_string(),
                        actual: other.to_string(),
                        context: None,
                    }
                }
            }
        }
    };
}

impl_value_mismatch!(u8);
impl_value_mismatch!(u16);
impl_value_mismatch!(u32);
impl_value_mismatch!(u64);
impl_value_mismatch!(i8);
impl_value_mismatch!(i16);
impl_value_mismatch!(i32);
impl_value_mismatch!(i64);
impl_value_mismatch!(f32);
impl_value_mismatch!(f64);
