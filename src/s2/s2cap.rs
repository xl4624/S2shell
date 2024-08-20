use crate::{s1::S1ChordAngle, s2::S2Point};

#[derive(Debug)]
pub struct S2Cap {
    center: S2Point,
    radius: S1ChordAngle,
}
