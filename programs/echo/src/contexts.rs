use crate::account::{EscrowAuthority, Offer};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{AssociatedToken, ID as SPL_ASSOCIATED_TOKEN_PROGRAM_ID};
use anchor_spl::token::spl_token::ID as SPL_TOKEN_PROGRAM_ID;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum OfferState {
    OPEN,
    CANCELLED,
    REJECTED,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateOfferAccounts<'info> {
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

#[derive(Accounts)]
pub struct AcceptOfferAccounts<'info> {
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
