use anchor_lang::prelude::*;
pub use constants::*;
pub use context::*;
pub use error::*;
pub use message::*;
pub use state::*;

pub mod constants;
pub mod context;
pub mod error;
pub mod message;
pub mod state;

declare_id!("FPJtzVRzQmKc1eNBCr2FzBT5UDX7b7TrRoaY59gVUE8h");
#[program]
pub mod echo_crosschain {
    use super::*;
    use anchor_lang::solana_program;
    use wormhole_anchor_sdk::wormhole;

    /// This instruction initializes the program config, which is meant
    /// to store data useful for other instructions. The config specifies
    /// an owner (e.g. multisig) and should be read-only for every instruction
    /// in this example. This owner will be checked for designated owner-only
    /// instructions like [`register_emitter`](register_emitter).
    ///
    /// # Arguments
    ///
    /// * `ctx` - `Initialize` context
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

    /// This instruction registers a new foreign emitter (from another network)
    /// and saves the emitter information in a ForeignEmitter account. This
    /// instruction is owner-only, meaning that only the owner of the program
    /// (defined in the [Config] account) can add and update emitters.
    ///
    /// # Arguments
    ///
    /// * `ctx`     - `RegisterForeignEmitter` context
    /// * `chain`   - Wormhole Chain ID
    /// * `address` - Wormhole Emitter Address
    pub fn register_emitter(
        ctx: Context<RegisterEmitterContext>,
        chain: u16,
        address: [u8; 32],
    ) -> Result<()> {
        // Foreign emitter cannot share the same Wormhole Chain ID as the
        // Solana Wormhole program's. And cannot register a zero address.
        require!(
            chain > 0 && chain != wormhole::CHAIN_ID_SOLANA && !address.iter().all(|&x| x == 0),
            EchoCrosschainError::InvalidForeignEmitter,
        );

        let emitter = &mut ctx.accounts.foreign_emitter;
        emitter.chain = chain;
        emitter.address = address;

        Ok(())
    }

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
                        &ctx.accounts.wormhole_sequence.next_value().to_le_bytes()[..],
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
                EchoCrosschainError::InvalidMessage,
            );

            // Save batch ID, keccak256 hash and message payload.
            let received = &mut ctx.accounts.received;
            received.batch_id = posted_message.batch_id();
            received.wormhole_message_hash = vaa_hash;
            received.message = message.clone();

            Ok(())
        } else {
            Err(EchoCrosschainError::InvalidMessage.into())
        }
    }
}
