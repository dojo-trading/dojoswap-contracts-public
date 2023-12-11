
## Build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.15.0
```

## Deploy
```
injectived tx wasm store ./artifacts/dojoswap_token-aarch64.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_pair-aarch64.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_factory-aarch64.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_router-aarch64.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443


```

## Codes

DojoswapToken - 4441
DojoswapPair - 4442
DojoswapFactory - 4443
DojoswapRouter - 4444

## Init

### Params

Factory
```
{"pair_code_id":4442,"token_code_id":4443}
```

Router
```
{"dojoswap_factory":"inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar"}
```

### Core Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=4444
export INIT='{"dojoswap_factory":"inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar"}'

injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://testnet.sentry.tm.injective.network:443
```

### Factory Settings
```sh
# send some native tokens as the factory contract will check for fund existence
injectived tx bank send inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar 100inj --node=https://testnet.sentry.tm.injective.network:443 --chain-id="injective-888" --gas-prices=500000000inj

# init native token
export CONTRACT=inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
CONFIG='{"add_native_token_decimals":{"denom": "inj", "decimals": 18}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--amount="1inj" \
--output json
```

### Token Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=4441
export INIT='{"name":"DojoSwap Token", "symbol": "DOJO", "decimals": 18, "initial_balances": [{"address": "inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt", "amount": "100000000000000000000000000000"}]}'

injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://testnet.sentry.tm.injective.network:443
```

### Staking Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=4627
export INIT='{"owner":"inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt", "reward_token_address": "inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy", "stake_token_address": "", "daily_reward_amount": "", "reward_interval": "", "lock_days": 0, "enabled": true}'
injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://testnet.sentry.tm.injective.network:443
```

### Create New Pair
```
export CONTRACT=inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"create_pair":{"assets":[{"info":{"token":{"contract_addr":"inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy"}},"amount":"1000000000000000000"},{"info":{"native_token":{"denom":"inj"}},"amount":"1000000000000000000"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--amount="1000000000000000000inj" \
--output json
```

### Addresses
Factory - inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar
Router - inj1d4qqdfpdp4mfpqrg5d5ka468gc3mvxrmhvfujd
Staking - 

DojoToken - inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy