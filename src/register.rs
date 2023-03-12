use rand::Rng;
use super::tensor::Tensor;
use super::tensor::braket::ket;

pub struct QuantumRegister {
    state: Tensor,
    bits: usize,
}

impl QuantumRegister {
    pub fn new(state: Tensor, bits: usize) -> QuantumRegister {
        QuantumRegister { state, bits }
    }

    pub fn measure(&mut self, bit: usize) -> bool {
        let states: Vec<Tensor> =
            vec![ ket![|0>].proj(), ket![|1>].proj() ].iter()
            .map(|p| &p.expand(self.bits, bit) * &self.state )
            .collect();
        let probs: Vec<f64> = states.iter().map(|state| state.norm_sqr()).collect();
        let outcome = rand::thread_rng().gen_bool(probs[1]);

        if outcome {
            self.state = states[1].unit();
        } else {
            self.state = states[0].unit();
        }

        outcome
    }
}

