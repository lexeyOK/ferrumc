use bevy_math::DVec2;
use bevy_math::DVec3;
use bevy_math::FloatExt;
use bevy_math::Vec3Swizzles;

pub fn clamped_map(v: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    v.clamp(in_min, in_max)
        .remap(in_min, in_max, out_min, out_max)
}

pub fn lerp2(delta: DVec2, start1: f64, end1: f64, start2: f64, end2: f64) -> f64 {
    start1
        .lerp(end1, delta.x)
        .lerp(start2.lerp(end2, delta.x), delta.y)
}

pub fn lerp3(
    delta: (f64, f64, f64),
    start: (f64, f64, f64, f64),
    end: (f64, f64, f64, f64),
) -> f64 {
    fn lerp(start: f64, end: f64, t: f64) -> f64 {
        start + (end - start) * t
    }

    let l0 = lerp(start.0, end.0, delta.0);
    let l1 = lerp(start.1, end.1, delta.0);
    let l2 = lerp(start.2, end.2, delta.0);
    let l3 = lerp(start.3, end.3, delta.0);

    let ll0 = lerp(l0, l1, delta.1);
    let ll1 = lerp(l2, l3, delta.1);
    lerp(ll0, ll1, delta.2)
}

use std::arch::x86_64::*;
use std::simd::f64x4;

#[target_feature(enable = "avx,fma")]
pub fn lerp3_f64x4(
    delta: (f64, f64, f64),
    start: (f64, f64, f64, f64),
    end: (f64, f64, f64, f64),
) -> f64 {
    let start = _mm256_set_pd(start.0, start.1, start.2, start.3);
    let end = _mm256_set_pd(end.0, end.1, end.2, end.3);
    let t = _mm256_set1_pd(delta.0);
    let d = _mm256_sub_pd(end, start);
    let lerp: f64x4 = _mm256_fmadd_pd(d, t, start).into();
    let [l0, l1, l2, l3] = lerp.to_array();
    let ll0 = l0 + (l1 - l0) * delta.1;
    let ll1 = l2 + (l3 - l2) * delta.1;
    ll0 + (ll1 - ll0) * delta.2
}
