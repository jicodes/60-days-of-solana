# Require, Revert, and Custom Errors in Solana

The significant difference between how Ethereum and Solana stops transactions with invalid parameters is that 
- Ethereum triggers a `revert` and 
- Solana returns an `error`.

### require! macro
- syntax sugar for returning an error if the condition is not met.
```rust
pub fn func(ctx: Context<ReturnError>) -> Result<()> {
		msg!("Will this print?");
		return err!(Day04Error::AlwaysErrors);
}
```
- Return type of `Result<()>` could either be an Ok(()) or an error.
- If the condition is not met, the `require!` macro will return an error. It won't print the message.
- If use Ok(()) instead of return, the message will be printed.

### Differences between Solana and Solidity with regards to errors

- In Solidity, if a transaction fails, it will revert the transaction.
- Solana does not halt execution but simply returns a different value (Error) instead of Ok.

### Error Handling in Anchor
- In Anchor, errors are an enum with the #[error_code] attribute.

```rust
#[error]
pub enum Day04Error {
    #[msg("Always errors")]
    AlwaysErrors,
}
```
