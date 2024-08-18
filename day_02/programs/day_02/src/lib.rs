use anchor_lang::prelude::*;

declare_id!("GAwXKQa3DuLdB7VbTF1TUD31U6WQQFomDDYcQZA4Yb5x");

#[program]
pub mod day_02 {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>, a: u64, b: u64, message: String) -> Result<()> {
        msg!("You said {:?}", message);
        msg!("You sent {} and {}", a, b);
        Ok(())
    }

    pub fn array(_ctx: Context<Initialize>, arr: Vec<u64>) -> Result<()> {
        msg!("Your array {:?}", arr);
        Ok(())
    }
    // build a calculator: add, subtract, multiply, divide, sqrt and log10
    pub fn add(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<u64> {
        msg!("Adding {} and {}", a, b);
        Ok(a + b)
    }

    pub fn subtract(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<u64> {
        msg!("Subtracting {} and {}", a, b);
        Ok(a - b)
    }

    pub fn multiply(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<u64> {
        msg!("Multiplying {} and {}", a, b);
        Ok(a * b)
    }

    pub fn divide(_ctx: Context<Initialize>, a: u64, b: u64) -> Result<u64> {
        msg!("Dividing {} and {}", a, b);
        if b == 0 {
            return Err(MathError::DivisionByZero.into());
        }
        Ok(a / b)
    }

    pub fn sqrt(_ctx: Context<Initialize>, a: u64) -> Result<u64> {
        msg!("Square root of {}", a);
        let sqrt_value = (a as f64).sqrt() as u64;
        Ok(sqrt_value)
    }

    pub fn log10(_ctx: Context<Initialize>, a: u64) -> Result<f64> {
        msg!("Log10 of {}", a);
        Ok((a as f64).log10())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[error_code]
pub enum MathError {
    DivisionByZero,
}