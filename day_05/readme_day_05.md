# [Solana Anchor program deployment](https://www.rareskills.io/post/solana-anchor-deploy)


### Deployment step
- Build the program to get the binary
```sh
anchor build
```

- Deploy the program
```sh
anchor deploy
```
If you rerun it again, it will redeploy the program at the same address(overwrite the previous program)


### Solana programs do not have constructors

- Instead, they have an `initialize` method that is called after the program is deployed

### Solana programs are mutable by default
### Solana does not have immutable variables

### Running tests without redeploying the program
```sh
anchor test --skip-local-validator --skip-deploy
```

