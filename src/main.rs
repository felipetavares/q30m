#[macro_use]
mod tensor;
mod register;

use register::QuantumRegister;

fn main() {
    // 1. Prepare a state that's either |00> or |11>: either bit can be 0 or 1 but after a
    //    measurement, the other bit will be have the same value.
    let entangled_state = &(ket![|0>].prod(&ket![|0>]) + ket![|1>].prod(&ket![|1>])) / 2f64.sqrt();
    let mut register = QuantumRegister::new(entangled_state, 2);

    // 2. Measure both bits
    dbg!(register.measure(0));
    dbg!(register.measure(1));
}
