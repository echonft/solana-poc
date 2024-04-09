use anchor_lang::prelude::*;
use anchor_spl::token::spl_token::{instruction::AuthorityType, ID as SPL_TOKEN_PROGRAM_ID};
use anchor_spl::token::{set_authority, Mint, SetAuthority, Token, TokenAccount};

declare_id!("Gzq13nAmkDZMgFjKs8Zd6jJbXE2X6iJJ4FEe2BqWcVWM");

#[program]
pub mod echo {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
    pub fn create_offer(ctx: Context<CreateOfferAccounts>) -> Result<()> {
        set_authority(
            CpiContext::new(
                ctx.accounts.spl_token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.token.to_account_info(),
                    current_authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.escrow.key()),
        )
        .unwrap();

        let offer = &mut ctx.accounts.offer;
        offer.sender = ctx.accounts.sender.key();
        offer.receiver = ctx.accounts.receiver.key();
        offer.sender_token = ctx.accounts.token.key();
        offer.receiver_token = ctx.accounts.receiver_token.key();
        offer.state = OfferState::OPEN;
        offer.bump = ctx.bumps.offer;

        let escrow = &mut ctx.accounts.escrow;
        escrow.owner = ctx.accounts.sender.key();
        escrow.token = ctx.accounts.token.key();
        escrow.offer = ctx.accounts.offer.key();
        escrow.bump = ctx.bumps.escrow;

        Ok(())
    }

    pub fn accept_offer(ctx: Context<AcceptOfferAccounts>) -> Result<()> {
        // TODO assert offer is not expired
        // transfer the token from sender escrow -> receiver
        set_authority(
            CpiContext::new_with_signer(
                ctx.accounts.spl_token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.sender_token.to_account_info(),
                    current_authority: ctx.accounts.sender_escrow.to_account_info(),
                },
                &[&[
                    b"escrow",
                    ctx.accounts.sender.key().as_ref(),
                    ctx.accounts.sender_token.key().as_ref(),
                    &[ctx.accounts.sender_escrow.bump],
                ]],
            ),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.receiver.key()),
        )
        .unwrap();
        // transfer the token from receiver -> sender
        set_authority(
            CpiContext::new(
                ctx.accounts.spl_token_program.to_account_info(),
                SetAuthority {
                    account_or_mint: ctx.accounts.sender_token.to_account_info(),
                    current_authority: ctx.accounts.sender_escrow.to_account_info(),
                },
            ),
            AuthorityType::AccountOwner,
            Some(ctx.accounts.sender.key()),
        )
        .unwrap();

        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum OfferState {
    OPEN,
    CANCELLED,
    REJECTED,
}

#[account]
pub struct Escrow {
    owner: Pubkey,
    token: Pubkey,
    offer: Pubkey,
    bump: u8,
}

#[account]
pub struct Offer {
    sender: Pubkey,
    receiver: Pubkey,
    sender_token: Pubkey,
    receiver_token: Pubkey,
    state: OfferState,
    bump: u8,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateOfferAccounts<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
    token::mint = token_mint,
    token::authority = sender
    )]
    pub token: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    #[account(
    init,
    payer = sender,
    space = 8 + 32 + 32 + 32 + 1,
    seeds = [b"escrow", sender.key().as_ref(), token.key().as_ref()],
    bump
    )]
    pub escrow: Account<'info, Escrow>,
    // TODO get a better seed: with a nonce (possibly # of offers created?)
    #[account(
    init,
    payer = sender,
    space = 8 + 32 + 32 + 32 + 32 + 1 + 1,
    seeds = [b"offer", sender.key().as_ref(), token.key().as_ref()],
    bump
    )]
    pub offer: Account<'info, Offer>,
    /// CHECK: This is not dangerous because we're checking the address from the offer
    #[account(address = offer.receiver.key())]
    pub receiver: UncheckedAccount<'info>,
    #[account(
    token::mint = receiver_token_mint,
    token::authority = receiver
    )]
    pub receiver_token: Account<'info, TokenAccount>,
    pub receiver_token_mint: Account<'info, Mint>,
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    pub spl_token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AcceptOfferAccounts<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(
    token::mint = token_mint,
    token::authority = receiver
    )]
    pub token: Account<'info, TokenAccount>,
    pub token_mint: Account<'info, Mint>,
    // TODO get a better seed: with a nonce (possibly # of offers created?)
    #[account(mut,
    close = sender,
    seeds = [b"offer", sender.key().as_ref(), sender_token.key().as_ref()],
    bump
    )]
    pub offer: Account<'info, Offer>,
    /// CHECK: This is not dangerous because we're checking the address from the offer
    #[account(mut,
    address = offer.sender.key())]
    pub sender: UncheckedAccount<'info>,
    #[account(mut,
    close = sender,
    seeds = [b"escrow", sender.key().as_ref(), sender_token.key().as_ref()],
    bump
    )]
    pub sender_escrow: Account<'info, Escrow>,
    #[account(
    token::mint = sender_token_mint,
    token::authority = sender_escrow
    )]
    pub sender_token: Account<'info, TokenAccount>,
    pub sender_token_mint: Account<'info, Mint>,
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    pub spl_token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
