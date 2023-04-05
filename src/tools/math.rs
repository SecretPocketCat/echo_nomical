use std::ops::{Add, Mul};

pub fn inverse_lerp(a: f32, b: f32, t: f32) -> f32 {
    (t - a) / (b - a)
}

pub fn inverse_lerp_clamped(a: f32, b: f32, t: f32) -> f32 {
    inverse_lerp(a, b, t).clamp(0., 1.)
}

pub fn asymptotic_smoothing<
    T: Mul<f32> + From<<T as Mul<f32>>::Output> + Add<T> + From<<T as Add<T>>::Output>,
>(
    val: T,
    target: T,
    t: f32,
) -> T {
    T::from(T::from(val * (1.0 - t)) + T::from(target * t))
}

pub fn asymptotic_smoothing_with_delta_time<
    T: Mul<f32> + From<<T as Mul<f32>>::Output> + Add<T> + From<<T as Add<T>>::Output>,
>(
    val: T,
    target: T,
    t: f32,
    delta_time: f32,
) -> T {
    let t = t * 60. * delta_time;
    asymptotic_smoothing(val, target, t)
}
