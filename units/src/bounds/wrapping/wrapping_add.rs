use std::ops::Add;
use crate::bounds::checked::CheckedAdd;

pub struct WrappingAdd<T: CheckedAdd>(T);

impl<T: CheckedAdd> WrappingAdd<T> {
    fn from_checked_add(checked_add: T) -> Self {
        Self(checked_add)
    }
}

impl<T: CheckedAdd> Add for WrappingAdd<T> {
    type Output = T;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
