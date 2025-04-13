use bytemuck::{Pod, Zeroable};
use pinocchio::pubkey::Pubkey;

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub amount: [u8; 8],
    pub receive: [u8; 8],
    pub seed: [u8; 8],
    pub bump: u8,
}

impl Escrow {
    pub const LEN: usize = core::mem::size_of::<Escrow>();
}
