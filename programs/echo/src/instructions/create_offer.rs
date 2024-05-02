use crate::{CreateOfferContext, OfferState};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

pub fn create_offer(ctx: Context<CreateOfferContext>) -> Result<()> {
    // set the offer account
    {
        let offer = &mut ctx.accounts.offer;
        offer.sender = ctx.accounts.sender.key();
        offer.receiver = ctx.accounts.receiver.key();
        offer.sender_token_mint = ctx.accounts.token_mint.key();
        offer.receiver_token_mint = ctx.accounts.receiver_token_mint.key();
        offer.state = OfferState::OPEN;
        offer.bump = ctx.bumps.offer;
    }

    // set the escrow account
    {
        let escrow = &mut ctx.accounts.escrow;
        escrow.owner = ctx.accounts.sender.key();
        escrow.mint = ctx.accounts.token_mint.key();
        escrow.offer = ctx.accounts.offer.key();
        escrow.bump = ctx.bumps.escrow;
    }

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

    Ok(())
}
