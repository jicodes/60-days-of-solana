use anchor_lang::prelude::*;

declare_id!("2JtdxDmXA2EdwRHh9916ew92L1apZJAuCbNPjNj6BiRR");

#[program]
pub mod day_03 {
    use super::*;

    pub fn add(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let sum = a + b;
        msg!("Sum is {}", sum);
        Ok(())
    }

    pub fn sub(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let difference = a - b;
        msg!("Difference is {}", difference);
        Ok(())
    }

    pub fn mul(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let product = a * b;
        msg!("Product is {}", product);
        Ok(())
    }

    pub fn div(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let quotient = a / b;
        msg!("Quotient is {}", quotient);
        Ok(())
    }

    pub fn modulo(ctx: Context<Initialize>, a: u64, b: u64) -> Result<()> {
        let remainder = a % b;
        msg!("Remainder is {}", remainder);
        Ok(())
    }

    pub fn non_empty_account_example(ctx: Context<NonEmptyAccountExample>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct NonEmptyAccountExample<'info> {
    signer: Signer<'info>,
    another_signer: Signer<'info>,

}
