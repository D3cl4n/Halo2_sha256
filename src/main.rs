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
    fixed: Column<Fixed>, // fixed column for the sha256 round constants
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

// trait for the Sha256Chip bitwise operations
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

// implementing gates and supporting methods for the Sha256Chip
impl<F: Field> Sha256Chip<F> {
    // constructor
    fn construct(config: <Self as Chip<F>>::Config) -> Self {
        Sha256Chip { config, _marker: PhantomData, }
    }

    // configure the chip with the set of columns, gates, and constraints
    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 8],
        fixed: Column<Fixed>,
        instance: Column<Instance>
    ) -> <Self as Sha256Chip<F>>::Config {
        meta.enable_equality(instance); // enable the copy constraint
        meta.enable_constant(fixed);

        // enable copy constraint on each advice column
        for advice_col in &advice {
            meta.enable_equality(*advice_col);
        }

        // all the selectors the chip needs for the set of gates
        let s_add_mod32 = meta.selector();
        let s_rotate = meta.selector();
        let s_xor = meta.selector();
        let s_and = meta.selector();
        let s_shift = meta.selector();

        // define the set of gates and constraints for the chip
        meta.create_gate("addition_mod32_gate", |meta| {
            //TODO: figure out how to do addition mod 2^32 here
        });

        // return the Sha256Chip config
        Sha256Config {
            advice,
            fixed, 
            instance, 
            s_add_mod32,
            s_rotate,
            s_xor,
            s_and,
            s_shift,
        }
    }
}


// main function - tests the circuit and runs benchmarks
fn main() {
    println!("Hello, world!");
}
