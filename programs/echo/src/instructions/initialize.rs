use crate::InitializeContext;
use anchor_lang::prelude::*;
use wormhole_anchor_sdk::wormhole;

pub fn initialize(ctx: Context<InitializeContext>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.owner = ctx.accounts.owner.key();
    // Set Wormhole related addresses.
    {
        let wormhole = &mut config.wormhole;
        wormhole.bridge = ctx.accounts.wormhole_bridge.key();
        wormhole.fee_collector = ctx.accounts.wormhole_fee_collector.key();
        wormhole.sequence = ctx.accounts.wormhole_sequence.key();
    }
    // Zero means no batching.
    config.batch_id = 0;
    // Anchor IDL default coder cannot handle wormhole::Finality enum,
    // so this value is stored as u8.
    config.finality = wormhole::Finality::Confirmed as u8;
    ctx.accounts.wormhole_emitter.bump = ctx.bumps.wormhole_emitter;
    Ok(())
}
