use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;
use wormhole_anchor_sdk::wormhole::CHAIN_ID_SOLANA;

use crate::{
    Address, Message, OfferCreatedMessage, OfferItem, OfferItems, SendMessageContext,
    SEED_PREFIX_SENT,
};

/// * `message` - Arbitrary message to send out
pub fn send_message(ctx: Context<SendMessageContext>) -> Result<()> {
    // If Wormhole requires a fee before posting a message, we need to
    // transfer lamports to the fee collector. Otherwise
    // `wormhole::post_message` will fail.
    let fee = ctx.accounts.wormhole_bridge.fee();
    if fee > 0 {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                &ctx.accounts.payer.key(),
                &ctx.accounts.wormhole_fee_collector.key(),
                fee,
            ),
            &ctx.accounts.to_account_infos(),
        )?;
    }

    // post message
    let wormhole_emitter = &ctx.accounts.wormhole_emitter;
    let config = &ctx.accounts.config;

    // TODO
    let solana_address = Pubkey::new_unique();
    let address = Address {
        chain_id: CHAIN_ID_SOLANA,
        solana_address: Some(solana_address),
        eth_address: None,
    };
    let offer_item = OfferItem {
        address: address.clone(),
        token_id: None,
    };
    let items = vec![offer_item.clone()];
    let offer_items = OfferItems {
        count: 1,
        items: items.clone(),
    };
    let message = OfferCreatedMessage {
        id: [0u8; 32],
        sender: address.clone(),
        receiver: address.clone(),
        sender_items: offer_items.clone(),
        receiver_items: offer_items.clone(),
        expiration: 0,
    };
    let payload: Vec<u8> = Message::OfferCreated { message }.try_to_vec()?;

    wormhole::post_message(
        CpiContext::new_with_signer(
            ctx.accounts.wormhole_program.to_account_info(),
            wormhole::PostMessage {
                config: ctx.accounts.wormhole_bridge.to_account_info(),
                message: ctx.accounts.wormhole_message.to_account_info(),
                emitter: wormhole_emitter.to_account_info(),
                sequence: ctx.accounts.wormhole_sequence.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                fee_collector: ctx.accounts.wormhole_fee_collector.to_account_info(),
                clock: ctx.accounts.clock.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[
                &[
                    SEED_PREFIX_SENT,
                    &ctx.accounts.wormhole_sequence.next_value().to_be_bytes()[..],
                    &[ctx.bumps.wormhole_message],
                ],
                &[wormhole::SEED_PREFIX_EMITTER, &[wormhole_emitter.bump]],
            ],
        ),
        config.batch_id,
        payload,
        config.finality.try_into().unwrap(),
    )?;

    Ok(())
}
