# NOTES

## Quickstart

```bash
$ cargo install just
# just codama-create-idl
# just codama-generate-rs
```

## Development

### Rust

[https://rustup.rs/]()

```bash
$ rustc --version
rustc 1.89.0 (29483883e 2025-08-04)
```

### Solana CLI

[https://solana.com/docs/intro/installation#install-the-solana-cli]()

```bash
# agave-install update
$ source ~/.profile
$ solana --version
solana-cli 2.3.8 (src:72664e23; feat:3640012085, client:Agave)

$ solana config get
$ solana config set --url https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP

# solana config set --url mainnet-beta
```

## Bun

[https://bun.sh/]()

```bash
# bun update
$ bun --version
1.2.15

$ bun install
```

## Star Atlas: Holosim

[https://explorer.atlasnet.staratlas.cloud/]()

`https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP`

### Holosim: Keypair (ID, Username and Password)

**Warning:** Do not use this keypair as a Solana mainnet wallet!

```bash
$ solana-keygen new -o ./vault/holosim_id.json
$ solana config set --keypair ./vault/holosim_id.json
```

### Holosim: Airdrop

```bash
$ solana airdrop 2
$ solana address
.. wallet_address ...
$ solana balance
2 SOL
```

### Programs

[Star Atlas Build: Resources for Builders](https://build.staratlas.com/)

* Holosim - [SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9](https://explorer.atlasnet.staratlas.cloud/address/SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9)
* Player Profile - [PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ](https://explorer.atlasnet.staratlas.cloud/address/PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ)
  - note: idl.json downloaded from mainnet `pprofELXjL5Kck7Jn5hCpwAL82DpTkSYBENzahVtbc9`
* Profile Faction - [pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj](https://explorer.atlasnet.staratlas.cloud/address/pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj)
  - note: idl.json downloaded from mainnet `pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq`

```bash
$ solana program dump SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9 programs/holosim/SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9.so
$ solana program dump PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ programs/player_profile/PprofUW1pURCnMW2si88GWPXEEK3Bvh9Tksy8WtnoYJ.so
$ solana program dump pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj programs/profile_faction/pFACzkX2eSpAjDyEohD6i3VRJvREtH9ynbtM1DwVFsj.so
```

## Star Atlas: C4 Sage

[https://explorer.atlasnet.staratlas.cloud/]()

`https://rpc.ironforge.network/devnet?apiKey=01JEB7YQ0YPK31WQTC0VQ5Y9YP`

* C4 Sage - [C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF](https://explorer.atlasnet.staratlas.cloud/address/C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF)

```bash
$ solana program dump C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF programs/c4_sage/C4SAgeKLgb3pTLWhVr6NRwWyYFuTR7ZeSXFrzoLwfMzF.so
```

## Rust Crates

* https://github.com/codama-idl/codama
