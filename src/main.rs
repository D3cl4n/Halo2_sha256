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


fn main() {
    println!("Hello, world!");
}
