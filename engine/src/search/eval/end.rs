use crate::defs::MATE;

#[must_use]
pub fn mated_in(ply: u8) -> i32 {
    -MATE + i32::from(ply)
}
