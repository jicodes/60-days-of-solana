use anchor_lang::prelude::*;

declare_id!("9NLWS6RZkiAfmQiuFP59mCASTCg9BeLr3voLRPAym39L");

#[program]
pub mod day_01 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello, world!");
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
