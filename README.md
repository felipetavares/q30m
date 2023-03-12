# Quantum Entanglement in 30 minutes

Following [Quantum in 30 seconds][q30s], the next step in simulating quantum
systems involves simulating entanglement.

[q30s]: https://github.com/felipetavares/q30s

# Experiment

This experiment is quite simple. We first initialize a system of two qubits to
an *entangled* state. That means we cannot describe this state from the
individual states of qubits:

$$S = \frac{\lvert 00 \rangle}{\sqrt{2}} + \frac{\lvert 11 \rangle}}{\sqrt{2}}$$

Then, we measure each qubit individually. Since this state is a superposition
of either both qubits being 0 or both being 1, whenever one of them is
measured, this forces the state of the other to match!

# Implementation

To create a simulator capable of entanglement, there are a few operations that
must be implemented:

- 2d tensors
- Kronecker product
- Projectors

Those are needed for a couple important concepts:

(1) Generating a n-qubit state can be done by using the Kronecker product
($\otimes$) of n-qubits:

$$b_{1} \otimes b_{2} \otimes \ldots \otimes b_{n}$$

(2) The probability of a qubit being in a state, for example the first qubit in
a two qubit state being $\lvert 0 \rangle$ can be calculated by:

$$|\lvert 0 \rangle \langle 0 \rvert \otimes I_2 \times S|^2$$

Where $I_2$ is a $2x2$ identity tensor.

The projector in this case is the result of the ket-bra multiplication. The
idea here is to build a n-gate operation out of the Kronecker product of
identity tensors and the projector for the qubit we are interested in.

The state after the measurement can be described similarly:

$$\frac{\lvert 0 \rangle \langle 0 \rvert \otimes I_2 \times S}{|\lvert 0 \rangle \langle 0 \rvert \otimes I_2 \times S|}$$

# Outcomes

```python
❯ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/q30m`
[src/main.rs:16] register.measure(0) = true
[src/main.rs:17] register.measure(1) = true
❯ cargo run --release
    Finished release [optimized] target(s) in 0.00s
     Running `target/release/q30m`
[src/main.rs:16] register.measure(0) = false
[src/main.rs:17] register.measure(1) = false
```
