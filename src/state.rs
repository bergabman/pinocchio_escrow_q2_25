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

    pub fn set_inner(&mut self, new_self: Self) {

        self.amount = new_self.amount;
        self.seed = new_self.seed;
        self.maker = new_self.maker;
        self.mint_a = new_self.mint_a;
        self.mint_b = new_self.mint_b;
        self.amount = new_self.amount;
        self.receive = new_self.receive;
        self.bump = new_self.bump;
        
    }
}
