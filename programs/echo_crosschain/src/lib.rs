use anchor_lang::prelude::*;

declare_id!("FPJtzVRzQmKc1eNBCr2FzBT5UDX7b7TrRoaY59gVUE8h");

#[program]
pub mod echo_crosschain {
    use super::*;
    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
