use enum_iterator::all;
use extensions::{self, types::RVector};

use crate::elys::{Cell, Q};

pub fn riemann_phi(left: &Cell, right: &Cell, x: RVector, da_dt: f64) {
    let r = x[0];

    let mut prim_l = RVector::zeros(6);
    let mut prim_r = RVector::zeros(6);

    for q in all::<Q>() {
        prim_l[q] = left.prim[q];
        prim_r[q] = right.prim[q];
    }
}
