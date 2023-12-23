
## Build
```
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.15.0
```

## Deploy
```
injectived tx wasm store ./artifacts/dojoswap_token.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_pair.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_factory.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_router.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443


```

## Codes

DojoswapToken - 4441
Old DojoswapPair - 4442
DojoswapPair - 4697
Old DojoswapFactory - 4443
DojoswapFactory - 4682
DojoswapRouter - 4444
Multicall - 4783

## Init

### Params

Factory
```
{"pair_code_id":4442,"token_code_id":4443}
```

Router
```
{"dojoswap_factory":"inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra"}
```

Staking
```
{"dojo_token":"inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy", "staking_token": "inj1knxx8crp0auxyvx8hn3jjl7htfsandx5xf0n8q", "distribution_schedule": [1703351907, 1705943907, 1000000000000000000000]}
```

### Core Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=4444
export INIT='{"dojoswap_factory":"inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra"}'

injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://testnet.sentry.tm.injective.network:443
```

### Factory Settings
```sh
# send some native tokens as the factory contract will check for fund existence
injectived tx bank send inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra 100peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5 --node=https://testnet.sentry.tm.injective.network:443 --chain-id="injective-888" --gas-prices=500000000inj


# init native token
export CONTRACT=inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
CONFIG='{"add_native_token_decimals":{"denom": "peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5", "decimals": 6}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
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
```sh
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=4627
export INIT='{"owner":"inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt", "reward_token_address": "inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy", "stake_token_address": "inj1knxx8crp0auxyvx8hn3jjl7htfsandx5xf0n8q", "daily_reward_amount": "100000000000000000000", "reward_interval": 86400, "lock_days": 1, "enabled": true}'
injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://testnet.sentry.tm.injective.network:443


# sends reward tokens to contract
export CONTRACT=inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy
export CONFIG='{"send":{"contract": "inj18j3tn5hrf3uex5lw2egp5epl6xuwnmu2rt2k0z", "amount": "10000000000000000000000", "msg": ""}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--output json

```

### Create New Pair
```sh
# Denom-CW20 pair
export CONTRACT=inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"create_pair":{"assets":[{"info":{"token":{"contract_addr":"inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy"}},"amount":"0"},{"info":{"native_token":{"denom":"inj"}},"amount":"0"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--output json

# Denom-Denom pair
export CONTRACT=inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"create_pair":{"assets":[{"info":{"native_token":{"denom":"peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5"}},"amount":"0"},{"info":{"native_token":{"denom":"inj"}},"amount":"0"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--output json

# updating of configuration
export CONTRACT=inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"update_config": {"owner": "inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt", "token_code_id": 4441, "pair_code_id": 4697}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--output json
```


### Provide Liquidity
```sh
export CONTRACT=inj15hzua8ldd90h6l6pc93qgand5vzhqll7gp4eh6
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"provide_liquidity":{"assets":[{"info":{"native_token":{"denom":"peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5"}},"amount":"2500000"},{"info":{"native_token":{"denom":"inj"}},"amount":"100000000000000000"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://testnet.sentry.tm.injective.network:443" \
--amount="2500000peggy0x87aB3B4C8661e07D6372361211B96ed4Dc36B1B5,100000000000000000inj" \
--output json
```

### Addresses
Old Factory - inj1x92gnfdkk6glr733lrrkkstpdavgsxyy5ze9ar
Factory - inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra
Old Router - inj1d4qqdfpdp4mfpqrg5d5ka468gc3mvxrmhvfujd
Router - inj18em4c6qvzc6kymk98p5j7dxdm4at6d9qy8wxqv
Staking - 
Multicall - inj1q54g9rhprqd5v87we3rrjhp2etpxx2w6qys6ku

DojoToken - inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy


#### Get Config
```
injectived query wasm contract-state smart inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra '{"config": {}}' --node=https://testnet.sentry.tm.injective.network:443


injectived query wasm contract-state smart inj13x5m8l5whmhvv2le44aq0j6ukr8h5xh8atxpra '{"pairs": {}}' --node=https://testnet.sentry.tm.injective.network:443
```

