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

// structure for numbers - cell values
struct Number<F: Field>(AssignedCell<F, F>);

// implement the Chip trait from halo2_proofs
impl<F: Field> Chip<F> for Sha256Chip<F> {
    type Config = Sha256Config;
    type Loaded = ();

    // getter for the config
    fn config(&self) -> &Self::Config {
        &self.config
    }

    // getter for the loaded instance variable
    fn loaded(&self) -> &Self::Loaded {
        &()
    }
}

// trait for the Sha256Chip
trait Sha256Operations<F: Field>: Chip<F> {
    type Num;

    // load a private (witness) value into the circuit
    fn load_private(&self, layouter: impl Layouter<F>, a: Value<F>) -> Result<Self::Num, Error>;

    // load a constant (witness) value into the circuit
    fn load_constant(&self, layouter: impl Layouter<F>, constant: F) -> Result<Self::Num, Error>;

    // expose a cell value as public
    fn expose_as_public(&self, layouter: impl Layouter<F>, num: Self::Num, row: usize) -> Result<(), Error>;

    // adds two numbers together modulo 32, returns the numeric result
    fn add_mod32(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;

    // bitwise rotate on a given number by n positions. Rotate left if flag is true, right if false
    fn bitwise_rotate(&self, layouter: impl Layouter<F>, a: Self::Num, n: usize, direction: bool) -> Result<Self::Num, Error>;

    // bitwise XOR on two given numbers
    fn bitwise_xor(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;

    // bitwise AND on two given numbers
    fn bitwise_and(&self, layouter: impl Layouter<F>, a: Self::Num, b: Self::Num) -> Result<Self::Num, Error>;

    // bitwise shift on a given number by n positions. Shift left if flag is true, right if false
    fn bitwise_shift(&self, layouter: impl Layouter<F>, a: Self::Num, n: usize, direction: bool) -> Result<Self::Num, Error>;
}


// main function - tests the circuit and runs benchmarks
fn main() {
    println!("Hello, world!");
}
