use std::ops::{Add, Sub, Div, Mul, BitOr, Index};
use num::Complex;

type R = f64;
type C = Complex<R>;
type Data = Vec<C>;
type Shape = (usize, usize);

#[derive(Debug, Clone)]
pub struct Tensor {
    data: Data,
    shape: Shape
}

impl Tensor {
    pub fn new(data: Vec<C>, shape: Shape) -> Tensor {
        Tensor { data, shape }
    }

    pub fn eye(n: usize) -> Tensor {
        let mut data = Vec::<C>::new();

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    data.push(C::new(1f64, 0f64));
                } else {
                    data.push(C::new(0f64, 0f64));
                }
            }
        }

        Tensor { data, shape: (n, n) }
    }

    pub fn norm_sqr(&self) -> R {
        self.data.iter().map(|c| c.norm_sqr()).sum()
    }

    pub fn norm(&self) -> R {
        self.norm_sqr().sqrt()
    }

    pub fn unit(&self) -> Tensor {
        self / self.norm()
    }

    // Dagger - conjugate transpose
    pub fn dag(&self) -> Tensor {
        match self.shape {
            (m, n) if m == 1 || n == 1 => Tensor::new(self.data.iter().map(|c| c.conj()).collect(), (n, m)),
            (m, n) => {
                let mut data = Vec::<C>::new();

                for j in 0..n {
                    for i in 0..m {
                        data.push(self[(i, j)].conj());
                    }
                }

                Tensor::new(data, (n, m))
            }
         }
    }

    // Projector
    pub fn proj(&self) -> Tensor {
        self * &self.dag()
    }

    // Kronecker product
    pub fn prod(&self, rhs: &Tensor) -> Tensor {
        let shape = (self.shape.0*rhs.shape.0, self.shape.1*rhs.shape.1);
        let mut data = vec![C::new(0f64, 0f64); shape.0 * shape.1];

        // Walk the first matrix
        for i in 0..self.shape.1 {
            for j in 0..self.shape.0 {
                // For each element, walk the second matrix
                for k in 0..rhs.shape.1 {
                    for l in 0..rhs.shape.0 {
                        let x = i * rhs.shape.1 + k;
                        let y = j * rhs.shape.0 + l;

                        data[x + y * shape.1] = self[(j, i)] * rhs[(l, k)];
                    }
                }
            }
        }

        Tensor::new(data, shape)
    }

    pub fn expand(&self, n: usize, i: usize) -> Tensor {
        let eye = Tensor::eye(2);
        let mut product = if i == 0 { self.clone() } else { eye.clone() };

        for k in 1..n {
            product = if k == i {
                product.prod(self)
            } else {
                product.prod(&eye)
            }
        }

        product.clone()
    }
}

macro_rules! tensor_elementwise_op {
    ( $trait:ident, $op:ident ) => {
        impl $trait for Tensor {
            type Output = Tensor;

            fn $op(self, rhs: Tensor) -> Tensor {
                assert!(self.shape == rhs.shape);

                Tensor::new(
                    self.data.iter()
                        .zip(rhs.data.iter())
                        .map(|(c1, c2)| c1.$op(c2))
                        .collect()
                    ,
                    self.shape
                )
            }
        }
    };
}

impl Index<(usize, usize)> for Tensor {
    type Output = C;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.1 + index.0 * self.shape.1]
    }
}

tensor_elementwise_op!(Add, add);
tensor_elementwise_op!(Sub, sub);

impl Div<f64> for &Tensor {
    type Output = Tensor;

    fn div(self, rhs: f64) -> Tensor {
        Tensor::new(self.data.iter().map(|c| c / rhs).collect(), self.shape)
    }
}

// Dot product
impl BitOr for Tensor {
    type Output = C;

    fn bitor(self, rhs: Tensor) -> C {
        self.data.iter()
            .zip(rhs.data.iter())
            .map(|(c1, c2)| c1*c2)
            .sum()
    }
}

// Matrix multiplication
impl Mul<&Tensor> for &Tensor {
    type Output = Tensor;

    fn mul(self, rhs: &Tensor) -> Tensor {
        assert!(self.shape.1 == rhs.shape.0);

        let shape = (self.shape.0, rhs.shape.1);
        let mut data = Vec::<C>::new();
        let n = self.shape.1;

        for i in 0..shape.0 {
            for j in 0..shape.1 {
                data.push((0..n).map(|k| self[(i, k)] * rhs[(k, j)]).sum());
            }
        }

        Tensor::new(data, shape)
    }
}

impl Mul for Tensor {
    type Output = Tensor;

    fn mul(self, rhs: Tensor) -> Tensor {
        &self * &rhs
    }
}
