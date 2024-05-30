use super::vector::Vector;

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
K:  std::ops::Mul<Output = K> + Copy + Default + std::ops::Add<Output = K>, 
{
    assert_eq!(u.len(), coefs.len(), "Both Vector and Coefs must have same size!");
    let len = u[0].data.len();
    // let len = u[0].size();
    let mut result: Vector<K> = Vector {data:vec![K::default(); len]};

    // for i in 0..len() {
    //     for j in 0..u.data[0].len() {
    //         result.sfma(u[i][j].clone(), coefs[i].clone());
    //     }
    // }
    // result

    for n in 0..len {
        for (index, item) in u.into_iter().enumerate() {
            result.data[n] = result.data[n] + item.data[n] * coefs[index];
        }
    }
    result

    // for n in 0..len {
    //     let mut cont = Vec::new();
    //     for (index, item) in u.into_iter().enumerate() {
            
    //         result.data[n] += item.data[n] * coefs[index];
    //         cont.push(num);
    //     }
        // ret.push(cont.iter().sum())
    // }
    // Vector { data: ret }
}

// fn main() {
//     let e1 = Vector::from(&[1., 0., 0.]);
//     let e2 = Vector::from(&[0., 1., 0.]);
//     let e3 = Vector::from(&[0., 0., 1.]);
//     let v1 = Vector::from(&[1., 2., 3.]);
//     let v2 = Vector::from(&[0., 10., -100.]);
//     println!("{}", linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]));
//     // [10.]
//     // [-2.]
//     // [0.5]
//     println!("{}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
//     // [10.]
//     // [0.]
//     // [230.]
// }