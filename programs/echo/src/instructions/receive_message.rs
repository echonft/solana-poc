use crate::ReceiveMessageContext;
use anchor_lang::prelude::*;

/// This instruction reads a posted verified Wormhole message and verifies
/// that the payload is of type [Message::Hello] (payload ID == 1). HelloWorldMessage
/// data is stored in a [Received] account.
///
/// See [Message] enum for deserialization implementation.
///
/// # Arguments
///
/// * `vaa_hash` - Keccak256 hash of verified Wormhole message
pub fn receive_message(ctx: Context<ReceiveMessageContext>, vaa_hash: [u8; 32]) -> Result<()> {
    let posted_message = &ctx.accounts.posted;
    let message = posted_message.data();
    // TODO
    // message cannot be larger than the maximum size of the account
    // require!(
    //     message.len() <= MESSAGE_MAX_LENGTH,
    //     EchoError::InvalidMessage,
    // );
    {
        let received = &mut ctx.accounts.received;
        received.batch_id = posted_message.batch_id();
        received.wormhole_message_hash = vaa_hash;
        // TODO
        // received.message = message.clone();
    }
    msg!("received msg: {:?}", message);
    // TODO
    Ok(())
}
