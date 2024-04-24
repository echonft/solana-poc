use crate::{program_accounts::escrow_authority::EscrowAuthority, program_accounts::offer::Offer};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken, ID as SPL_ASSOCIATED_TOKEN_PROGRAM_ID};
use anchor_spl::token::spl_token::ID as SPL_TOKEN_PROGRAM_ID;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CreateOfferContext<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut,
    token::mint = token_mint,
    token::authority = sender
    )]
    pub token: Box<Account<'info, TokenAccount>>,
    pub token_mint: Box<Account<'info, Mint>>,
    #[account(init,
    payer = sender,
    space = 8 + 32 + 32 + 32 + 1,
    seeds = [b"escrow", sender.key().as_ref(), token_mint.key().as_ref()],
    bump
    )]
    pub escrow: Box<Account<'info, EscrowAuthority>>,
    #[account(init,
    payer = sender,
    associated_token::mint = token_mint,
    associated_token::authority = escrow,
    associated_token::token_program = spl_token_program
    )]
    pub escrow_associated_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we're not writing or reading from this account
    pub receiver: UncheckedAccount<'info>,
    #[account(
    token::mint = receiver_token_mint,
    token::authority = receiver
    )]
    pub receiver_token: Box<Account<'info, TokenAccount>>,
    pub receiver_token_mint: Box<Account<'info, Mint>>,
    // TODO Modify seed, too limitative as of now
    #[account(
    init,
    payer = sender,
    space = 8 + 32 + 32 + 32 + 32 + 1 + 1,
    seeds = [b"offer", sender.key().as_ref(), receiver.key().as_ref()],
    bump
    )]
    pub offer: Box<Account<'info, Offer>>,
    #[account(address = SPL_TOKEN_PROGRAM_ID)]
    pub spl_token_program: Program<'info, Token>,
    #[account(address = SPL_ASSOCIATED_TOKEN_PROGRAM_ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
