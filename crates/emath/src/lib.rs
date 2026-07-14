//! Opinionated 2D math library for building GUIs.
//!
//! Includes vectors, positions, rectangles etc.
//!
//! Conventions (unless otherwise specified):
//!
//! * All angles are in radians
//! * X+ is right and Y+ is down.
//! * (0,0) is left top.
//! * Dimension order is always `x y`
//!
//! ## Integrating with other math libraries.
//! `emath` does not strive to become a general purpose or all-powerful math library.
//!
//! For that, use something else ([`glam`](https://docs.rs/glam), [`nalgebra`](https://docs.rs/nalgebra), …)
//! and enable the `mint` feature flag in `emath` to enable implicit conversion to/from `emath`.
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!


#![no_std]
#![expect(clippy::float_cmp)]
#[macro_use]
extern crate alloc;

use core::ops::{Add, Div, Mul, RangeInclusive, Sub};
use alloc::string::String;

// ----------------------------------------------------------------------------

pub mod align;
pub mod easing;
mod gui_rounding;
mod history;
mod numeric;
mod ordered_float;
mod pos2;
mod range;
mod rect;
mod rect_align;
mod rect_transform;
mod rot2;
pub mod smart_aim;
mod ts_transform;
mod vec2;
mod vec2b;

pub use self::{
    align::{Align, Align2},
    gui_rounding::{GUI_ROUNDING, GuiRounding},
    history::History,
    numeric::*,
    ordered_float::*,
    pos2::*,
    range::Rangef,
    rect::*,
    rect_align::RectAlign,
    rect_transform::*,
    rot2::*,
    ts_transform::*,
    vec2::*,
    vec2b::*,
};

// ----------------------------------------------------------------------------

/// Helper trait to implement [`lerp`] and [`remap`].
pub trait One {
    const ONE: Self;
}

impl One for f32 {
    const ONE: Self = 1.0;
}

impl One for f64 {
    const ONE: Self = 1.0;
}

/// Helper trait to implement [`lerp`] and [`remap`].
pub trait Real:
    Copy
    + PartialEq
    + PartialOrd
    + One
    + Add<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Div<Self, Output = Self>
{
}

impl Real for f32 {}

impl Real for f64 {}

// ----------------------------------------------------------------------------
// no_std float math via libm

pub trait FloatExt32: Sized {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn fract(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn powf(self, exp: Self) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn acos(self) -> Self;
    fn cbrt(self) -> Self;
    fn abs(self) -> Self;
    fn trunc(self) -> Self;
    fn ln(self) -> Self;
    fn exp(self) -> Self;
}

impl FloatExt32 for f32 {
    fn floor(self) -> Self { libm::floorf(self) as f32 }
    fn ceil(self) -> Self { libm::ceilf(self) as f32 }
    fn round(self) -> Self { libm::roundf(self) as f32 }
    fn fract(self) -> Self { self - libm::floorf(self) as f32 }
    fn sqrt(self) -> Self { libm::sqrtf(self) }
    fn sin(self) -> Self { libm::sinf(self) }
    fn cos(self) -> Self { libm::cosf(self) }
    fn tan(self) -> Self { libm::tanf(self) }
    fn atan2(self, other: Self) -> Self { libm::atan2f(self, other) }
    fn powf(self, exp: Self) -> Self { libm::powf(self, exp) }
    fn powi(self, exp: i32) -> Self { libm::powf(self, exp as f32) }
    fn hypot(self, other: Self) -> Self { libm::hypotf(self, other) }
    fn sin_cos(self) -> (Self, Self) { libm::sincosf(self) }
    fn acos(self) -> Self { libm::acosf(self) }
    fn cbrt(self) -> Self { libm::cbrtf(self) }
    fn abs(self) -> Self { libm::fabsf(self) }
    fn trunc(self) -> Self { libm::truncf(self) }
    fn ln(self) -> Self { libm::logf(self) }
    fn exp(self) -> Self { libm::expf(self) }
}

pub trait FloatExt64: Sized {
    fn floor(self) -> Self;
    fn round(self) -> Self;
    fn powf(self, exp: Self) -> Self;
    fn powi(self, exp: i32) -> Self;
    fn log10(self) -> Self;
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn sin_cos(self) -> (Self, Self);
    fn abs(self) -> Self;
    fn fract(self) -> Self;
    fn ceil(self) -> Self;
    fn trunc(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn ln(self) -> Self;
    fn exp(self) -> Self;
}

impl FloatExt64 for f64 {
    fn floor(self) -> Self { libm::floor(self) }
    fn round(self) -> Self { libm::round(self) }
    fn powf(self, exp: Self) -> Self { libm::pow(self, exp) }
    fn powi(self, exp: i32) -> Self { libm::pow(self, exp as f64) }
    fn log10(self) -> Self { libm::log10(self) }
    fn sqrt(self) -> Self { libm::sqrt(self) }
    fn sin(self) -> Self { libm::sin(self) }
    fn cos(self) -> Self { libm::cos(self) }
    fn tan(self) -> Self { libm::tan(self) }
    fn sin_cos(self) -> (Self, Self) { libm::sincos(self) }
    fn abs(self) -> Self { libm::fabs(self) }
    fn fract(self) -> Self { self - libm::floor(self) }
    fn ceil(self) -> Self { libm::ceil(self) }
    fn trunc(self) -> Self { libm::trunc(self) }
    fn hypot(self, other: Self) -> Self { libm::hypot(self, other) }
    fn atan2(self, other: Self) -> Self { libm::atan2(self, other) }
    fn ln(self) -> Self { libm::log(self) }
    fn exp(self) -> Self { libm::exp(self) }
}

pub mod prelude {
    pub use crate::{FloatExt32, FloatExt64};
    pub use alloc::format;
    pub use alloc::string::String;
}

// ----------------------------------------------------------------------------

/// Linear interpolation.
///
/// ```
/// # use emath::lerp;
/// assert_eq!(lerp(1.0..=5.0, 0.0), 1.0);
/// assert_eq!(lerp(1.0..=5.0, 0.5), 3.0);
/// assert_eq!(lerp(1.0..=5.0, 1.0), 5.0);
/// assert_eq!(lerp(1.0..=5.0, 2.0), 9.0);
/// ```
#[inline(always)]
pub fn lerp<R, T>(range: impl Into<RangeInclusive<R>>, t: T) -> R
where
    T: Real + Mul<R, Output = R>,
    R: Copy + Add<R, Output = R>,
{
    let range = range.into();
    (T::ONE - t) * *range.start() + t * *range.end()
}

/// This is a faster version of [`f32::midpoint`] which doesn't handle overflow.
///
/// ```
/// # use emath::fast_midpoint;
/// assert_eq!(fast_midpoint(1.0, 5.0), 3.0);
/// ```
#[inline(always)]
pub fn fast_midpoint<R>(a: R, b: R) -> R
where
    R: Copy + Add<R, Output = R> + Div<R, Output = R> + One,
{
    let two = R::ONE + R::ONE;
    (a + b) / two
}

/// Where in the range is this value? Returns 0-1 if within the range.
///
/// Returns <0 if before and >1 if after.
///
/// Returns `None` if the input range is zero-width.
///
/// ```
/// # use emath::inverse_lerp;
/// assert_eq!(inverse_lerp(1.0..=5.0, 1.0), Some(0.0));
/// assert_eq!(inverse_lerp(1.0..=5.0, 3.0), Some(0.5));
/// assert_eq!(inverse_lerp(1.0..=5.0, 5.0), Some(1.0));
/// assert_eq!(inverse_lerp(1.0..=5.0, 9.0), Some(2.0));
/// assert_eq!(inverse_lerp(1.0..=1.0, 3.0), None);
/// ```
#[inline]
pub fn inverse_lerp<R>(range: RangeInclusive<R>, value: R) -> Option<R>
where
    R: Copy + PartialEq + Sub<R, Output = R> + Div<R, Output = R>,
{
    let min = *range.start();
    let max = *range.end();
    if min == max {
        None
    } else {
        Some((value - min) / (max - min))
    }
}

/// Linearly remap a value from one range to another,
/// so that when `x == from.start()` returns `to.start()`
/// and when `x == from.end()` returns `to.end()`.
pub fn remap<T>(x: T, from: impl Into<RangeInclusive<T>>, to: impl Into<RangeInclusive<T>>) -> T
where
    T: Real,
{
    let from = from.into();
    let to = to.into();
    debug_assert!(
        from.start() != from.end(),
        "from.start() and from.end() should not be equal"
    );
    let t = (x - *from.start()) / (*from.end() - *from.start());
    lerp(to, t)
}

/// Like [`remap`], but also clamps the value so that the returned value is always in the `to` range.
pub fn remap_clamp<T>(
    x: T,
    from: impl Into<RangeInclusive<T>>,
    to: impl Into<RangeInclusive<T>>,
) -> T
where
    T: Real,
{
    let from = from.into();
    let to = to.into();
    if from.end() < from.start() {
        return remap_clamp(x, *from.end()..=*from.start(), *to.end()..=*to.start());
    }
    if x <= *from.start() {
        *to.start()
    } else if *from.end() <= x {
        *to.end()
    } else {
        debug_assert!(
            from.start() != from.end(),
            "from.start() and from.end() should not be equal"
        );
        let t = (x - *from.start()) / (*from.end() - *from.start());
        // Ensure no numerical inaccuracies sneak in:
        if T::ONE <= t { *to.end() } else { lerp(to, t) }
    }
}

/// Round a value to the given number of decimal places.
pub fn round_to_decimals(value: f64, decimal_places: usize) -> f64 {
    // This is a stupid way of doing this, but stupid works.
    alloc::format!("{value:.decimal_places$}").parse().unwrap_or(value)
}

pub fn format_with_minimum_decimals(value: f64, decimals: usize) -> String {
    format_with_decimals_in_range(value, decimals..=6)
}

/// Use as few decimals as possible to show the value accurately, but within the given range.
///
/// Decimals are counted after the decimal point.
pub fn format_with_decimals_in_range(value: f64, decimal_range: RangeInclusive<usize>) -> String {
    let min_decimals = *decimal_range.start();
    let max_decimals = *decimal_range.end();
    debug_assert!(
        min_decimals <= max_decimals,
        "min_decimals should be <= max_decimals, but got min_decimals: {min_decimals}, max_decimals: {max_decimals}"
    );
    debug_assert!(
        max_decimals < 100,
        "max_decimals should be < 100, but got {max_decimals}"
    );
    let max_decimals = max_decimals.min(16);
    let min_decimals = min_decimals.min(max_decimals);

    if min_decimals < max_decimals {
        // Ugly/slow way of doing this. TODO(emilk): clean up precision.
        for decimals in min_decimals..max_decimals {
            let text = alloc::format!("{value:.decimals$}");
            let epsilon = 16.0 * f32::EPSILON; // margin large enough to handle most peoples round-tripping needs
            if let Ok(parsed_value) = text.parse::<f32>()
                && almost_equal(parsed_value, value as f32, epsilon)
            {
                // Enough precision to show the value accurately - good!
                return text;
            }
        }
        // The value has more precision than we expected.
        // Probably the value was set not by the slider, but from outside.
        // In any case: show the full value
    }
    alloc::format!("{value:.max_decimals$}")
}

/// Return true when arguments are the same within some rounding error.
///
/// For instance `almost_equal(x, x.to_degrees().to_radians(), f32::EPSILON)` should hold true for all x.
/// The `epsilon`  can be `f32::EPSILON` to handle simple transforms (like degrees -> radians)
/// but should be higher to handle more complex transformations.
pub fn almost_equal(a: f32, b: f32, epsilon: f32) -> bool {
    if a == b {
        true // handle infinites
    } else {
        let abs_max = a.abs().max(b.abs());
        abs_max <= epsilon || ((a - b).abs() / abs_max) <= epsilon
    }
}

// ----------------------------------------------------------------------------

/// Extends `f32`, [`Vec2`] etc with `at_least` and `at_most` as aliases for `max` and `min`.
pub trait NumExt {
    /// More readable version of `self.max(lower_limit)`
    #[must_use]
    fn at_least(self, lower_limit: Self) -> Self;

    /// More readable version of `self.min(upper_limit)`
    #[must_use]
    fn at_most(self, upper_limit: Self) -> Self;
}

macro_rules! impl_num_ext {
    ($t: ty) => {
        impl NumExt for $t {
            #[inline(always)]
            fn at_least(self, lower_limit: Self) -> Self {
                self.max(lower_limit)
            }

            #[inline(always)]
            fn at_most(self, upper_limit: Self) -> Self {
                self.min(upper_limit)
            }
        }
    };
}

impl_num_ext!(u8);
impl_num_ext!(u16);
impl_num_ext!(u32);
impl_num_ext!(u64);
impl_num_ext!(u128);
impl_num_ext!(usize);
impl_num_ext!(i8);
impl_num_ext!(i16);
impl_num_ext!(i32);
impl_num_ext!(i64);
impl_num_ext!(i128);
impl_num_ext!(isize);
impl_num_ext!(f32);
impl_num_ext!(f64);
impl_num_ext!(Vec2);
impl_num_ext!(Pos2);

// ----------------------------------------------------------------------------

/// Wrap angle to `[-PI, PI]` range.
pub fn normalized_angle(mut angle: f32) -> f32 {
    use core::f32::consts::{PI, TAU};
    angle %= TAU;
    if angle > PI {
        angle -= TAU;
    } else if angle < -PI {
        angle += TAU;
    }
    angle
}

// ----------------------------------------------------------------------------

/// Calculate a lerp-factor for exponential smoothing using a time step.
///
/// * `exponential_smooth_factor(0.90, 1.0, dt)`: reach 90% in 1.0 seconds
/// * `exponential_smooth_factor(0.50, 0.2, dt)`: reach 50% in 0.2 seconds
///
/// Example:
/// ```
/// # use emath::{lerp, exponential_smooth_factor};
/// # let (mut smoothed_value, target_value, dt) = (0.0_f32, 1.0_f32, 0.01_f32);
/// let t = exponential_smooth_factor(0.90, 0.2, dt); // reach 90% in 0.2 seconds
/// smoothed_value = lerp(smoothed_value..=target_value, t);
/// ```
pub fn exponential_smooth_factor(
    reach_this_fraction: f32,
    in_this_many_seconds: f32,
    dt: f32,
) -> f32 {
    1.0 - (1.0 - reach_this_fraction).powf(dt / in_this_many_seconds)
}

/// If you have a value animating over time,
/// how much towards its target do you need to move it this frame?
///
/// You only need to store the start time and target value in order to animate using this function.
///
/// ``` rs
/// struct Animation {
///     current_value: f32,
///
///     animation_time_span: (f64, f64),
///     target_value: f32,
/// }
///
/// impl Animation {
///     fn update(&mut self, now: f64, dt: f32) {
///         let t = interpolation_factor(self.animation_time_span, now, dt, ease_in_ease_out);
///         self.current_value = emath::lerp(self.current_value..=self.target_value, t);
///     }
/// }
/// ```
pub fn interpolation_factor(
    (start_time, end_time): (f64, f64),
    current_time: f64,
    dt: f32,
    easing: impl Fn(f32) -> f32,
) -> f32 {
    let animation_duration = (end_time - start_time) as f32;
    let prev_time = current_time - dt as f64;
    let prev_t = easing((prev_time - start_time) as f32 / animation_duration);
    let end_t = easing((current_time - start_time) as f32 / animation_duration);
    if end_t < 1.0 {
        (end_t - prev_t) / (1.0 - prev_t)
    } else {
        1.0
    }
}

/// Ease in, ease out.
///
/// `f(0) = 0, f'(0) = 0, f(1) = 1, f'(1) = 0`.
#[inline]
pub fn ease_in_ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    (3.0 * t * t - 2.0 * t * t * t).clamp(0.0, 1.0)
}
