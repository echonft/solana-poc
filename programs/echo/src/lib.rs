use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::{approve, Approve, freeze_account, FreezeAccount},
};

declare_id!("9YimkcCy3hXuMkCRU2CHbWGZTpKF4o4zLCkpSpGANfDN");

#[program]
pub mod echo {
    use anchor_spl::token::accessor::authority;
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn delegate_and_lock(ctx: Context<DelegateAndLock>) -> Result<()> {

        let metadata_program = ctx.accounts.metadata_program.to_account_info();
        let delegate = ctx.accounts.delegate.to_account_info();
        let metadata = ctx.accounts.metadata.to_account_info();
        let mint = ctx.accounts.mint.to_account_info();
        let token = ctx.accounts.token.to_account_info();
        let authority = ctx.accounts.authority.to_account_info();
        let spl_token_program = ctx.accounts.spl_token_program.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();
        let sysvar = ctx.accounts.sysvar_instructions.to_account_info();
        ctx.accounts.delegate.mint = mint.key();
        ctx.accounts.delegate.authority = authority.key();
        ctx.accounts.delegate.bump = ctx.bumps.delegate;


        msg!("Calling the token program to approve PDA...");
        let delegate_result = approve(CpiContext::new_with_signer(ctx.accounts.spl_token_program.to_account_info(), Approve {
            to: ctx.accounts.token.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            delegate: ctx.accounts.delegate.to_account_info(),
        }, &[&[authority.key().as_ref()]]), 1);

        if delegate_result.is_err() {
            return err!(EchoError::DelegateError);
        }

        let lock_result = freeze_account(CpiContext::new_with_signer(ctx.accounts.spl_token_program.to_account_info(), FreezeAccount {
            authority: ctx.accounts.metadata.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            account: ctx.accounts.token.to_account_info(),
        }, &[&[
                    b"echo-delegate",
                    ctx.accounts.authority.key().as_ref(),
                    ctx.accounts.mint.key().as_ref(),
                    &[ctx.bumps.delegate],
                ]]));


        if lock_result.is_err() {
            return err!(EchoError::LockError);
        }
        msg!("lock done");
        Ok(())
    }

}

#[account]
pub struct TokenDelegate {
    authority: Pubkey,
    mint: Pubkey,
    bump: u8,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct DelegateAndLock<'info> {
    /// Update authority or token owner
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Metadata account
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub metadata_program: UncheckedAccount<'info>,
    /// Mint of metadata
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: UncheckedAccount<'info>,
    /// Token account of mint
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub spl_token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sysvar_instructions: UncheckedAccount<'info>,
    #[account(init,
    payer = authority,
    space = 8 + 32 + 32 + 1,
    seeds = [b"echo-delegate", authority.key().as_ref(), mint.key().as_ref()],
    bump)]
    pub delegate: Account<'info, TokenDelegate>,
}

#[derive(Accounts)]
pub struct Lock<'info> {
    /// Update authority or token owner
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Metadata account
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub metadata_program: UncheckedAccount<'info>,
    /// Mint of metadata
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub mint: UncheckedAccount<'info>,
    /// Token account of mint
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub spl_token_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub sysvar_instructions: UncheckedAccount<'info>,
    #[account(
    seeds = [b"echo-delegate", authority.key().as_ref(), mint.key().as_ref()],
    bump)]
    pub delegate: Account<'info, TokenDelegate>,
}

#[error_code]
pub enum EchoError {
    #[msg("delegate error")]
    DelegateError,
    #[msg("lock error")]
    LockError,
}
