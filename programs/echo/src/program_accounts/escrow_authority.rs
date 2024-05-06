use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
#[derive(Default)]
pub struct EscrowAuthority {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub offer: Pubkey,
    pub bump: u8,
}
