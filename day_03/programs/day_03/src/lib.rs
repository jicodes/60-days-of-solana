use anchor_lang::prelude::*;

declare_id!("3n2rbjuvX9uKz6BH9wNNUzeG8qUQiDJnQt2Mo37dn4Yp");

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
}

#[derive(Accounts)]
pub struct Initialize {}
