use enum_iterator::Sequence;
use extensions::types::RVector;

/// Primitive variables
#[derive(Sequence)]
pub enum Prim {
    /// Mass density
    Rho,
    /// Pressure
    Ppp,
    /// Radial velocity
    Urr,
    /// Azimuthal velocity
    Upp,
    /// Axial velocity
    Uzz,
    /// Radial magnetic field
    Brr,
    /// Azimuthal magnetic field
    Bpp,
    /// Axial magnetic field
    Bzz,
}

pub type Q = Prim;

/// Conserved variables
pub enum Cons {
    /// Lab-frame mass density
    Ddd,
    /// total energy density
    Tau,
    /// Radial momentum density
    Srr,
    /// Azimuthal angular momentum density
    Lll,
    /// Axial momentum density
    Szz,
}

pub type U = Cons;

impl std::ops::Index<Q> for RVector {
    type Output = f64;
    fn index(&self, q: Q) -> &Self::Output {
        &self[q as usize]
    }
}
impl std::ops::IndexMut<Q> for RVector {
    fn index_mut(&mut self, q: Q) -> &mut Self::Output {
        &mut self[q as usize]
    }
}

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
