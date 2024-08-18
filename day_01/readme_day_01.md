# Setup Anchor and Solana

Make sure you have installed rust and solana cli before starting.
Version I am using:
- Rust: 1.8.1
- Anchor: 0.30.1
- Solana CLI: 1.18.18


Initialize and build an Anchor Program
```sh
anchor init day_1
cd day_1
anchor build
```

Configure Solana to run on localhost

```sh
# shell 1
solana config set --url localhost
```

Run the local test validator node

```sh
# shell 2
solana-test-validator
```

Sync the program_id with Anchor key

```sh
# shell 1
anchor keys sync
```

Realtime Solana logs

```sh
# shell 3
solana logs
```

Or find the log file by running

```sh
ls .anchor/program-logs/
```

Run the tests

```sh
# shell 1
anchor test --skip-local-validator
```

If you modified the program, before running the test again, kill the local
validator process and restart it with:

```sh
# shell 2
solana-test-validator --reset
```

Run the test again

```sh
# shell 1
anchor test --skip-local-validator
```

Clear the cache

```sh
rm -rf ~/.cache/solana/*
```

### Error shooting

If you encounter an error like:

```sh
> anchor build
...
=============================================================================
Error: Deploying program failed: RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: account data too small for instruction [3 log messages]
There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
```

Try running the following command:

```sh
rm -rf target/
```

And rebuild the program

```sh
anchor build
```
