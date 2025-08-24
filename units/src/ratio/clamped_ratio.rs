use crate::ratio::auto_clamped_ratio::AutoClampedRatio;

/// A floating-point value inside [0, 1].
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default)]
pub struct ClampedRatio(f32);

pub enum ClampedRatioError {
    Underflow,
    Overflow,
}

impl ClampedRatio {
    pub unsafe fn new_unchecked(value: f32) -> Self {
        Self(value)
    }

    pub fn new(value: f32) -> Result<Self, ClampedRatioError> {
        match value {
            0.0..=1.0 => Ok(unsafe { Self::new_unchecked(value) }),
            ..0.0 => Err(ClampedRatioError::Underflow),
            _ => Err(ClampedRatioError::Overflow),
        }
    }

    pub fn zero() -> Self {
        unsafe { Self::new_unchecked(0.0) }
    }

    pub fn one() -> Self {
        unsafe { Self::new_unchecked(1.0) }
    }

    pub fn get(self) -> f32 {
        self.0
    }

    pub fn auto_clamp(self) -> AutoClampedRatio {
        self.into()
    }
}
