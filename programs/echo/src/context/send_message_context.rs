use crate::{
    constants::SEED_PREFIX_SENT,
    error::EchoError,
    program_accounts::{Config, ForeignEmitter, Received, WormholeEmitter},
};
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

#[derive(Accounts)]
pub struct SendMessageContext<'info> {
    #[account(mut)]
    /// Payer will pay Wormhole fee to post a message.
    pub payer: Signer<'info>,

    #[account(
    seeds = [Config::SEED_PREFIX],
    bump,
    )]
    /// Config account. Wormhole PDAs specified in the config are checked
    /// against the Wormhole accounts in this context. Read-only.
    pub config: Account<'info, Config>,

    /// Wormhole program.
    pub wormhole_program: Program<'info, wormhole::program::Wormhole>,

    #[account(
    mut,
    address = config.wormhole.bridge @ EchoError::InvalidWormholeConfig
    )]
    /// Wormhole bridge data. [`wormhole::post_message`] requires this account
    /// be mutable.
    pub wormhole_bridge: Account<'info, wormhole::BridgeData>,

    #[account(
    mut,
    address = config.wormhole.fee_collector @ EchoError::InvalidWormholeFeeCollector
    )]
    /// Wormhole fee collector. [`wormhole::post_message`] requires this
    /// account be mutable.
    pub wormhole_fee_collector: Account<'info, wormhole::FeeCollector>,

    #[account(
    seeds = [WormholeEmitter::SEED_PREFIX],
    bump,
    )]
    /// Program's emitter account. Read-only.
    pub wormhole_emitter: Account<'info, WormholeEmitter>,

    #[account(
    mut,
    address = config.wormhole.sequence @ EchoError::InvalidWormholeSequence
    )]
    /// Emitter's sequence account. [`wormhole::post_message`] requires this
    /// account be mutable.
    pub wormhole_sequence: Account<'info, wormhole::SequenceTracker>,

    #[account(
    mut,
    seeds = [
    SEED_PREFIX_SENT,
    &wormhole_sequence.next_value().to_be_bytes()[..]
    ],
    bump,
    )]
    /// CHECK: Wormhole Message. [`wormhole::post_message`] requires this
    /// account be mutable.
    pub wormhole_message: UncheckedAccount<'info>,

    /// System program.
    pub system_program: Program<'info, System>,

    /// Clock sysvar.
    pub clock: Sysvar<'info, Clock>,

    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,
}
