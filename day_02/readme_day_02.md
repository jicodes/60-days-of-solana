# Math in Solana

### Floating point math: 
- computationally expensive
- best avoided

### Arithmetic Overflow

#### How does Solana defend against arithmetic overflow?
- Method 1: `overflow-checks = true` in Cargo.toml
- Method 2: using checked_* operators.
```rust
// will silently overflow
let x: u64 = y + z; 

// will panic if overflow happens
let xSafe: u64 = y.checked_add(z).unwrap(); 

// checked_sub, checked_mul, etc are also available
```


### Solana compute units 101
In Ethereum, a transaction runs until it consumes the “gas limit” specified by the transaction. 
Solana calls "gas" a "compute unit". 
By default, a transaction is limited to `200,000` compute units. If more than 200,000 compute units are consumed, the transaction reverts.


### Powers in Solana(Rust)
```rust
let x: u64 = 2; // it is important that the base's data type is explicit
let y = 3; // the exponent data type can be inferred
let result = x.pow(y);

// use `.checked_pow` if you are concerned about overflow.
```