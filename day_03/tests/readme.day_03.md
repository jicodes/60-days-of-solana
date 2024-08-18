### What is #[derive(Accounts)] struct?

- `#[derive(Accounts)]` is a macro that generates a struct that contains all the
  accounts that are used in the function. This is useful for passing the
  accounts to the function.

### What is #[account] attribute?

- `#[account]` is an attribute that is used to specify the account that is used
  in the function.
- There is no need to specify

### Accounts IDL key

```json
{
  "name": "accounts",
  "accounts": [
    {
      "name": "signer",
      "isSigner": false
    },
    {
      "name": "another_signer",
      "isSigner": false
    }
  ],
  "args": []
}
```
`args` is an empty array because the function does not take any arguments except the accounts.

### ctx: Context\<StructName\>

- The `ctx` is a `Context` struct that is used to pass the accounts to the
  function.

### Summary:

- Solana uses an IDL (iInterface Definition Language) to show how to interact
  with a Solana program and what fields appear in the IDL.
- The struct modified by #[derive(Accounts)] and how it relates to function
  arguments.
- Anchor interprets `snake_case` functions in Rust as `camelCase` functions in
  the Typescript tests.
