use std::ops::{Add, Div, Mul};

macro_rules! non_negative_integer {
    ($name: ident, $inner_type: ty) => {
        #[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash, Debug)]
        pub struct $name($inner_type);

        impl $name {
            /// # Safety
            ///
            /// The caller must guarantee that `value` is not `< 0`.
            pub unsafe fn new_unchecked(value: $inner_type) -> Self {
                Self(value)
            }

            pub fn zero() -> Self {
                unsafe { Self::new_unchecked(0) }
            }

            pub fn one() -> Self {
                unsafe { Self::new_unchecked(1) }
            }

            pub fn new(value: $inner_type) -> Option<Self> {
                if value >= 0 {
                    Some(unsafe { Self::new_unchecked(value) })
                } else {
                    None
                }
            }

            pub fn get(self) -> $inner_type {
                self.0
            }
        }

        impl Add for $name {
            type Output = $name;

            fn add(self, rhs: Self) -> Self::Output {
                unsafe { Self::new_unchecked(self.get() + rhs.get()) }
            }
        }

        impl Mul for $name {
            type Output = $name;

            fn mul(self, rhs: Self) -> Self::Output {
                unsafe { Self::new_unchecked(self.get() * rhs.get()) }
            }
        }

        impl Div for $name {
            type Output = $name;

            fn div(self, rhs: Self) -> Self::Output {
                unsafe { Self::new_unchecked(self.get() / rhs.get()) }
            }
        }
    };
}

non_negative_integer!(NonNegativeI8, i8);
non_negative_integer!(NonNegativeI16, i16);
non_negative_integer!(NonNegativeI32, i32);
non_negative_integer!(NonNegativeI64, i64);
non_negative_integer!(NonNegativeI128, i128);
non_negative_integer!(NonNegativeIsize, isize);
