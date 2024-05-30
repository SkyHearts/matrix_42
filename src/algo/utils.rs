
pub trait Utils{
    fn absolute(&self) -> Self;
    fn square(&self) -> Self;
    fn sroot(&self) -> Self;
    fn one() -> Self;
    fn zero() -> Self;
    // fn power(&self, n: usize) -> Self;
}

impl Utils for f32 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        self.sqrt()
    }

    fn one() -> Self {
        1.0
    }

    fn zero() -> Self {
        0.0
    }
    
    // fn power(&self, n: usize) -> Self {
    //     self.powi(n as i32)
    // }
}

impl Utils for f64 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        self.sqrt()
    }

    fn one() -> Self {
        1.0
    }

    fn zero() -> Self {
        0.0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.powi(n as i32)
    // }
}

impl Utils for i8 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as i8
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for i16 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as i16
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for i32 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as i32
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for i64 {
    fn absolute(&self) -> Self {
        self.abs()
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as i64
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for u8 {
    fn absolute(&self) -> Self {
        *self
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as u8
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for u16 {
    fn absolute(&self) -> Self {
        *self
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as u16
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for u32 {
    fn absolute(&self) -> Self {
        *self
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as u32
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}

impl Utils for u64 {
    fn absolute(&self) -> Self {
        *self
    }

    fn square(&self) -> Self {
        self * self
    }

    fn sroot(&self) -> Self {
        // self.isqrt()
        (*self as f64).sqrt() as u64
    }

    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }

    // fn power(&self, n: usize) -> Self {
    //     self.pow(n as u32)
    // }
}