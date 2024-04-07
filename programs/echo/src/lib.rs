use anchor_lang::prelude::*;
use mpl_token_metadata::instructions::{DelegateStandardV1CpiBuilder, LockV1CpiBuilder};

declare_id!("FNwASR1r8FD9HA4beTDJnf5yAJkNYkFB9Wa1QwbV8v1P");

#[program]
pub mod echo {
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

        msg!("is writable? {}", delegate.is_writable);
        let delegate_result = DelegateStandardV1CpiBuilder::new(metadata_program.as_ref())
            .authority(authority.as_ref())
            .delegate(delegate.as_ref())
            .metadata(metadata.as_ref())
            .mint(mint.as_ref())
            .token(token.as_ref())
            .payer(authority.as_ref())
            .spl_token_program(Some(spl_token_program.as_ref()))
            .system_program(system_program.as_ref())
            .sysvar_instructions(sysvar.as_ref())
            .invoke();

        if delegate_result.is_err() {
            return err!(EchoError::DelegateError);
        }

        let lock_result = LockV1CpiBuilder::new(metadata_program.as_ref())
            .authority(delegate.as_ref())
            .token_owner(Some(authority.as_ref()))
            .metadata(metadata.as_ref())
            .mint(mint.as_ref())
            .token(token.as_ref())
            .payer(delegate.as_ref())
            .spl_token_program(Some(spl_token_program.as_ref()))
            .system_program(system_program.as_ref())
            .sysvar_instructions(sysvar.as_ref())
            .invoke_signed(&[&[
                b"echo-delegate",
                ctx.accounts.mint.key().as_ref(),
                ctx.accounts.authority.key().as_ref(),
                &[ctx.bumps.delegate],
            ]]);

        if lock_result.is_err() {
            return err!(EchoError::LockError);
        }
        msg!("lock done");
        Ok(())
    }
    pub fn delegate(ctx: Context<DelegateAndLock>) -> Result<()> {
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

        let delegate_result = DelegateStandardV1CpiBuilder::new(metadata_program.as_ref())
            .authority(authority.as_ref())
            .delegate(delegate.as_ref())
            .metadata(metadata.as_ref())
            .mint(mint.as_ref())
            .token(token.as_ref())
            .payer(authority.as_ref())
            .spl_token_program(Some(spl_token_program.as_ref()))
            .system_program(system_program.as_ref())
            .sysvar_instructions(sysvar.as_ref())
            .invoke();

        if delegate_result.is_err() {
            return err!(EchoError::DelegateError);
        }

        msg!("delegate done");
        Ok(())
    }

    pub fn lock(ctx: Context<DelegateAndLock>) -> Result<()> {
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

        let (found_delegate, bump) = Pubkey::find_program_address(
            &[
                b"echo-delegate",
                ctx.accounts.mint.key().as_ref(),
                ctx.accounts.authority.key().as_ref(),
                // &[ctx.bumps.delegate],
            ],
            ctx.program_id,
        );
        msg!("found delegate {}", found_delegate.key());
        msg!("found delegate bump {}", bump);
        Ok(())

        // let cpi_ctx = CpiContext::new_with_signer(
        //     metadata_program,
        //     CreateMetadataAccountsV3 {
        //         metadata: ctx.accounts.metadata_account.to_account_info(), // the metadata account being created
        //         mint: ctx.accounts.reward_token_mint.to_account_info(), // the mint account of the metadata account
        //         mint_authority: ctx.accounts.reward_token_mint.to_account_info(), // the mint authority of the mint account
        //         update_authority: ctx.accounts.reward_token_mint.to_account_info(), // the update authority of the metadata account
        //         payer: ctx.accounts.admin.to_account_info(), // the payer for creating the metadata account
        //         system_program: ctx.accounts.system_program.to_account_info(), // the system program account
        //         rent: ctx.accounts.rent.to_account_info(), // the rent sysvar account
        //     },
        //     signer,
        // );

        // let lock_result = LockV1CpiBuilder::new(metadata_program.as_ref())
        //     .authority(delegate.as_ref())
        //     .token_owner(Some(authority.as_ref()))
        //     .metadata(metadata.as_ref())
        //     .mint(mint.as_ref())
        //     .token(token.as_ref())
        //     .payer(delegate.as_ref())
        //     .spl_token_program(Some(spl_token_program.as_ref()))
        //     .system_program(system_program.as_ref())
        //     .sysvar_instructions(sysvar.as_ref())
        //     .add_remaining_account(delegate.as_ref(), true, true)
        //     // .invoke();
        //     .invoke_signed(&[&[
        //         b"echo-delegate",
        //         ctx.accounts.mint.key().as_ref(),
        //         ctx.accounts.authority.key().as_ref(),
        //         &[ctx.bumps.delegate],
        //     ]]);
        //
        // if lock_result.is_err() {
        //     return err!(EchoError::LockError);
        // }
        // msg!("lock done");
        // Ok(())
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

#[error_code]
pub enum EchoError {
    #[msg("delegate error")]
    DelegateError,
    #[msg("lock error")]
    LockError,
}
