use planetes_ext::types::RVector;

use crate::elys::{Cell, NUM_Q, Prim};

pub fn riemann_phi(left: &Cell, right: &Cell, x: &RVector, da_dt: f64) {
    let r = x[0];

    let mut prim_l = Prim::default();
    let mut prim_r = Prim::default();

    for q in 0..NUM_Q {
        prim_l[q] = left.prim[q];
        prim_r[q] = right.prim[q];
    }
}
