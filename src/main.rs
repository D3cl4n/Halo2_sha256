use std::marker::PhantomData;
use ff::Field;
use halo2_proofs::{
    circuit::{AssignedCell, Chip, Layouter, Region, SimpleFloorPlanner, Value},
    plonk::{Advice, Circuit, Column, ConstraintSystem, Error, Fixed, Instance, Selector},
    poly::Rotation
};


// stucture for the circuit
struct Sha256Circuit<F: Field> {
    constant: F,
    preimage: Vec<u8>,
    digest: Vec<u8>,
}

// chip configuration structure - this chip is for bitwise operators mod 32
#[derive(Clone, Debug)]
struct Sha256Config {
    advice: [Column<Advice>; 8], // one advice column per internal variable h0, h1, ..., h7
    instance: Column<Instance>,
    s_add_mod32: Selector,
    s_rotate: Selector,
    s_xor: Selector,
    s_and: Selector,
    s_shift: Selector,
}

// chip structure, holding a configuration and phantom data
struct Sha256Chip<F: Field> {
    config: Sha256Config,
    _marker: PhantomData<F>,
}


// main function - tests the circuit and runs benchmarks
fn main() {
    println!("Hello, world!");
}
