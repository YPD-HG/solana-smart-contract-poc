use anchor_lang::prelude::*;

declare_id!("H3LLo111111111111111111111111111111111111111");

#[program]
pub mod helloworld {
    use super::*;
    pub fn say_hello(_ctx: Context<Hello>) -> Result<()> {
        msg!("Hello, Solana + Anchor!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello {}
