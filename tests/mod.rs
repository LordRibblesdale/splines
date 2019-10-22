use splines::{ Interpolate, Interpolation, Key, Spline};
use splines::interpolate::{Additive, Linear};
use std::ops::{Add, Div, Sub, Mul};

#[cfg(feature = "cgmath")] use cgmath as cg;
#[cfg(feature = "nalgebra")] use nalgebra as na;

// Small utility 2D-point.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct P2<T> {
  x: T,
  y: T
}

impl<T> Add for P2<T> where T: Add<T, Output = T> {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    P2 {
      x: self.x + rhs.x,
      y: self.y + rhs.y
    }
  }
}

impl<T> Sub for P2<T> where T: Sub<T, Output = T> {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    P2 {
      x: self.x - rhs.x,
      y: self.y - rhs.y
    }
  }
}

impl<T> Linear<T> for P2<T> where Self: Additive, T: Mul<T, Output = T> + Div<T, Output = T> {
  fn outer_mul(self, t: T) -> Self {
    P2 {
      x: self.x * t,
      y: self.y * t,
    }
  }

  fn outer_div(self, t: T) -> Self {
    P2 {
      x: self.x / t,
      y: self.y / t,
    }
  }
}

#[test]
fn step_interpolation_f32() {
  let start = Key::new(0., 0., Interpolation::Step(0.));
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::<f32, _>::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(10.));
  assert_eq!(spline.sample(0.1), Some(10.));
  assert_eq!(spline.sample(0.2), Some(10.));
  assert_eq!(spline.sample(0.5), Some(10.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
  assert_eq!(spline.sample_with_key(0.2), Some((10., &start, Some(&end))));
  assert_eq!(spline.clamped_sample_with_key(1.), Some((10., &end, None)));
}

#[test]
fn step_interpolation_f64() {
  let start = Key::new(0., 0., Interpolation::Step(0.));
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::<f64, _>::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(10.));
  assert_eq!(spline.sample(0.1), Some(10.));
  assert_eq!(spline.sample(0.2), Some(10.));
  assert_eq!(spline.sample(0.5), Some(10.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
  assert_eq!(spline.sample_with_key(0.2), Some((10., &start, Some(&end))));
  assert_eq!(spline.clamped_sample_with_key(1.), Some((10., &end, None)));
}

#[test]
fn step_interpolation_0_5() {
  let start = Key::new(0., 0., Interpolation::Step(0.5));
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(10.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
}

#[test]
fn step_interpolation_0_75() {
  let start = Key::new(0., 0., Interpolation::Step(0.75));
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(0.));
  assert_eq!(spline.sample(0.9), Some(10.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
}

#[test]
fn step_interpolation_1() {
  let start = Key::new(0., 0., Interpolation::Step(1.));
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(0.));
  assert_eq!(spline.sample(0.9), Some(0.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
}

#[test]
fn linear_interpolation() {
  let start = Key::new(0., 0., Interpolation::Linear);
  let end = Key::new(1., 10., Interpolation::default());
  let spline = Spline::from_vec(vec![start, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(1.));
  assert_eq!(spline.sample(0.2), Some(2.));
  assert_eq!(spline.sample(0.5), Some(5.));
  assert_eq!(spline.sample(0.9), Some(9.));
  assert_eq!(spline.sample(1.), None);
  assert_eq!(spline.clamped_sample(1.), Some(10.));
}

#[test]
fn linear_interpolation_several_keys() {
  let start = Key::new(0., 0., Interpolation::Linear);
  let k1 = Key::new(1., 5., Interpolation::Linear);
  let k2 = Key::new(2., 0., Interpolation::Linear);
  let k3 = Key::new(3., 1., Interpolation::Linear);
  let k4 = Key::new(10., 2., Interpolation::Linear);
  let end = Key::new(11., 4., Interpolation::default());
  let spline = Spline::from_vec(vec![start, k1, k2, k3, k4, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.5));
  assert_eq!(spline.sample(0.2), Some(1.));
  assert_eq!(spline.sample(0.5), Some(2.5));
  assert_eq!(spline.sample(0.9), Some(4.5));
  assert_eq!(spline.sample(1.), Some(5.));
  assert_eq!(spline.sample(1.5), Some(2.5));
  assert_eq!(spline.sample(2.), Some(0.));
  assert_eq!(spline.sample(2.75), Some(0.75));
  assert_eq!(spline.sample(3.), Some(1.));
  assert_eq!(spline.sample(6.5), Some(1.5));
  assert_eq!(spline.sample(10.), Some(2.));
  assert_eq!(spline.clamped_sample(11.), Some(4.));
}

#[test]
fn several_interpolations_several_keys() {
  let start = Key::new(0., 0., Interpolation::Step(0.5));
  let k1 = Key::new(1., 5., Interpolation::Linear);
  let k2 = Key::new(2., 0., Interpolation::Step(0.1));
  let k3 = Key::new(3., 1., Interpolation::Linear);
  let k4 = Key::new(10., 2., Interpolation::Linear);
  let end = Key::new(11., 4., Interpolation::default());
  let spline = Spline::from_vec(vec![start, k1, k2, k3, k4, end]);

  assert_eq!(spline.sample(0.), Some(0.));
  assert_eq!(spline.sample(0.1), Some(0.));
  assert_eq!(spline.sample(0.2), Some(0.));
  assert_eq!(spline.sample(0.5), Some(5.));
  assert_eq!(spline.sample(0.9), Some(5.));
  assert_eq!(spline.sample(1.), Some(5.));
  assert_eq!(spline.sample(1.5), Some(2.5));
  assert_eq!(spline.sample(2.), Some(0.));
  assert_eq!(spline.sample(2.05), Some(0.));
  assert_eq!(spline.sample(2.099), Some(0.));
  assert_eq!(spline.sample(2.75), Some(1.));
  assert_eq!(spline.sample(3.), Some(1.));
  assert_eq!(spline.sample(6.5), Some(1.5));
  assert_eq!(spline.sample(10.), Some(2.));
  assert_eq!(spline.clamped_sample(11.), Some(4.));
}

#[test]
fn stroke_bezier_straight() {
  let keys = vec![
    Key::new(0.0, [0., 1.], Interpolation::StrokeBezier([0., 1.], [0., 1.])),
    Key::new(5.0, [5., 1.], Interpolation::StrokeBezier([5., 1.], [5., 1.]))
  ];
  let spline = Spline::from_vec(keys);

  assert_eq!(spline.clamped_sample(0.0)[1], 1.);
  assert_eq!(spline.clamped_sample(1.0)[1], 1.);
  assert_eq!(spline.clamped_sample(2.0)[1], 1.);
  assert_eq!(spline.clamped_sample(3.0)[1], 1.);
  assert_eq!(spline.clamped_sample(4.0)[1], 1.);
  assert_eq!(spline.clamped_sample(5.0)[1], 1.);
}

#[cfg(feature = "cgmath")]
#[test]
fn cgmath_vector_interpolation() {
  use splines::Interpolate;

  let start = cg::Vector2::new(0.0, 0.0);
  let mid = cg::Vector2::new(0.5, 0.5);
  let end = cg::Vector2::new(1.0, 1.0);

  assert_eq!(Interpolate::lerp(start, end, 0.0), start);
  assert_eq!(Interpolate::lerp(start, end, 1.0), end);
  assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}

#[cfg(feature = "nalgebra")]
#[test]
fn nalgebra_vector_interpolation() {
  use splines::Interpolate;

  let start = na::Vector2::new(0.0, 0.0);
  let mid = na::Vector2::new(0.5, 0.5);
  let end = na::Vector2::new(1.0, 1.0);

  assert_eq!(Interpolate::lerp(start, end, 0.0), start);
  assert_eq!(Interpolate::lerp(start, end, 1.0), end);
  assert_eq!(Interpolate::lerp(start, end, 0.5), mid);
}

#[test]
fn add_key_empty() {
  let mut spline: Spline<f32, f32> = Spline::from_vec(vec![]);
  spline.add(Key::new(0., 0., Interpolation::Linear));

  assert_eq!(spline.keys(), &[Key::new(0., 0., Interpolation::Linear)]);
}

#[test]
fn add_key() {
  let start = Key::new(0., 0., Interpolation::Step(0.5));
  let k1 = Key::new(1., 5., Interpolation::Linear);
  let k2 = Key::new(2., 0., Interpolation::Step(0.1));
  let k3 = Key::new(3., 1., Interpolation::Linear);
  let k4 = Key::new(10., 2., Interpolation::Linear);
  let end = Key::new(11., 4., Interpolation::default());
  let new = Key::new(2.4, 40., Interpolation::Linear);
  let mut spline = Spline::from_vec(vec![start, k1, k2.clone(), k3, k4, end]);

  assert_eq!(spline.keys(), &[start, k1, k2, k3, k4, end]);
  spline.add(new);
  assert_eq!(spline.keys(), &[start, k1, k2, new, k3, k4, end]);
}

#[test]
fn remove_element_empty() {
  let mut spline: Spline<f32, f32> = Spline::from_vec(vec![]);
  let removed = spline.remove(0);

  assert_eq!(removed, None);
  assert!(spline.is_empty());
}

#[test]
fn remove_element() {
  let start = Key::new(0., 0., Interpolation::Step(0.5));
  let k1 = Key::new(1., 5., Interpolation::Linear);
  let k2 = Key::new(2., 0., Interpolation::Step(0.1));
  let k3 = Key::new(3., 1., Interpolation::Linear);
  let k4 = Key::new(10., 2., Interpolation::Linear);
  let end = Key::new(11., 4., Interpolation::default());
  let mut spline = Spline::from_vec(vec![start, k1, k2.clone(), k3, k4, end]);
  let removed = spline.remove(2);

  assert_eq!(removed, Some(k2));
  assert_eq!(spline.len(), 5);
}
