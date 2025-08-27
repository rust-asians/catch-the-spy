use crate::ratio::auto_clamped_ratio::SaturatingClampedRatio;

/// A floating-point value inside [0, 1].
///
/// `ClampedRatio` does not assume how underflows and overflows should be handled.
/// For this reason, this type should be the default over other implementations as
/// field, parameter, and return types, and other implementations should just be used as needed
/// on the use site.
///
/// If you want underflows and overflows to be automatically coerced to 0 and 1 respectively,
/// use `SaturatingClampedRatio` by calling `ClampedRatio::auto_clamp`.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default)]
pub struct ClampedRatio(f32);

pub enum ClampedRatioError {
    Underflow,
    Overflow,
}

impl ClampedRatio {
    /// # Safety
    ///
    /// The caller must guarantee that `value` is inside `[0, 1]`.
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

    pub fn auto_clamp(self) -> SaturatingClampedRatio {
        self.into()
    }
}
