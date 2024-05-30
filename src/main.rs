mod algo;
use algo::{matrix::Matrix, vector::Vector, linear_combination::linear_combination, linear_interpolation::{lerp, Lerp}};
// use vector::{vector::Vector, linear_combination::linear_combination};
use algo::projection_matrix::projection;
use algo::complex::Complex;

fn main() {

    // let mut _v: Vector<i32> = Vector::<i32> {
    //     data: vec![1,2,3],
    // };

    // Ex00
    println!("--------Ex00--------\n");
    let mut v = Vector::from(&[2., 3., 4.]);

    let u = Vector::from(&[1., 2., 3.]);

    let m = Vector::from(&[1., 1., 1.]);

    let k: f32 = 2.;
    println!("Original is\n{}", v);
    v.add(&u);
    println!("Addition with\n{} is\n{}", u, v);
    v.sub(&m);
    println!("Subtraction with\n{} is\n{}", m, v);
    v.scl(k);
    println!("Scale with {} is\n{}", k, v);

    // let mut mat1 = Matrix::<f32> {
        // data: vec![vec![1.,2.,3.], vec![4.,5.,6.]],
    // };
    
    let mut mat1 = Matrix::from(&[&[1.,2.,3.], &[4.,5.,6.]]);

    let mat2 = Matrix::from(&[&[1.,2.,3.], &[4.,5.,6.]]);

    let mat3 = Matrix::from(&[&[1.,1.,1.], &[1.,1.,1.]]);

    println!("Original is\n{}", mat1);
    mat1.add(&mat2);
    println!("Addition with\n{} is\n{}", mat2, mat1);
    mat1.sub(&mat3);
    println!("Subtraction with\n{} is\n{}", mat3, mat1);
    mat1.scl(k);
    println!("Scale with {} is\n{}", k, mat1);

    // Ex01
    println!("--------Ex01--------\n");
    let e1 = Vector::from(&[1., 0., 0.]);
    let e2 = Vector::from(&[0., 1., 0.]);
    let e3 = Vector::from(&[0., 0., 1.]);
    let v1 = Vector::from(&[1., 2., 3.]);
    let v2 = Vector::from(&[0., 10., -100.]);
    println!("{}", linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]));
    // [10.]
    // [-2.]
    // [0.5]
    println!("{}", linear_combination::<f32>(&[v1, v2], &[10., -2.]));
    // [10.]
    // [0.]
    // [230.]

    // Ex02
    println!("--------Ex02--------\n");
    println!("{}", lerp(0., 1., 0.));
    // 0.0
    println!("{}", lerp(0., 1., 1.));
    // 1.0
    println!("{}", lerp(0., 1., 0.5));
    // 0.5
    println!("{}\n", lerp(21., 42., 0.3));
    // 27.3

    println!("{}", lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3));
    // [2.6]
    // [1.3]
    println!("{}", Vector::lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3));
    // [2.6]
    // [1.3]

    println!("{}", lerp(Matrix::from(&[&[2., 1.], &[3., 4.]]), Matrix::from(&[&[20.,10.], &[30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]

    println!("{}", Matrix::lerp(Matrix::from(&[&[2., 1.], &[3., 4.]]), Matrix::from(&[&[20.,10.], &[30., 40.]]), 0.5));
    // [[11., 5.5]
    // [16.5, 22.]]

    // Ex03
    println!("--------Ex03--------\n");
    let u = Vector::from(&[0., 0.]);
    let v = Vector::from(&[1., 1.]);
    println!("{}", u.dot(v));
    // 0.0
    let u = Vector::from(&[1., 1.]);
    let v = Vector::from(&[1., 1.]);
    println!("{}", u.dot(v));
    // 2.0
    let u = Vector::from(&[-1., 6.]);
    let v = Vector::from(&[3., 2.]);
    println!("{}\n", u.dot(v));
    // 9.0

    // Ex04
    println!("--------Ex04--------\n");
    let mut u = Vector::from(&[0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 0.0, 0.0, 0.0
    let mut u = Vector::from(&[1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
    // 6.0, 3.74165738, 3.0
    let mut u = Vector::from(&[-1., -2.]);
    println!("{}, {}, {}\n", u.norm_1(), u.norm(), u.norm_inf());
    // 3.0, 2.236067977, 2.0

    // Ex05
    println!("--------Ex05--------\n");
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[1., 0.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(&[1., 0.]);
    let v = Vector::from(&[0., 1.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 0.0
    let u = Vector::from(&[-1., 1.]);
    let v = Vector::from(&[ 1., -1.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // -1.0
    let u = Vector::from(&[2., 1.]);
    let v = Vector::from(&[4., 2.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 1.0
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    println!("{}", Vector::angle_cos(&u, &v));
    // 0.974631846

    // Ex06
    println!("--------Ex06--------\n");
    let u = Vector::from(&[0., 0., 1.]);
    let v = Vector::from(&[1., 0., 0.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [0.]
    // [1.]
    // [0.]
    let u = Vector::from(&[1., 2., 3.]);
    let v = Vector::from(&[4., 5., 6.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [-3.]
    // [6.]
    // [-3.]
    let u = Vector::from(&[4., 2., -3.]);
    let v = Vector::from(&[-2., -5., 16.]);
    println!("{}", Vector::cross_product(&u, &v));
    // [17.]
    // [-58.]
    // [-16.]

    // Ex07
    println!("--------Ex07--------\n");
    let mut u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}", u.mul_vec(&v));
        // [4.]
        // [2.]
        let mut u = Matrix::from(&[
        &[2., 0.],
        &[0., 2.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}", u.mul_vec(&v));
        // [8.]
        // [4.]
        let mut u = Matrix::from(&[
        &[2., -2.],
        &[-2., 2.],
        ]);
        let v = Vector::from(&[4., 2.]);
        println!("{}\n", u.mul_vec(&v));
        // [4.]
        // [-4.]

        let mut u = Matrix::from(&[
            &[1., 0.],
            &[0., 1.],
            ]);
        let v = Matrix::from(&[
            &[1., 0.],
            &[0., 1.],
            ]);
        println!("{}", u.mul_mat(&v));
        // [1., 0.]
        // [0., 1.]
        let mut u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]);
        let v = Matrix::from(&[
        &[2., 1.],
        &[4., 2.],
        ]);
        println!("{}", u.mul_mat(&v));
        // [2., 1.]
        // [4., 2.]
        let mut u = Matrix::from(&[
        &[3., -5.],
        &[6., 8.],
        ]);
        let v = Matrix::from(&[
        &[2., 1.],
        &[4., 2.],
        ]);
        println!("{}", u.mul_mat(&v));
        // [-14., -7.]
        // [44., 22.]

    // Ex08
    println!("--------Ex08--------\n");
    let mut u = Matrix::from(&[
        &[1., 0.],
        &[0., 1.],
        ]);
    println!("{}", u.trace());
    // 2.0
    let  mut u = Matrix::from(&[
        &[2., -5., 0.],
        &[4., 3., 7.],
        &[-2., 3., 4.],
        ]);
    println!("{}", u.trace());
    // 9.0
    let mut u = Matrix::from(&[
        &[-2., -8., 4.],
        &[1., -23., 4.],
        &[0., 6., 4.],
        ]);
    println!("{}\n", u.trace());
    // -21.0

    // Ex09
    println!("--------Ex09--------\n");
    let mut u = Matrix::from(&[
        &[-2., -8., 4.],
        &[1., -23., 4.],
        &[0., 6., 4.],
        ]);
    println!("{}\n", u.transpose());
    // [-2.0, 1.0, 0.0]
    // [-8.0, -23.0, 6.0]
    // [4.0, 4.0, 4.0]

    let mut u = Matrix::from(&[
        &[-2., -8., 4.],
        &[1., -23., 4.],
        ]);
    println!("{}\n", u.transpose());
    // [-2.0, 1.0]
    // [-8.0, -23.0]
    // [4.0, 4.0]

    println!("{}\n", u.transpose().transpose());
    // [-2.0, -8.0, 4.0]
    // [1.0, -23.0, 4.0]

    // Ex10
    println!("--------Ex10--------\n");
    let mut u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from(&[
        &[1., 2.],
        &[3., 4.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0]
    // [0.0, 1.0]
    let mut u = Matrix::from(&[
        &[1., 2.],
        &[2., 4.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 2.0]
    // [0.0, 0.0]
    let mut u = Matrix::from(&[
        &[8., 5., -2., 4., 28.],
        &[4., 2.5, 20., 4., -4.],
        &[8., 5., 1., 4., 17.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.625, 0.0, 0.0, -12.1666667]
    // [0.0, 0.0, 1.0, 0.0, -3.6666667]
    // [0.0, 0.0, 0.0, 1.0, 29.5 ]
    let mut u = Matrix::from(&[
        &[0., 3., -6., 6., 4., -5.],
        &[3., -7., 8., -5., 8., 9.],
        &[3., -9., 12., -9., 6., 15.],
        ]);
    println!("{}", u.row_echelon());
    // [1.0, 0.0, -2.0, 3.0, 0.0, -24.0]
    // [0.0, 1.0, -2.0, 2.0, 0.0, -7.0]
    // [0.0, 0.0, 0.0, 0.0, 1.0, 4.0]
    let mut u = Matrix::from(&[
        &[ 8., 5., -2., 4.],
        &[ 4., 2.5, 20., 4.],
        &[ 8., 5., 1., 4.],
        &[28., -4., 17., 1.],
        ]);
    // [1.0, 0.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0, 0.0]
    // [0.0, 0.0, 1.0, 0.0]
    // [0.0, 0.0, 0.0, 1.0]
    println!("{}\n", u.row_echelon());
    let mut u = Matrix::from(&[
        &[-2., -5., 2., 7.],
        &[5., -23., -4., 8.],
        &[1., 6., 17., -9.],
        ]);
    println!("{}\n", u.row_echelon());
    // [1.0, 0.0, 0.0, -1.85759]
    // [0.0, 1.0, 0.0, -0.72296]
    // [0.0, 0.0, 1.0, -0.16498]

    // Ex11
    println!("--------Ex11--------\n");
    let mut u = Matrix::from(&[
        &[ 1., -1.],
        &[-1., 1.],
        ]);
    println!("{}", u.determinant());
    // 0.0
    let mut u = Matrix::from(&[
        &[ 3., -1.],
        &[4., 3.],
        ]);
    println!("{}", u.determinant());
    // 13.0
    let mut u = Matrix::from(&[
        &[2., 0., 0.],
        &[0., 2., 0.],
        &[0., 0., 2.],
        ]);
    println!("{}", u.determinant());
    // 8.0
    let mut u = Matrix::from(&[
        &[8., 5., -2.],
        &[4., 7., 20.],
        &[7., 6., 1.],
        ]);
    println!("{}", u.determinant());
    // -174.0
    let mut u = Matrix::from(&[
        &[ 8., 5., -2., 4.],
        &[ 4., 2.5, 20., 4.],
        &[ 8., 5., 1., 4.],
        &[28., -4., 17., 1.],
        ]);
    println!("{}\n", u.determinant());
    // 1032

    // Ex12
    println!("--------Ex12--------\n");
    let mut u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
        ]);
    println!("{}", u.inverse());
    // [1.0, 0.0, 0.0]
    // [0.0, 1.0, 0.0]
    // [0.0, 0.0, 1.0]
    let mut u = Matrix::from(&[
        &[2., 0., 0.],
        &[0., 2., 0.],
        &[0., 0., 2.],
        ]);
    println!("{}", u.inverse());
    // [0.5, 0.0, 0.0]
    // [0.0, 0.5, 0.0]
    // [0.0, 0.0, 0.5]
    let mut u = Matrix::from(&[
        &[1., 2., 3.],
        &[4., 5., 6.],
        &[7., 8., 9.],
        ]);
    println!("{}", u.cofactor());
    // [-3.0, 6.0, -3.0]
    // [6.0, -12.0, 6.0]
    // [-3.0, 6.0, -3.0]
    let mut u = Matrix::from(&[
        &[8., 5., -2.],
        &[4., 7., 20.],
        &[7., 6., 1.],
        ]);
    println!("{}\n", u.inverse());
    // [0.649425287, 0.097701149, -0.655172414]
    // [-0.781609195, -0.126436782, 0.965517241]
    // [0.143678161, 0.074712644, -0.206896552]

    // Ex13
    println!("--------Ex13--------\n");
    let mut u = Matrix::from(&[
        &[1., 0., 0.],
        &[0., 1., 0.],
        &[0., 0., 1.],
        ]);
    println!("{}", u.rank());
    // 3
    let mut u = Matrix::from(&[
        &[ 1., 2., 0., 0.],
        &[ 2., 4., 0., 0.],
        &[-1., 2., 1., 1.],
        ]);
    println!("{}", u.rank());
    // 2
    let mut u = Matrix::from(&[
        &[ 8., 5., -2.],
        &[ 4., 7., 20.],
        &[ 7., 6., 1.],
        &[21., 18., 7.],
        ]);
    println!("{}", u.rank());
    // 3

    // Ex14
    println!("--------Ex14--------\n");
    println!("{}", projection(0.9, 1., 1., 10.));

    // Ex15
    println!("--------Ex15--------\n");
    println!("-----Add,Sub,Scale------\n");
    let mut a = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(3., 4.)],
        &[Complex::new(5., 6.), Complex::new(7., 8.)]
    ]);

    let b = Matrix::from(&[
        &[Complex::new(9., 10.), Complex::new(11., 12.)],
        &[Complex::new(13., 14.), Complex::new(15., 16.)]
    ]);

    let c = Matrix::from(&[
        &[Complex::new(2., 2.), Complex::new(2., 2.)],
        &[Complex::new(2., 2.), Complex::new(2., 26.)]
    ]);
    a.add(&b);
    println!("{}", a);
    // [[10.0+12.0i], [14.0+16.0i]]
    // [[18.0+20.0i], [22.0+24.0i]]
    a.sub(&c);
    println!("{}", a);
    // [[8.0+10.0i], [12.0+14.0i]]
    // [[16.0+18.0i], [20.0-2.0i]]
    a.scl(Complex::new(2., 3.));
    println!("{}", a);
    // [[-14.0+44.0i], [-18.0+64.0i]]
    // [[-22.0+84.0i], [46.0+56.0i]]

    println!("----------Linear Combination----------\n");
    let e1 = Vector::from(&[Complex::new(1., 0.), Complex::new(0., 0.), Complex::new(0., 0.)]);
    let e2 = Vector::from(&[Complex::new(0., 0.), Complex::new(1., 0.), Complex::new(0., 0.)]);
    let e3 = Vector::from(&[Complex::new(0., 0.), Complex::new(0., 0.), Complex::new(1., 0.)]);
    let v1 = Vector::from(&[Complex::new(1., 2.), Complex::new(3., 4.), Complex::new(5., 6.)]);
    let v2 = Vector::from(&[Complex::new(0., 10.), Complex::new(-100., 0.), Complex::new(0., 0.)]);
    let test1 = linear_combination::<Complex<f32>>(&[e1, e2, e3], &[Complex::new(10., 0.), Complex::new(-2., 0.), Complex::new(0.5, 0.)]);
    let test2 = linear_combination(&[v1.clone(), v2.clone()], &[Complex::new(10., 0.), Complex::new(-2., 0.)]);
    let test3 = linear_combination(&[v1, v2], &[Complex::new(10., 4.), Complex::new(-2., -8.)]);
    
    println!("{}", test1);
    println!("{}", test2);
    println!("{}", test3);

    // [[10.0+0.0i]]
    // [[-2.0+0.0i]]
    // [[0.5+0.0i]]

    // [[10.0+0.0i]]
    // [[230.0+40.0i]]
    // [[50.0+60.0i]]

    // [[82.0+4.0i]]
    // [[214.0+852.0i]]
    // [[26.0+80.0i]]
    println!("----------Lerp----------\n");
    let u = Vector::from(&[Complex::new(1., 0.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(0., 1.), Complex::new(1., 0.)]);
    // let t = Complex::new(0.5, 0.);
    // let res = Vector::from(&[Complex::new(0.5, 0.5), Complex::new(0.5, 0.5)]);
    println!("{}\n", lerp(u, v, 0.5));
    // println!("{}", res);
    // [[0.5+0.5i]]
    // [[0.5+0.5i]]

    println!("----------Dot product----------\n");

    let u = Vector::from(&[Complex::new(0., 1.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(1., 1.)]);
    println!("{}\n", u.dot(v));
    // [-2.0+2.0i]

    println!("----------Norm----------\n");

    let mut u = Vector::from(&[Complex::new(0., 1.), Complex::new(0., 1.), Complex::new(0., 1.)]);
    println!("{}", u.norm());
    println!("{}", u.norm_1());
    println!("{}\n", u.norm_inf());
    // [0., 1.732050]
    // [0., 3.]
    // [0., 1]

    println!("---------Cos theta-----------\n");

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.)]);
    let v = Vector::from(&[Complex::new(1., 1.), Complex::new(0., 1.)]);
    println!("{}",Vector::angle_cos(&u, &v));
    // [-0.6, -0.8]

    let u = Vector::from(&[Complex::new(1., 1.), Complex::new(2., 1.), Complex::new(3., 1.)]);
    let v = Vector::from(&[Complex::new(4., 1.), Complex::new(5., 1.), Complex::new(6., 1.)]);
    println!("{}\n", Vector::angle_cos(&u, &v));
    // [0.99291223, 0.01960226]

    // if -0. < 0. {
    //     println!("Less than\n")
    // } else {
    //     println!("Equal\n")
    // }

    println!("---------Cross Product-----------\n");

    let u = Vector::from(&[Complex::new(4., 2.), Complex::new(2., 1.), Complex::new(-3., 1.)]);
    let v = Vector::from(&[Complex::new(-2., 1.), Complex::new(-5., 1.), Complex::new(16., 1.)]);
    println!("{}\n", Vector::cross_product(&u, &v));
    // [[17.0+26.0i]]
    // [[-57.0-41.0i]]
    // [[-17.0-6.0i]]

    println!("---------Mul Vec-----------\n");

    let mut u = Matrix::from(&[
        &[Complex::new(11., 3.), Complex::new(7., 1.)],
        &[Complex::new(8., 5.), Complex::new(2., 1.)],
        ]);

    let v = Vector::from(&[Complex::new(5., 6.), Complex::new(4., 3.)]);
    println!("{}\n", u.mul_vec(&v));
    // [[62.0+106.0i]]
    // [[15.0+83.0i]]

    println!("---------Mul Mat-----------\n");

    let mut u = Matrix::from(&[
        &[Complex::new(2., 0.), Complex::new(0., 0.)],
        &[Complex::new(0., 0.), Complex::new(2., 0.)],
        ]);

    let v = Matrix::from(&[
        &[Complex::new(4., 3.), Complex::new(2., 1.)],
        &[Complex::new(4., 3.), Complex::new(2., 1.)],
        ]);
    println!("{}\n", u.mul_mat(&v));
    // [[8.0+6.0i], [4.0+2.0i]]
    // [[8.0+6.0i], [4.0+2.0i]]

    println!("---------Trace-----------\n");
    let mut u = Matrix::from(&[
        &[Complex::new(1., 1.), Complex::new(0., 1.)],
        &[Complex::new(0., 1.), Complex::new(1., 1.)],
    ]);
    println!("{}\n", u.trace());
    // [2.0+2.0i]
    let mut u = Matrix::from(&[
        &[Complex::new(2., 1.), Complex::new(-5., 1.), Complex::new(0., 1.)],
        &[Complex::new(4., 1.), Complex::new(3., 1.), Complex::new(7., 1.)],
        &[Complex::new(-2., 1.), Complex::new(3., 1.), Complex::new(4., 1.)],
    ]);
    println!("{}\n", u.trace());
    // [9.0+3.0i]

    println!("---------Transpose-----------\n");
    let mut u = Matrix::from(&[
        &[Complex::new(1., 4.), Complex::new(0., 2.), Complex::new(0., -5.)],
        &[Complex::new(2., -3.), Complex::new(3., 12.), Complex::new(4., 4.)],
    ]);
    println!("{}\n", u.transpose());
    // [[1.0+4.0i], [2.0-3.0i]]
    // [[0.0+2.0i], [3.0+12.0i]]
    // [[0.0-5.0i], [4.0+4.0i]]

    println!("---------Row Echelon-----------\n");
    let mut u = Matrix::from(&[
        &[Complex::new(1., 2.), Complex::new(2., -4.), Complex::new(3., 1.)],
        &[Complex::new(3., -1.), Complex::new(4., 12.), Complex::new(5., 2.)],
    ]);
    println!("{}\n", u.row_echelon());
    // println!("{}\n", u.determinant());
    // [[1.0+0.0i], [0.0+0.0i], [1.402439-0.378049i]]
    // [[0.0+0.0i], [1.0+0.0i], [0.369512+0.025610i]]

    println!("---------Determinant-----------\n");

    let mut mat = Matrix::from(&[
		&[Complex::new(8., 2.), Complex::new(5., -3.), Complex::new(-2., 1.), Complex::new(7., 1.)],
		&[Complex::new(4., 1.), Complex::new(2.5, 1.), Complex::new(20., 1.), Complex::new(18., 1.)],
		&[Complex::new(7., 1.), Complex::new(5., 1.), Complex::new(1., 1.), Complex::new(4., 1.)],
		&[Complex::new(10., 1.), Complex::new(8., 1.), Complex::new(2., 1.), Complex::new(9., 1.)],
	]);
    println!("{}\n", mat.determinant());
    // [-495.0-1833.5i]


    println!("---------Inverse-----------\n");

    let mut u = Matrix::from(&[
        &[Complex::new(1., 1.), Complex::new(1., -1.)],
        &[Complex::new(1., 1.), Complex::new(-1., 1.)],
    ]);
    println!("{}\n", u.inverse());
    // [[0.25-0.25i], [0.25-0.25i]]
    // [[0.25+0.25i], [-0.25-0.25i]]
    let mut u = Matrix::from(&[
        &[Complex::new(4., 5.), Complex::new(3., 2.), Complex::new(-4., -6.)],
        &[Complex::new(12., 9.), Complex::new(14., -1.), Complex::new(10., -4.)],
        &[Complex::new(-13., 1.), Complex::new(-9., 3.), Complex::new(8., 7.)],
    ]);
    // println!("{}\n", u);
    println!("{}\n", u.inverse());
    // [[-0.116494-0.343592i], [-0.026613-0.076456i], [0.065785-0.234465i]]
    // [[-0.028939+0.336427i], [0.038430+0.105258i], [-0.191192+0.201445i]]
    // [[-0.155064-0.018762i], [0.008556-0.024397i], [-0.009344-0.082729i]]

    println!("---------Rank-----------\n");
    let mut u = Matrix::from(&[
        &[Complex::new(8., 2.), Complex::new(5., -3.), Complex::new(-2., 1.)],
        &[Complex::new(4., 1.), Complex::new(7., 1.), Complex::new(20., 1.)],
        &[Complex::new(7., 1.), Complex::new(6., 1.), Complex::new(1., 1.)],
        &[Complex::new(21., 1.), Complex::new(18., 1.), Complex::new(7., 1.)],
    ]);
    println!("{}\n", u.rank());
    // 3

    if -0. < 0. {
        println!("True\n");
    }
    else {
        print!("False\n");
    }
}