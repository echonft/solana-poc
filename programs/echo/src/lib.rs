use anchor_lang::prelude::*;
pub use constants::*;
pub use context::*;
pub use enums::*;
pub use error::*;
pub use instructions::*;
pub use message::*;
pub use program_accounts::*;
pub use types::*;

pub mod constants;
pub mod context;
pub mod enums;
pub mod error;
pub mod instructions;
pub mod message;
pub mod program_accounts;
pub mod types;

declare_id!("Gzq13nAmkDZMgFjKs8Zd6jJbXE2X6iJJ4FEe2BqWcVWM");

#[program]
pub mod echo {
    use super::*;
    // use anchor_lang::solana_program;

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
}
