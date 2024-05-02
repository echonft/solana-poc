use crate::{constants::SEED_PREFIX_SENT, program_accounts::Config, WormholeEmitter};
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

#[derive(Accounts)]
pub struct InitializeContext<'info> {
    #[account(mut)]
    /// Whoever initializes the config will be the owner of the program. Signer
    /// for creating the [`Config`] account and posting a Wormhole message
    /// indicating that the program is alive.
    pub owner: Signer<'info>,
    #[account(
    init,
    payer = owner,
    seeds = [Config::SEED_PREFIX],
    bump,
    space = Config::MAXIMUM_SIZE,
    )]
    /// Config account, which saves program data useful for other instructions.
    /// Also saves the payer of the [`initialize`](crate::initialize) instruction
    /// as the program's owner.
    pub config: Account<'info, Config>,
    /// Wormhole program.
    pub wormhole_program: Program<'info, wormhole::program::Wormhole>,
    #[account(
    mut,
    seeds = [wormhole::BridgeData::SEED_PREFIX],
    bump,
    seeds::program = wormhole_program,
    )]
    /// Wormhole bridge data account (a.k.a. its config).
    /// [`wormhole::post_message`] requires this account be mutable.
    pub wormhole_bridge: Account<'info, wormhole::BridgeData>,
    #[account(
    mut,
    seeds = [wormhole::FeeCollector::SEED_PREFIX],
    bump,
    seeds::program = wormhole_program
    )]
    /// Wormhole fee collector account, which requires lamports before the
    /// program can post a message (if there is a fee).
    /// [`wormhole::post_message`] requires this account be mutable.
    pub wormhole_fee_collector: Account<'info, wormhole::FeeCollector>,
    #[account(
    init,
    payer = owner,
    seeds = [WormholeEmitter::SEED_PREFIX],
    bump,
    space = WormholeEmitter::MAXIMUM_SIZE
    )]
    /// This program's emitter account. We create this account in the
    /// [`initialize`](crate::initialize) instruction, but
    /// [`wormhole::post_message`] only needs it to be read-only.
    pub wormhole_emitter: Account<'info, WormholeEmitter>,
    #[account(
    mut,
    seeds = [
    wormhole::SequenceTracker::SEED_PREFIX,
    wormhole_emitter.key().as_ref()
    ],
    bump,
    seeds::program = wormhole_program
    )]
    /// CHECK: Emitter's sequence account. This is not created until the first
    /// message is posted, so it needs to be an [UncheckedAccount] for the
    /// [`initialize`](crate::initialize) instruction.
    /// [`wormhole::post_message`] requires this account be mutable.
    pub wormhole_sequence: UncheckedAccount<'info>,
    #[account(
    mut,
    seeds = [
    SEED_PREFIX_SENT,
    &wormhole::INITIAL_SEQUENCE.to_be_bytes()[..]
    ],
    bump,
    )]
    /// CHECK: Wormhole message account. The Wormhole program writes to this
    /// account, which requires this program's signature.
    /// [`wormhole::post_message`] requires this account be mutable.
    pub wormhole_message: UncheckedAccount<'info>,
    /// Clock sysvar.
    pub clock: Sysvar<'info, Clock>,
    /// Rent sysvar.
    pub rent: Sysvar<'info, Rent>,
    /// System program.
    pub system_program: Program<'info, System>,
}
