use crate::enums::offer_state::OfferState;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::pubkey::Pubkey;

#[account]
pub struct Offer {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub sender_token_mint: Pubkey,
    pub receiver_token_mint: Pubkey,
    pub state: OfferState,
    // TODO add expiration date
    // pub expiration: i64,
    pub bump: u8,
}
