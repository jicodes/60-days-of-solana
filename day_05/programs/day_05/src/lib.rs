use anchor_lang::prelude::*;

declare_id!("6P7tLp524S6RtqQvrzcLLDTWidpyq7UxeByTGhXMrqcv");

#[program]
pub mod day_05 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Update the program");
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
