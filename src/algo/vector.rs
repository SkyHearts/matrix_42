use std::fmt;
use std::ops::{Add, Sub, Mul, Div, Index, IndexMut};
use super::linear_interpolation::Lerp;
use crate::algo::utils::Utils;

#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Vector<K> {
    pub data: Vec<K>,
}

// Vector<K> methods implementations
impl<K> Vector<K> 
where 
K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Copy + Clone + Default + Utils + PartialOrd,
{
    pub fn add(&mut self, v: &Vector<K>) {
        for (index, element) in self.data.iter_mut().enumerate() {
            *element = *element + v.data[index];
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        // for (index, element) in self.data.iter_mut().enumerate() {
        //     *element = *element - v.data[index];
        // }
        for index in 0..self.size() {
            // print!("{}", index);
            self[index] = self[index] - v[index];
            // print!("{:?}\n", self[index]);
        }
    }
    pub fn scl(&mut self, a: K) {
        for element in self.data.iter_mut() {
            *element = *element * a;
        }
    }

    pub fn dot(&self, v: Vector::<K>) -> K {
        let mut result = K::default();
        for (index, element) in self.clone().data.into_iter().enumerate() {
            result = result + (element * v[index]);
        }
        result
    }

    pub fn norm_1(&mut self) -> K {
        let mut result = K::default();
        for element in self.data.clone().into_iter() {
            result = result + element.absolute();
        }
        result
    }

    pub fn norm(&mut self) -> K {
        let mut result = K::default();
        for element in self.data.clone().into_iter() {
            result = result + element.square();
        }
        result.sroot()
    }

    pub fn norm_inf(&mut self) -> K {
        let mut result = K::default();
        for element in self.data.clone().into_iter() {
            if element.absolute() > result {
                result = element.absolute();
            }
        }
        result
    }

    pub fn angle_cos(u: &Vector<K>, v: &Vector<K>) -> K {
        (u.clone().dot(v.clone())) / (u.clone().norm() * v.clone().norm())
    }

    pub fn cross_product(u: &Vector<K>, v: &Vector<K>) -> Vector<K> {

        assert_eq!(u.size(), 3, "Not 3-Dimensional");
        assert_eq!(v.size(), 3, "Not 3-Dimensional");
        // let mut result: Vector<K> = Vector {data:vec![K::default(); u.size()]};
        // result
        Vector {data: vec![
            u.data[1] * v.data[2] - u.data[2] * v.data[1],
            u.data[2] * v.data[0] - u.data[0] * v.data[2],
            u.data[0] * v.data[1] - u.data[1] * v.data[0],
          ]}
    }
}


impl<K> Vector<K>
where 
    K: Default + Clone,
{
    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn from(arr: &[K]) -> Self {
        Vector { data: arr.to_vec() }
    }
}

// Implement the Add trait for Vector<K>
impl<K> Add for Vector<K>
where
    K: Add<Output = K> + Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        for (i, element) in result.data.iter_mut().enumerate() {
            *element = element.clone() + other.data[i].clone();
        }
        result
    }
}

// Implement the Sub trait for Vector<K>
impl<K> Sub for Vector<K>
where
    K: Sub<Output = K> + Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self.clone();
        for (i, element) in result.data.iter_mut().enumerate() {
            *element = element.clone() - other.data[i].clone();
        }
        result
    }
}

// Implement the Mul trait for Vector<K> with f32
impl<K> Mul<f32> for Vector<K>
where
    K: Mul<f32, Output = K> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();
        for element in result.data.iter_mut() {
            *element = element.clone() * scalar;
        }
        result
    }
}

impl<K> Lerp for Vector<K> 
where 
K: Add<Output = K> + Sub<Output = K> + Mul<f32, Output = K> + Copy + Clone + Default,
{
    fn lerp(u: Vector<K>, v: Vector<K>, t: f32) -> Vector<K>
    {
        let mut result: Vector<K> = Vector {data:vec![K::default(); u.size()]};
        for index in 0..u.size() {
            result[index] = u[index] +(v[index] - u[index]) * t
        }
        result
    }
}

impl<K: std::fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for element in &self.data{
            let mut max_precision = 6;
            let num = String::from(element.to_string());
            // if *element == K::zero(){
            //     write!(f, "[{:.1}]\n", K::zero())?;
            // }
            // write!(f, "[num :{}]\n", num.clone())?;
            if num.find('.').is_none() {
                max_precision = 1;
                // write!(f, "[{:.max_precision$}]\n", element)?;
            }
            else {
                let precision = num.split('.').last().unwrap().len();
                if precision < max_precision {
                    max_precision = precision;
                }
                // write!(f, "[{:.max_precision$}]\n", element)?;
            }
            write!(f, "[{:.max_precision$}]\n", element)?;
        }
        Ok(())
    }
}

// [] overloading / indexing
impl<K> Index<usize> for Vector<K> {
    type Output = K;

    fn index(&self, index: usize) -> &K {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Vector<K> {
    fn index_mut(&mut self, index: usize) -> &mut K {
        &mut self.data[index]
    }
}

// impl<K> Index<Range<usize>> for Vector<K> {
//     type Output = [K];

//     fn index(&self, range: Range<usize>) -> &Self::Output {
//         &self.data[range]
//     }
// }

// impl<K> IndexMut<Range<usize>> for Vector<K> {
//     fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
//         &mut self.data[range]
//     }
// }
