use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Neg};
use crate::algo::utils::Utils;
extern crate num;
use num::{Float, Zero, One};

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Complex<K>{
    pub real: K,
    pub imaginary: K,
}

impl<K> Complex<K>
where 
K: Clone,
{
    pub fn new(rl: K, im: K) -> Complex<K> {
        Complex {
            real: rl,
            imaginary: im,
        }
    }
}

// Implement the Add trait for Complex<K>
impl<K> Add for Complex<K>
where
K: Add<Output = K> + Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Complex {
            real: other.real + self.real,
            imaginary: other.imaginary + self.imaginary,
        }
    }
}

// Implement the Sub trait for Complex<K>
impl<K> Sub for Complex<K>
where
    K: Sub<Output = K> + Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Complex {
            real: (self.real - other.real),
            imaginary: (self.imaginary - other.imaginary),
        }
    }
}

// Implement the Mul trait for Complex<K>
impl<K> Mul for Complex<K>
where
K: Mul<Output = K> + Clone + Copy + Add<Output = K> + Sub<Output = K>,
{
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let rl = (self.real * other.real) - (self.imaginary * other.imaginary);
        let im = (self.real * other.imaginary) + (self.imaginary * other.real);

        Complex {
            real: rl,
            imaginary: im,
        }
    }
}

// Implement the Mul trait for Complex<K> with f32
impl<K> Mul<f32> for Complex<K>
where
K: Mul<f32, Output = K> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Complex {
            real: self.real * scalar,
            imaginary: self.imaginary * scalar,
        }
    }
}

impl<K> Div for Complex<K>
where
K: Mul<Output = K> + Add<Output = K> + Sub<Output = K> + Div<Output = K> + Clone + Copy,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Complex{
            real: (self.real * rhs.real + self.imaginary * rhs.imaginary) / (rhs.real * rhs.real + rhs.imaginary * rhs.imaginary),
            imaginary: (self.imaginary * rhs.real - self.real * rhs.imaginary) / (rhs.real * rhs.real + rhs.imaginary * rhs.imaginary),
        }
    }
}


// impl<K> Div<f32> for Complex<K>
// where
// K: Div<f32, Output = K> + Clone + Copy,
// {
//     type Output = Self;
//     fn div(self, scalar: f32) -> Self::Output {
//         Complex{
//             real: self.real / scalar,
//             imaginary: self.imaginary / scalar,
//         }
//     }
// }

// K: Neg<Output = K> + Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Clone + Copy + Utils,
impl<K> Neg for Complex<K>
where
K: Neg<Output = K> + Clone + Copy
{
	type Output = Self;

	fn neg(self) -> Self {
		Self::new(-self.real, -self.imaginary)
        // self * Complex::new(-K::one(), -K::zero())
	}
}

impl<K> Utils for Complex<K>
where
K: Utils + Float + Zero + One + Div<Output = K> + Add<Output = K> + Sub<Output = K> + Mul<Output = K>,
{
    fn absolute(&self) -> Self {
        Self::new(self.real.abs(), self.imaginary.abs())
    }

    fn square(&self) -> Self {
        let rl = (self.real * self.real) - (self.imaginary * self.imaginary);
        let im = (self.real * self.imaginary) + (self.imaginary * self.real);

        Complex {
            real: rl,
            imaginary: im,
        }
    }

    fn sroot(&self) -> Self {
        let r = (self.real * self.real + self.imaginary * self.imaginary).sqrt().sqrt();
		let theta = (self.imaginary / self.real).atan() / K::from(2.0).unwrap();
		if self.real < <K as Utils>::zero() {
			Self::new(r * theta.sin(), r * theta.cos())
		} else {
			Self::new(r * theta.cos(), r * theta.sin())
		}
    }

    fn one() -> Self {
        Self::new(<K as Utils>::one(), <K as Utils>::zero())
    }

    fn zero() -> Self {
        Self::new(<K as Utils>::zero(), <K as Utils>::zero())
    }
    
    // fn power(&self, n: usize) -> Self {
    //     let r = (self.real * self.real + self.imaginary * self.imaginary).sqrt().powi(n as i32);
	// 	let theta = (self.imaginary / self.real).atan() * K::from(n).unwrap();
	// 	Self::new(r * theta.cos(), r * theta.sin())
    // }
}

impl<K: std::fmt::Display + Utils + std::cmp::PartialOrd> fmt::Display for Complex<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sign: String = String::new();
        if self.imaginary >= K::zero() {
            sign += "+";
        }
        let real = String::from(self.real.to_string());
        let imaginary = String::from(self.imaginary.to_string());
        let mut rl_max_precision = 6;
        let mut im_max_precision = 6;
        if real.find('.').is_none() {
            rl_max_precision = 1;
        }
        else {
            let precision = real.split('.').last().unwrap().len();
            if precision < rl_max_precision {
                rl_max_precision = precision;
            }
        }
        if imaginary.find('.').is_none() {
            im_max_precision = 1;
        }
        else {
            let precision = imaginary.split('.').last().unwrap().len();
            if precision < im_max_precision {
                im_max_precision = precision;
            }
        }
        write!(f, "[{:.rl_max_precision$}{}{:.im_max_precision$}i]", self.real, sign, self.imaginary)?;
        // write!(f, "[{:.1}{}{:.1}i]", self.real, sign, self.imaginary)?;
        Ok(())
    }
}
