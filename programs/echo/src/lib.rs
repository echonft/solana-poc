use anchor_lang::prelude::*;
pub(crate) use constants::*;
pub(crate) use context::*;
pub(crate) use enums::*;
pub(crate) use error::*;
pub(crate) use program_accounts::*;
pub(crate) use types::*;

pub mod constants;
pub mod context;
pub mod enums;
pub mod error;
pub mod instructions;
pub mod program_accounts;

pub mod types;

declare_id!("Gzq13nAmkDZMgFjKs8Zd6jJbXE2X6iJJ4FEe2BqWcVWM");

#[program]
pub mod echo {
    use super::*;

    pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
        return instructions::initialize(ctx);
    }
    pub fn accept_offer(ctx: Context<AcceptOfferContext>) -> Result<()> {
        return instructions::accept_offer(ctx);
    }
    pub fn create_offer(ctx: Context<CreateOfferContext>) -> Result<()> {
        return instructions::create_offer(ctx);
    }
    // TODO cancel_offer

    // Wormhole
    pub fn register_emitter(
        ctx: Context<RegisterEmitterContext>,
        chain: u16,
        address: [u8; 32],
    ) -> Result<()> {
        return instructions::register_emitter(ctx, chain, address);
    }
    pub fn receive_message(ctx: Context<ReceiveMessageContext>, vaa_hash: [u8; 32]) -> Result<()> {
        return instructions::receive_message(ctx, vaa_hash);
    }
    pub fn send_message(ctx: Context<SendMessageContext>) -> Result<()> {
        return instructions::send_message(ctx);
    }
}
