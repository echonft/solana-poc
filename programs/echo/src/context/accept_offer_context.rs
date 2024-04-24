use crate::{program_accounts::escrow_authority::EscrowAuthority, program_accounts::offer::Offer};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken, ID as SPL_ASSOCIATED_TOKEN_PROGRAM_ID};
use anchor_spl::token::spl_token::ID as SPL_TOKEN_PROGRAM_ID;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct AcceptOfferContext<'info> {
    #[account(mut)]
    pub receiver: Signer<'info>,
    #[account(mut,
    token::mint = token_mint,
    token::authority = receiver
    )]
    pub token: Box<Account<'info, TokenAccount>>,
    pub token_mint: Box<Account<'info, Mint>>,
    #[account(init_if_needed,
    payer = receiver,
    associated_token::mint = sender_token_mint,
    associated_token::authority = receiver,
    associated_token::token_program = spl_token_program
    )]
    pub asociated_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: This is not dangerous because we're not writing or reading from this account
    #[account(mut)]
    pub sender: UncheckedAccount<'info>,
    #[account(mut,
    close = sender,
    seeds = [b"escrow", sender.key().as_ref(), sender_token_mint.key().as_ref()],
    bump
    )]
    pub sender_escrow: Box<Account<'info, EscrowAuthority>>,
    #[account(mut,
    associated_token::mint = sender_token_mint,
    associated_token::authority = sender_escrow
    )]
    pub sender_escrow_associated_token_account: Box<Account<'info, TokenAccount>>,
    pub sender_token_mint: Box<Account<'info, Mint>>,
    #[account(init_if_needed,
    payer = receiver,
    associated_token::mint = token_mint,
    associated_token::authority = sender,
    associated_token::token_program = spl_token_program
    )]
    pub sender_associated_token_account: Box<Account<'info, TokenAccount>>,
    #[account(mut,
    close = sender,
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
