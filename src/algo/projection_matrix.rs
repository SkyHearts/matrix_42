use crate::algo::matrix::Matrix;

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32> {
    let tan = 1. / (fov/2.).tan();
    Matrix::from(&[
        &[tan / ratio, 0., 0., 0.],
        &[0., tan, 0. ,0.],
        &[0., 0.,-1. * ((far + near) / (far - near)), -1. * (2. * (far * near) / (far - near))],
        &[0., 0., -1., 0.]
    ])
}