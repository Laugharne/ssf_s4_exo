# Solana Native Developpement

## Deployed program

**Program Id:** `TODO`

**On devnet : [Solana Explorer Link](https://solana.fm/address/TODO/transactions?cluster=devnet-solana)**

--------


## Overview

![](2024-08-11-14-07-55.png)

**Exercise:** Build a **native Solana program** to create/initialize an account, deposit SOL into it, and withdraw 10% of the deposited SOL at a given time from it.

Topics in this exercice :
- Rust
- PDA
- CPI
- SOL transfer
- Build native solana program & deploy on devnet

## Native Solana Project


```bash
cargo new my-solana-program --lib
cd my-solana-program
```

### **Add Solana dependancies**
In `Cargo.toml` file, add needed solana dependancies :

```toml
[dependencies]
solana-program = "1.18.17"
```

And libs...
```toml
[lib]
crate-type = ["cdylib", "lib"]
```


Replace `1.18.17` by the last stable needed version.

### **Compile the program**
Compile your program to generate a Solana-compatible binary.

```bash
cargo build-bpf
```

This will generate a `.so` file in the `target/deploy/` directory that is the program binary.

### **Deploy the program**
Deploy the compiled program to a Solana network (like devnet):

```bash
solana program deploy target/deploy/program.so
```

This will return the address of your program on Solana.


## Resources

- [How to write a Native Rust Program | Solana](https://solana.com/developers/guides/getstarted/intro-to-native-rust)
- [Native](https://pitch.com/v/native-c7gvwj)
- [Solana Playground | Solana IDE](https://beta.solpg.io/66b7a81ccffcf4b13384d2be)
- [program-examples/basics/program-derived-addresses/native/program/src/instructions/create.rs at main · solana-developers/program-examples · GitHub](https://github.com/solana-developers/program-examples/blob/main/basics/program-derived-addresses/native/program/src/instructions/create.rs)
- [rust - How to get the current time in Solana program without using any external SystemProgram account - Stack Overflow](https://stackoverflow.com/questions/72223450/how-to-get-the-current-time-in-solana-program-without-using-any-external-systemp)
- [Clock in anchor_lang::prelude - Rust](https://docs.rs/anchor-lang/latest/anchor_lang/prelude/struct.Clock.html)
- [Solana Bytes - Processing Instructions (Native) - YouTube](https://www.youtube.com/watch?v=T5p8rGD0-vs)
- [Solana Account Data Comparison -- Native vs. Anchor - YouTube](https://www.youtube.com/watch?v=71pkNLasq6c)
- [Working with Accounts in Rust Native Solana Programs [Solana Dev Course: M3 P3] - Dec 2nd '22 - YouTube](https://www.youtube.com/watch?v=Dg9p_JaqQQE)
