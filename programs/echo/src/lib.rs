mod account;
mod contexts;
use crate::contexts::{AcceptOfferAccounts, CreateOfferAccounts, Initialize};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

#[program]
pub mod echo {
    use super::*;
    use crate::contexts::{AcceptOfferAccounts, CreateOfferAccounts, OfferState};
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn create_offer(ctx: Context<CreateOfferAccounts>) -> anchor_lang::Result<()> {
        let offer = &mut ctx.accounts.offer;
        offer.sender = ctx.accounts.sender.key();
        offer.receiver = ctx.accounts.receiver.key();
        offer.sender_token_mint = ctx.accounts.token_mint.key();
        offer.receiver_token_mint = ctx.accounts.receiver_token_mint.key();
        offer.state = OfferState::OPEN;
        offer.bump = ctx.bumps.offer;

        let escrow = &mut ctx.accounts.escrow;
        escrow.owner = ctx.accounts.sender.key();
        escrow.mint = ctx.accounts.token_mint.key();
        escrow.offer = ctx.accounts.offer.key();

        escrow.bump = ctx.bumps.escrow;

        transfer(
            CpiContext::new(
                ctx.accounts.spl_token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token.to_account_info(),
                    to: ctx
                        .accounts
                        .escrow_associated_token_account
                        .to_account_info(),
                    authority: ctx.accounts.sender.to_account_info(),
                },
            ),
            1,
        )
        .unwrap();
        msg!("Tokens escrowed, saving offer data");

        Ok(())
    }

    pub fn accept_offer(ctx: Context<AcceptOfferAccounts>) -> anchor_lang::Result<()> {
        // transfer the token from sender escrow -> receiver
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.spl_token_program.to_account_info(),
                Transfer {
                    from: ctx
                        .accounts
                        .sender_escrow_associated_token_account
                        .to_account_info(),
                    to: ctx.accounts.asociated_token_account.to_account_info(),
                    authority: ctx.accounts.sender_escrow.to_account_info(),
                },
                &[&[
                    b"escrow",
                    ctx.accounts.sender.key().as_ref(),
                    ctx.accounts.sender_token_mint.key().as_ref(),
                    &[ctx.accounts.sender_escrow.bump],
                ]],
            ),
            1,
        )
        .unwrap();
        // transfer the token from receiver -> sender
        transfer(
            CpiContext::new(
                ctx.accounts.spl_token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token.to_account_info(),
                    to: ctx
                        .accounts
                        .sender_associated_token_account
                        .to_account_info(),
                    authority: ctx.accounts.receiver.to_account_info(),
                },
            ),
            1,
        )
        .unwrap();

        // TODO Close sender_associated_token_account
        Ok(())
    }

    // TODO cancel_offer with CPI
}
