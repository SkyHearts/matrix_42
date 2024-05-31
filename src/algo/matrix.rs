use std::fmt;
use crate::algo::{vector::Vector, linear_interpolation::Lerp, utils::Utils};
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
// use num::Zero;

#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct Matrix<K> {
    data: Vec<Vec<K>>,
}

impl<K> Matrix<K>
where 
K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Div<Output = K> + Neg<Output = K> + Copy + Clone + Default + PartialEq + Utils,
{
    pub fn add(&mut self, v: &Matrix<K>) {
        for (index1, element1) in self.data.iter_mut().enumerate() {
            for (index2, element2) in element1.iter_mut().enumerate() {
                *element2 = *element2 + v.data[index1][index2];
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        for (index1, element1) in self.data.iter_mut().enumerate() {
            for (index2, element2) in element1.iter_mut().enumerate() {
                *element2 = *element2 - v.data[index1][index2];
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for element1 in self.data.iter_mut() {
            for element2 in element1.iter_mut() {
                *element2 = *element2 * a;
            }
        }
    }

    pub fn mul_vec(&mut self, vec: &Vector<K>) -> Vector<K> {
        let mut result: Vector<K> = Vector {data:vec![K::default(); vec.size()]};
        for (index1, element1) in self.data.clone().into_iter().enumerate()  {
            for (index2, element2) in element1.into_iter().enumerate() {
                result[index1] = result[index1] + element2 * vec[index2];
            }
        }
        result
    }
    pub fn mul_mat(&mut self, mat: &Matrix<K>) -> Matrix<K> {
        let mut result: Matrix<K> = Matrix::new(self.shape().0, self.shape().1);
        for i in 0..self.shape().0 {
            for j in 0..mat.shape().1 {
                for k in 0..self.shape().1 {
                    result.data[i][j] = result.data[i][j] + self.data[i][k].clone() * mat.data[k][j];
                }
            }
        }
        result
    }

    pub fn trace(&mut self) -> K {
        assert_eq!(self.shape().0, self.shape().1, "Must be square matrix");
        let mut result = K::default();

        for i in 0..self.shape().0 {
            result = result + self[(i, i)].clone();
        }
        result
    }

    pub fn transpose(&mut self) -> Matrix<K> {
        
        let mut result: Matrix<K> = Matrix::new(self.shape().1, self.shape().0);

        for i in 0..self.shape().1 {
            for j in 0..self.shape().0 {
                result[(i,j)] = self[(j,i)].clone();
            }
        }
        result
    }

    pub fn row_echelon(&mut self) -> Matrix<K> {
        let mut matrix = self.data.clone();
        let rows = matrix.len();
        if rows == 0 {
            return Matrix { data: matrix };
        }
        let cols = matrix[0].len();

        let mut lead = 0;
        for r in 0..rows {
            if lead >= cols {
                break;
            }

            let mut i = r;
            while matrix[i][lead] == K::default() {
                i += 1;
                if i == rows {
                    i = r;
                    lead += 1;
                    if lead == cols {
                        return Matrix { data: matrix };
                    }
                }
            }

            matrix.swap(i, r);

            let lead_value = matrix[r][lead];
            for j in 0..cols {
                matrix[r][j] = matrix[r][j] / lead_value;
            }

            for i in 0..rows {
                if i != r {
                    let lead_value = matrix[i][lead];
                    for j in 0..cols {
                        matrix[i][j] = matrix[i][j] - lead_value * matrix[r][j];
                    }
                }
            }

            lead += 1;
        }

        Matrix { data: matrix }
    }

    pub fn determinant(&mut self) -> K {
        assert_eq!(self.shape().0, self.shape().1, "Must be square matrix");

        if self.shape() == (1, 1) {
            return self[0][0]
        }
        if self.shape() == (2, 2) {
            return (self[0][0] * self[1][1]) - (self[0][1] * self[1][0])
        }

        let (rows, cols) = self.shape();
        let mut sign = K::one();
        let mut matrix = self.clone();
        for k in 0..(rows - 1) {
            // Pivot row swap if needed
            if matrix[k][k] == K::zero() {
                let mut m = k + 1;
                while m < rows {
                    if matrix[m][k] != K::zero() {
                        matrix.data.swap(m, k);
                        sign = -sign;
                        break;
                    }
                    m += 1;
                }
                if m == rows {
                    return K::zero();
                }
            }

            // Formula
            for i in (k + 1)..rows {
                for j in (k + 1)..cols {
                    matrix[i][j] = matrix[k][k] * matrix[i][j] - matrix[i][k] * matrix[k][j];
                    if k != 0 {
                        matrix[i][j] = matrix[i][j] / matrix[k - 1][k - 1];
                    }
                }
            }
        }
        sign * matrix[rows - 1][rows - 1]
    }

    pub fn cofactor(&mut self) -> Matrix<K> {
        assert_eq!(self.shape().0, self.shape().1, "Must be square matrix");
        let mut cofactor= Matrix::new(self.shape().0 , self.shape().1);
        let mut subvec: Matrix<K> = Matrix::new(self.shape().0 - 1, self.shape().1 - 1);

        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                let mut p = 0;
                for x in 0..self.shape().0{
                    if x == i {
                        continue;
                    }
                    let mut q = 0;
                    for y in 0..self.shape().0{
                        if y == j {
                            continue;
                        }
                        subvec[p][q] = self[x][y];
                        q = q + 1;
                    }
                    p = p + 1;
                }
                // println!("i + j: {}\n",i + j);
                if (i + j) % 2 == 0 {
                    cofactor[i][j] = subvec.determinant();
                } else {
                    cofactor[i][j] = -subvec.determinant();
                }
                // cofactor[i][j] = (-K::one()).power(i + j) * subvec.determinant();
            }
        }
        cofactor
    }

    // pub fn inverse(&mut self) -> Result<Matrix<K>, MatrixError> {
    //     assert_eq!(self.shape().0, self.shape().1, "Must be square matrix");
    //     let det = K::one() / self.determinant();
    //     // assert_eq!(det, K::zero(), "Determinant is 0");
    //     // let mut inverse = Matrix::new(self.shape().0, self.shape().1);
    //     let mut inverse = self.clone().cofactor().transpose();
    //     for i in 0..self.shape().0 {
    //         for j in 0..self.shape().1 {
    //             inverse[i][j] = inverse[i][j] * det;
    //         }
    //     }
        
    //     Ok(inverse)
    // }

    pub fn inverse(&mut self) -> Matrix<K> {
        assert_eq!(self.shape().0, self.shape().1, "Must be square matrix");
        let det = self.determinant();
        // assert_eq!(det, K::zero(), "Determinant is 0");
        // let mut inverse = Matrix::new(self.shape().0, self.shape().1);
        let mut inverse = self.clone().cofactor().transpose();
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                inverse[i][j] = inverse[i][j] / det;
            }
        }
        inverse
    }

    pub fn rank(&mut self) -> usize{
        let mut count = 0;
        let echelon_form = self.clone().row_echelon();
        for i in 0..self.shape().0 {
            for j in 0..self.shape().1 {
                if echelon_form[i][j] != K::zero() {
                    count = count + 1;
                    // println!("Row {}:{:?}", i, self[i][j]);
                    break;
                }
            }
        }
        count
    }
}

impl<K> Matrix<K>
where 
K: Default + Clone,
{
    pub fn from(arr: &[&[K]]) -> Self {
        let mut ret_vec = Vec::new();
        for vec in arr.iter() {
            ret_vec.push(vec.to_vec());
        };
        Matrix { data: ret_vec }
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.data.len(), self.data[0].len())
    }

    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data: Vec<Vec<K>> = Vec::with_capacity(rows);
        for _ in 0..rows {
          data.push(vec![K::default(); cols]);
        }
        Matrix { data }
    }

    // pub fn zero() -> Self {
    //     Matrix { data: K::zero() }
    // }
}

// Implement the Add trait for Vector<K>
impl<K> Add for Matrix<K>
where
    K: Add<Output = K> + Clone,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        let mut result = self.clone();

        for (index1, element1) in result.data.iter_mut().enumerate() {
            for (index2, element2) in element1.iter_mut().enumerate() {
                *element2 = element2.clone() + other.data[index1][index2].clone();
            }
        }
        result
    }
}

// Implement the Sub trait for Vector<K>
impl<K> Sub for Matrix<K>
where
    K: Sub<Output = K> + Clone,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        let mut result = self.clone();

        for (index1, element1) in result.data.iter_mut().enumerate() {
            for (index2, element2) in element1.iter_mut().enumerate() {
                *element2 = element2.clone() - other.data[index1][index2].clone();
            }
        }
        result
    }
}

// Implement the Mul trait for Vector<K> with f32
impl<K> Mul<f32> for Matrix<K>
where
K: Mul<f32, Output = K> + Clone,
{
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        let mut result = self.clone();

        for element1 in result.data.iter_mut() {
            for element2 in element1.iter_mut() {
                *element2 = element2.clone() * scalar;
            }
        }
        result
    }
}

impl<K> Lerp for Matrix<K>
where 
K: Add<Output = K> + Sub<Output = K> + Mul<f32, Output = K> + Copy + Clone + Default,
{
    fn lerp(u: Matrix<K>, v: Matrix<K>, t: f32) -> Matrix<K> {
        let mut result: Matrix<K> = Matrix::new(u.shape().0, u.shape().1);
        for index1 in 0..u.shape().0 {
            for index2 in 0..u.shape().1 {
            result[(index1, index2)] = u[(index1, index2)] +(v[(index1, index2)] - u[(index1, index2)]) * t
            }
        }
        result
    }
}

impl<K: std::fmt::Display + Utils + std::cmp::PartialEq<K>> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for vector in &self.data{
            write!(f, "[")?;
            let mut iter = vector.into_iter().peekable();
            while let Some(element) = iter.next() {
                let num = String::from(element.to_string());
                let mut max_precision = 6;
                if num.find('.').is_none() {
                    max_precision = 1;
                }
                else {
                    let precision = num.split('.').last().unwrap().len();
                    if precision < max_precision {
                        max_precision = precision;
                    }
                }
                if iter.peek().is_none() {
                    if *element == K::zero(){
                        write!(f, "{:.1}", K::zero())?;
                    } else {
                        write!(f, "{:.max$}", element, max=max_precision)?;
                    }
                }
                else {
                    if *element == K::zero(){
                        write!(f, "{:.1}, ", K::zero())?;
                    } else {
                        write!(f, "{:.max$}, ", element, max=max_precision)?;
                    }
                }
            }
            write!(f, "]\n")?;
        }
        Ok(())
    }
}

// [] overloading / indexing
impl<K> Index<(usize, usize)> for Matrix<K> {
    type Output = K;

    fn index(&self, index: (usize, usize)) -> &K {
        &self.data[index.0][index.1]
    }
}

impl<K> IndexMut<(usize, usize)> for Matrix<K> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut K {
        &mut self.data[index.0][index.1]
    }
}

impl<K> Index<usize> for Matrix<K> {
    type Output = Vec<K>;

    fn index(&self, index: usize) -> &Vec<K> {
        &self.data[index]
    }
}

impl<K> IndexMut<usize> for Matrix<K> {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}