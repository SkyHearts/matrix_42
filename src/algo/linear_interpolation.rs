// use super::vector::Vector;

// #![allow(dead_code)]
use std::ops::{Add, Mul, Sub};

pub trait Lerp {
    fn lerp(u: Self, v: Self, t: f32) -> Self ;
}

impl Lerp for f32 {
    fn lerp(u: f32, v: f32, t: f32) -> f32 {
        u + (v - u) * t
    }
}

pub fn lerp<V>(u: V, v: V, t: f32) -> V 
where
V: Lerp + Clone + Add<Output = V> + Mul<f32, Output = V> + Sub<Output = V>
{
    u.clone() + (v - u) * t
}