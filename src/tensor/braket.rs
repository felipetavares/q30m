#[macro_export]
macro_rules! ket {
    ( $( $re:literal $(+ $im:literal i)? ),* ) => {
        {
            use num::Complex;
            use crate::tensor::Tensor;

            let mut data: Vec<Complex<f64>> = Vec::new();
            $(
                // Rust gets confused by the optional immaginary part and thinks it's never getting
                // used or assigned to.
                #[allow(unused_mut, unused_assignments)]
                {
                    let re = $re as f64;
                    let mut im = 0f64; $( im = $im as f64; )?
                    data.push(Complex::<f64>::new(re, im));
                }
            )*
            let shape = (data.len(), 1);

            Tensor::new(data, shape)
        }
    };
    ( |0> ) => { ket![1, 0] };
    ( |1> ) => { ket![0, 1] };
    ( |+> ) => { (ket![|0>] + ket![|1>]).unit() };
    ( |-> ) => { (ket![|0>] - ket![|1>]).unit() };
}

#[macro_export]
macro_rules! bra {
    ( $( $re:literal $(+ $im:literal i)? ),* ) => {
        {
            use num::Complex;
            use crate::tensor::Tensor;

            let mut data: Vec<Complex<f64>> = Vec::new();
            $(
                // Rust gets confused by the optional immaginary part and thinks it's never getting
                // used or assigned to.
                #[allow(unused_mut, unused_assignments)]
                {
                    let re = $re as f64;
                    let mut im = 0f64; $( im = $im as f64; )?
                    data.push(Complex::<f64>::new(re, im));
                }
            )*
            let shape = (1, data.len());

            Tensor::new(data, shape)
        }
    };
    ( <0| ) => { bra![1, 0] };
    ( <1| ) => { bra![0, 1] };
    ( <+| ) => { (bra![<0|] + bra![<1|]).unit() };
    ( <-| ) => { (bra![<0|] - bra![<1|]).unit() };
}

pub use bra;
pub use ket;
