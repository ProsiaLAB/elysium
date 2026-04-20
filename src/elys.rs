use prosia_extensions::types::RVector;

pub const NUM_Q: usize = 6;

pub mod prim {
    pub const RHO: usize = 0;
    pub const PPP: usize = 1;
    pub const URR: usize = 2;
    pub const UPP: usize = 3;
    pub const UZZ: usize = 4;
    pub const BRR: usize = 5;
    pub const BPP: usize = 6;
    pub const BZZ: usize = 7;
}

pub mod cons {
    pub const DDD: usize = 0;
    pub const TAU: usize = 1;
    pub const SRR: usize = 2;
    pub const LLL: usize = 3;
    pub const SZZ: usize = 4;
}

/// Primitive variables
#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Prim(pub [f64; NUM_Q]);

use std::ops::{Index, IndexMut};

impl Index<usize> for Prim {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.0[i]
    }
}

impl IndexMut<usize> for Prim {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.0[i]
    }
}

impl Prim {
    #[must_use]
    pub fn zero() -> Self {
        Self([0.0; NUM_Q])
    }

    #[must_use]
    pub fn as_array(&self) -> &[f64; NUM_Q] {
        &self.0
    }

    pub fn as_mut_array(&mut self) -> &mut [f64; NUM_Q] {
        &mut self.0
    }
}

impl Prim {
    #[must_use]
    pub fn rho(&self) -> f64 {
        self[prim::RHO]
    }

    #[must_use]
    pub fn p(&self) -> f64 {
        self[prim::PPP]
    }

    #[must_use]
    pub fn vr(&self) -> f64 {
        self[prim::URR]
    }

    #[must_use]
    pub fn vp(&self) -> f64 {
        self[prim::UPP]
    }

    #[must_use]
    pub fn vz(&self) -> f64 {
        self[prim::UZZ]
    }

    #[must_use]
    pub fn br(&self) -> f64 {
        self[prim::BRR]
    }

    #[must_use]
    pub fn bp(&self) -> f64 {
        self[prim::BPP]
    }

    #[must_use]
    pub fn bz(&self) -> f64 {
        self[prim::BZZ]
    }
}

/// Conserved variables
#[derive(Debug, Default)]
pub struct Cons(pub [f64; 5]);

pub struct Cell {
    pub prim: RVector,
    pub cons: RVector,
    pub rk_cons: RVector,
    pub grad: RVector,
    pub grad_p: RVector,
    pub piph: f64,
    pub dphi: f64,
    pub wiph: f64,
    pub e: RVector,
    pub b: RVector,
    pub e_phi: RVector,
    pub phi: RVector,
    pub rk_phi: RVector,
    pub temp: f64,
}
