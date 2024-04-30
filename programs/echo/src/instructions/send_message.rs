use crate::{Message, SendMessageContext, SEED_PREFIX_SENT};
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

/// This instruction posts a Wormhole message of some arbitrary size
/// in the form of bytes ([Vec<u8>]). The message is encoded as
/// [Message::Hello], which serializes a payload ID (1) before the message
/// specified in the instruction. Instead of using the native borsh
/// serialization of [Vec] length (little endian u32), length of the
/// message is encoded as big endian u16 (in EVM, bytes for numerics are
/// natively serialized as big endian).
///
/// See [Message] enum for serialization implementation.
///
/// # Arguments
///
/// * `message` - Arbitrary message to send out
pub fn send_message(ctx: Context<SendMessageContext>, message: Vec<u8>) -> Result<()> {
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

    // There is only one type of message that this example uses to
    // communicate with its foreign counterparts (payload ID == 1).
    let payload: Vec<u8> = Message::Hello { message }.try_to_vec()?;

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
