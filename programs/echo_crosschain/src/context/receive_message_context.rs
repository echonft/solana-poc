use crate::{
    error::EchoCrosschainError,
    message::Message,
    state::{Config, ForeignEmitter, Received, WormholeEmitter},
};
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

#[derive(Accounts)]
#[instruction(vaa_hash: [u8; 32])]
pub struct ReceiveMessageContext<'info> {
    #[account(mut)]
    /// Payer will initialize an account that tracks his own message IDs.
    pub payer: Signer<'info>,

    #[account(
    seeds = [Config::SEED_PREFIX],
    bump,
    )]
    /// Config account. Wormhole PDAs specified in the config are checked
    /// against the Wormhole accounts in this context. Read-only.
    pub config: Account<'info, Config>,

    // Wormhole program.
    pub wormhole_program: Program<'info, wormhole::program::Wormhole>,

    #[account(
    seeds = [
    wormhole::SEED_PREFIX_POSTED_VAA,
    &vaa_hash
    ],
    bump,
    seeds::program = wormhole_program
    )]
    /// Verified Wormhole message account. The Wormhole program verified
    /// signatures and posted the account data here. Read-only.
    pub posted: Account<'info, wormhole::PostedVaa<Message>>,

    #[account(
    seeds = [
    ForeignEmitter::SEED_PREFIX,
    &posted.emitter_chain().to_le_bytes()[..]
    ],
    bump,
    constraint = foreign_emitter.verify(posted.emitter_address()) @ EchoCrosschainError::InvalidForeignEmitter
    )]
    /// Foreign emitter account. The posted message's `emitter_address` must
    /// agree with the one we have registered for this message's `emitter_chain`
    /// (chain ID). Read-only.
    pub foreign_emitter: Account<'info, ForeignEmitter>,

    #[account(
    init,
    payer = payer,
    seeds = [
    Received::SEED_PREFIX,
    &posted.emitter_chain().to_le_bytes()[..],
    &posted.sequence().to_le_bytes()[..]
    ],
    bump,
    space = Received::MAXIMUM_SIZE
    )]
    /// Received account. [`receive_message`](crate::receive_message) will
    /// deserialize the Wormhole message's payload and save it to this account.
    /// This account cannot be overwritten, and will prevent Wormhole message
    /// replay with the same sequence.
    pub received: Account<'info, Received>,

    /// System program.
    pub system_program: Program<'info, System>,
}