use crate::{EchoError, Message, ReceiveMessageContext, MESSAGE_MAX_LENGTH};
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

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

    if let Message::Hello { message } = posted_message.data() {
        // HelloWorldMessage cannot be larger than the maximum size of the account.
        require!(
            message.len() <= MESSAGE_MAX_LENGTH,
            EchoError::InvalidMessage,
        );

        // Save batch ID, keccak256 hash and message payload.
        let received = &mut ctx.accounts.received;
        received.batch_id = posted_message.batch_id();
        received.wormhole_message_hash = vaa_hash;
        received.message = message.clone();

        Ok(())
    } else {
        Err(EchoError::InvalidMessage.into())
    }
}
