use std::ops::{Add, Div, Mul, Sub};
use crate::ratio::clamped_ratio::{ClampedRatio, ClampedRatioError};

#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default)]
pub struct AutoClampedRatio(ClampedRatio);

impl AutoClampedRatio {
    pub unsafe fn new_unchecked(value: f32) -> Self {
        unsafe { Self(ClampedRatio::new_unchecked(value)) }
    }

    pub fn from_clamped_ratio(clamped_ratio: ClampedRatio) -> Self {
        Self(clamped_ratio)
    }

    pub fn new(value: f32) -> Self {
        let clamped = ClampedRatio::new(value).unwrap_or_else(|error| match error {
            ClampedRatioError::Underflow => ClampedRatio::zero(),
            ClampedRatioError::Overflow => ClampedRatio::one(),
        });
        Self::from_clamped_ratio(clamped)
    }

    pub fn into_clamped_ratio(self) -> ClampedRatio {
        self.0
    }

    pub fn get(&self) -> f32 {
        self.into_clamped_ratio().get()
    }
}

impl From<ClampedRatio> for AutoClampedRatio {
    fn from(value: ClampedRatio) -> Self {
        Self::from_clamped_ratio(value)
    }
}

impl From<AutoClampedRatio> for ClampedRatio {
    fn from(value: AutoClampedRatio) -> Self {
        value.into_clamped_ratio()
    }
}

impl Add for AutoClampedRatio {
    type Output = AutoClampedRatio;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.get() + rhs.get())
    }
}

impl Sub for AutoClampedRatio {
    type Output = AutoClampedRatio;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.get() - rhs.get())
    }
}

impl Mul for AutoClampedRatio {
    type Output = AutoClampedRatio;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.get() * rhs.get())
    }
}


impl Div for AutoClampedRatio {
    type Output = AutoClampedRatio;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.get() * rhs.get())
    }
}
