macro_rules! non_negative_operators {
    ($name: ident, $inner_type: ty) => {
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

pub(super) use non_negative_operators;
