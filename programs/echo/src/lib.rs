use anchor_lang::prelude::*;
use mpl_token_metadata::instructions::{DelegateStandardV1CpiBuilder, LockV1CpiBuilder};

declare_id!("3xnJZhQ8U7whiBToRmQ3H4UWHmRWkJJEGZ2fLY2SmZ95");
const PREFIX: &str = "echo-delegate";

#[program]
pub mod echo {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn delegate_and_lock(ctx: Context<DelegateAndLock>) -> Result<()> {
        let metadata_program_info = ctx.accounts.metadata_program.to_account_info();
        let delagate_info = ctx.accounts.delegate.to_account_info();
        let metadata_info = ctx.accounts.metadata.to_account_info();
        let mint_info = ctx.accounts.mint.to_account_info();
        let token_info = ctx.accounts.token.to_account_info();
        let owner_info = ctx.accounts.owner.to_account_info();
        let system_program_info = ctx.accounts.system_program.to_account_info();
        let sysvar_info = ctx.accounts.sysvar_instructions.to_account_info();

        let delegate_result = DelegateStandardV1CpiBuilder::new(metadata_program_info.as_ref())
            .authority(delagate_info.as_ref())
            .metadata(metadata_info.as_ref())
            .mint(mint_info.as_ref())
            .token(token_info.as_ref())
            .payer(owner_info.as_ref())
            .system_program(system_program_info.as_ref())
            .sysvar_instructions(sysvar_info.as_ref())
            .invoke();

        if delegate_result.is_err() {
            return Err(DelegateError::DelegateError.into());
        }

        let lock_result = LockV1CpiBuilder::new(metadata_program_info.as_ref())
            .authority(delagate_info.as_ref())
            .metadata(metadata_info.as_ref())
            .mint(mint_info.as_ref())
            .token(token_info.as_ref())
            .payer(owner_info.as_ref())
            .system_program(system_program_info.as_ref())
            .sysvar_instructions(sysvar_info.as_ref())
            .invoke_signed(&[&[
                PREFIX.as_bytes(),
                ctx.accounts.mint.key.as_ref(),
                ctx.accounts.owner.key.as_ref(),
                &[ctx.accounts.delegate.bump],
            ]]);

        if lock_result.is_err() {
            return Err(LockError::LockError.into());
        }

        let delegate = &mut ctx.accounts.delegate;
        delegate.mint = mint_info.key();
        delegate.owner = owner_info.key();
        delegate.bump = ctx.bumps.delegate;

        Ok(())
    }
}

#[account]
pub struct TokenDelegate {
    mint: Pubkey,
    owner: Pubkey,
    bump: u8,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DelegateAndLock<'info> {
    /// Update authority or token owner
    #[account(mut)]
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
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sysvar_instructions: UncheckedAccount<'info>,
    #[account(init,
    payer = owner,
    space = 8 + 32 + 32 + 1,
    seeds = [b"echo-delegate", mint.key().as_ref(), owner.key().as_ref()],
    bump)]
    pub delegate: Account<'info, TokenDelegate>,
}

#[error_code]
pub enum DelegateError {
    #[msg("delegate error")]
    DelegateError,
}

#[error_code]
pub enum LockError {
    #[msg("lock error")]
    LockError,
}
