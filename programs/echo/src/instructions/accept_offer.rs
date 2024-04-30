use crate::{AcceptOfferContext, SEED_PREFIX_ESCROW};
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Transfer};

pub fn accept_offer(ctx: Context<AcceptOfferContext>) -> Result<()> {
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
                SEED_PREFIX_ESCROW,
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
