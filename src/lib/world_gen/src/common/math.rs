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

#[allow(clippy::too_many_arguments)]
pub fn lerp3(
    delta: DVec3,
    start1: f64,
    end1: f64,
    start2: f64,
    end2: f64,
    start3: f64,
    end3: f64,
    start4: f64,
    end4: f64,
) -> f64 {
    lerp2(delta.xy(), start1, end1, start2, end2)
        .lerp(lerp2(delta.xy(), start3, end3, start4, end4), delta.z)
}

use std::arch::x86_64::*;
use std::simd::f32x4;

#[target_feature(enable = "sse,fma")]
pub fn lerp3_f32x4(delta: (f32, f32, f32), start: f32x4, end: f32x4) -> f32 {
    let start = start.into();
    let end = end.into();
    let t = _mm_set1_ps(delta.0);
    let half_lerp = _mm_fnmadd_ps(start, t, start);
    let lerp: f32x4 = _mm_fmadd_ps(end, t, half_lerp).into();
    let [l0, l1, l2, l3] = lerp.to_array();
    let ll0 = l3 + (l2 - l3) * delta.1;
    let ll1 = l1 + (l0 - l1) * delta.1;
    ll1 + (ll0 - ll1) * delta.2
}
