use crate::ratio::clamped_ratio::{ClampedRatio, ClampedRatioError};
use std::ops::{Add, Div, Mul, Sub};

/// A floating-point value inside [0, 1].
///
/// Unlike `ClampedRatio`, 
/// `AutoClampedRatio` automatically rounds values `< 0` to `0` and `> 1` to `1`.
///
/// This is basically a convenience wrapper around `ClampedRatio`
/// that provides a default behavior when encountering out-of-bound values.
///
/// However, please prefer using `ClampedRatio` as field, parameter, and return types,
/// since this implementation assumes that you want the auto-coercing behavior,
/// which is not always the obvious default behavior.
///
/// Example:
///
/// ```
/// use units::ratio::auto_clamped_ratio::AutoClampedRatio;
///
/// fn foo(
///     a: AutoClampedRatio,
///     b: AutoClampedRatio,
///     c: AutoClampedRatio,
/// ) -> AutoClampedRatio {
///     a + b - c
/// }
/// ```
///
/// In the example above, if `a + b > 100` and you subtract `c`, you will lose the amount that was
/// over `100` after doing `a + b`, since auto-coercion is done after every operation.
/// This happens even if `a + b - c` would have been `< 100` if we did the operations on the
/// underlying values instead.
///
/// `AutoClampedRatio` should be used as an opt-in to the auto-coercion behavior as needed;
/// i.e., right when you are about to do the operations, by using `ClampedRatio::auto_clamp`.
#[derive(PartialEq, PartialOrd, Copy, Clone, Debug, Default)]
pub struct AutoClampedRatio(ClampedRatio);

impl AutoClampedRatio {
    /// # Safety
    ///
    /// The caller must guarantee that `value` is inside `[0, 1]`.
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

    pub fn zero() -> Self {
        Self::from_clamped_ratio(ClampedRatio::zero())
    }

    pub fn one() -> Self {
        Self::from_clamped_ratio(ClampedRatio::one())
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
        Self::new(self.get() / rhs.get())
    }
}
