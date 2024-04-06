use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::instructions::{DelegateStandardV1CpiBuilder, LockV1CpiBuilder};
use solana_program::sysvar::instructions::Instructions;
use solana_program::sysvar::Sysvar;

declare_id!("251VpQ2e7acPSqM4m7DRoUMpfX9mEtFXHjbYRx2C5JGX");
const PREFIX: &str = "echo-delegate";

#[program]
pub mod solana {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn create_delegate(ctx: Context<CreateDeletage>) -> Result<()> {
        let pda = &mut ctx.accounts.delegate;
        pda.mint = ctx.accounts.mint.key.key();
        pda.owner = ctx.accounts.owner.key();
        pda.bump = ctx.bumps.delegate;
        Ok(())
    }
    pub fn lock(ctx: Context<LockToken>) -> Result<()> {
        let metadata_program_info = ctx.accounts.metadata_program.to_account_info();
        let delagate_info = ctx.accounts.delegate.to_account_info();
        let metadata_info = ctx.accounts.metadata.to_account_info();
        let mint_info = ctx.accounts.mint.to_account_info();
        let token_info = ctx.accounts.token.to_account_info();
        let owner_info = ctx.accounts.owner.to_account_info();
        let system_program_info = ctx.accounts.system_program.to_account_info();
        let sysvar_info = ctx.accounts.sysvar_instructions.to_account_info();

        let mut cpi_lock = LockV1CpiBuilder::new(metadata_program_info.as_ref());
        cpi_lock
            .authority(delagate_info.as_ref())
            .metadata(metadata_info.as_ref())
            .mint(mint_info.as_ref())
            .token(token_info.as_ref())
            .payer(owner_info.as_ref())
            .system_program(system_program_info.as_ref())
            .sysvar_instructions(sysvar_info.as_ref());

        let result = cpi_lock.invoke_signed(&[&[
            PREFIX.as_bytes(),
            ctx.accounts.mint.key.as_ref(),
            ctx.accounts.owner.key.as_ref(),
        ]]);
        if (result.is_err()) {
            return Err(LockError::LockError.into());
        }
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[account]
pub struct TokenDelegate {
    mint: Pubkey,
    owner: Pubkey,
    bump: u8,
}

#[derive(Accounts)]
pub struct CreateDeletage<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    // space: 8 discriminator + (32*8) mint + (32*8) owner + 1 bump
    #[account(
    init,
    payer = owner,
    space = 8 + 32*8 + 32*8 + 1, seeds = [b"echo-delegate", mint.key().as_ref(), owner.key().as_ref()], bump
    )]
    pub delegate: Account<'info, TokenDelegate>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct LockToken<'info> {
    pub owner: Signer<'info>,
    /// Metadata account
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub metadata_program: UncheckedAccount<'info>,
    /// Mint of metadata
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: UncheckedAccount<'info>,
    /// Token account of mint
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token: UncheckedAccount<'info>,
    /// Update authority or token owner
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sysvar_instructions: UncheckedAccount<'info>,
    #[account(mut, seeds = [b"echo-delegate", mint.key().as_ref(), owner.key().as_ref()], bump = delegate.bump)]
    pub delegate: Account<'info, TokenDelegate>,
}

#[error_code]
pub enum LockError {
    #[msg("lock error")]
    LockError,
}
