# NOTES

## Quickstart

```
$ cargo install just
# just codama-create-idl
# just codama-generate-rs
```

## Development

### Rust

[https://rustup.rs/]()

```
$ rustc --version
rustc 1.87.0 (17067e9ac 2025-05-09)
```

### Solana CLI

[https://solana.com/docs/intro/installation#install-the-solana-cli]()

```
$ source ~/.profile
$ solana --version
solana-cli 2.1.22 (src:26944979; feat:1416569292, client:Agave)

$ solana config get
$ solana config set --url https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP

# solana config set --url mainnet-beta
```

## Bun

[https://bun.sh/]()

```
$ bun --version
1.2.15

$ bun install
```

## Star Atlas: Holosim

[https://explorer.atlasnet.staratlas.cloud/]()

`https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP`

### Holosim: Keypair (ID, Username and Password)

**Warning:** Do not use this keypair as a Solana mainnet wallet!

```
$ solana-keygen new -o ./vault/holosim_id.json
$ solana config set --keypair ./vault/holosim_id.json
```

### Holosim: Airdrop

```
$ solana airdrop 2
$ solana address
.. wallet_address ...
$ solana balance
2 SOL
```

### Programs

* Holosim - [SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF](https://explorer.atlasnet.staratlas.cloud/address/SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF)

```
$ solana program dump SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF programs/holosim/SAgeTraQfBMdvGVDJYoEvjnbq5szW7RJPi6obDTDQUF.so
```

## Rust Crates

* https://github.com/codama-idl/codama
