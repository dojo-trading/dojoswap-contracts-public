# DojoSwap
[![dojoswap on crates.io](https://img.shields.io/crates/v/dojoswap.svg)](https://crates.io/crates/dojoswap)
[![workflow](https://github.com/dojoswap/dojoswap/actions/workflows/tests.yml/badge.svg)](https://github.com/dojoswap/dojoswap/actions/workflows/tests.yml)
[![codecov](https://codecov.io/gh/dojoswap/dojoswap/branch/main/graph/badge.svg?token=ERMFLEY6Y7)](https://codecov.io/gh/dojoswap/dojoswap)

Uniswap-inspired automated market-maker (AMM) protocol powered by Smart Contracts on the [Injective](https://injective.com) blockchain.

## Contracts

| Name                                               | Description                                  |
| -------------------------------------------------- | -------------------------------------------- |
| [`dojoswap_factory`](contracts/dojoswap_factory) |                                              |
| [`dojoswap_pair`](contracts/dojoswap_pair)       |                                              |
| [`dojoswap_router`](contracts/dojoswap_router)   |                                              |
| [`dojoswap_token`](contracts/dojoswap_token)     | CW20 (ERC20 equivalent) token implementation |

* dojoswap_factory

   Mainnet: `inj1zfkh35ff8gmvczve27yr24u26cq0zmh723gh9v`

   Testnet: `inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra`

* dojoswap_pair

   <!-- Mainnet (CodeID): 5 -->

   Testnet (CodeID): 4697

* dojoswap_token

   <!-- Mainnet (CodeID): 4 -->

   Testnet (CodeID): 4441

* dojoswap_router

   Mainnet: `inj1yt3t5cuux2t3huxctwtfg0lmzvhkqm6uzrrmdv`

   Testnet: `inj1hz3qjp6sh36ch0kv6v6uj7z0ngucxj5mgetyqk`

## Running this contract

You will need Rust 1.44.1+ with wasm32-unknown-unknown target installed.

You can run unit tests on this on each contracts directory via :

```
cargo unit-test
cargo integration-test
```

Once you are happy with the content, you can compile it to wasm on each contracts directory via:

```
RUSTFLAGS='-C link-arg=-s' cargo wasm
cp ../../target/wasm32-unknown-unknown/release/cw1_subkeys.wasm .
ls -l cw1_subkeys.wasm
sha256sum cw1_subkeys.wasm
```

Or for a production-ready (compressed) build, run the following from the repository root:

```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer:0.12.6
```

The optimized contracts are generated in the artifacts/ directory.
