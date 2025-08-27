use crate::bounds::out_of_bounds_error::OutOfBoundsError;

macro_rules! checked_operator {
    ($name: ident) => {
        pub trait $name {
            fn add(self, rhs: Self) -> Result<Self, OutOfBoundsError>
            where
                Self: Sized;
        }
    };
}

checked_operator!(CheckedAdd);
checked_operator!(CheckedSub);
checked_operator!(CheckedMul);
checked_operator!(CheckedDiv);
