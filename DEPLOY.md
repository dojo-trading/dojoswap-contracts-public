
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
--chain-id="injective-1" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_pair.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-1" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_factory.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-1" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_router.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-1" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://sentry.tm.injective.network:443

injectived tx wasm store ./artifacts/dojoswap_staking.wasm \
--from="inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt" \
--chain-id="injective-1" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://sentry.tm.injective.network:443


```

## Codes
Testnet
DojoswapToken - 4441
DojoswapPair - 4697
DojoswapFactory - 4682
DojoswapRouter - 4444
Multicall - 4783
Staking - 5053

Mainnet
DojoswapToken - 292
DojoswapPair - 290
DojoswapFactory - 289
DojoswapRouter - 291
Multicall - 293
Staking - 301

New DojoswapFactory - 305
New DojoswapPair - 306


## Init

### Params

Factory
```
{"pair_code_id":290,"token_code_id":292}
```

Router
```
{"dojoswap_factory":"inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk"}
```

Staking
```
{"dojo_token":"inj1l73x8hh6du0h8upp65r7ltzpj5twadtp5490n0", "staking_token": "inj15xk5d4d3we8z9s9avcqfns2xsrqq9u5mgaw6q6", "distribution_schedule": [[1703653200, 1704171600, "10000000000000000000000"]]}
```

### Core Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=301
export INIT='{"dojo_token":"inj1l73x8hh6du0h8upp65r7ltzpj5twadtp5490n0", "staking_token": "inj1ytl5y7plqak300e42akc3pzn2j9hp35lw2pv3k", "distribution_schedule": [[1703653200, 1704171600, "3000000000000000000000"]]}'

injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-1" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://sentry.tm.injective.network:443
```

### Factory Settings
```sh
# send some native tokens as the factory contract will check for fund existence
injectived tx bank send inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk 1peggy0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599 --node=https://sentry.tm.injective.network:443 --chain-id="injective-1" --gas-prices=500000000inj



# init native token
export CONTRACT=inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
CONFIG='{"add_native_token_decimals":{"denom": "peggy0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599", "decimals": 8}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json
```

### Token Init
```
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CODE_ID=292
export INIT='{"name":"Point Token", "symbol": "POINT", "decimals": 18, "initial_balances": [{"address": "inj1x26aln79hxrfm8c6v00208wlumrc0l6qtrtn8y", "amount": "100000000000000000000000000000"}]}'

injectived tx wasm instantiate $CODE_ID $INIT --label="Dojoswap Deployment" --from=$(echo $INJ_ADDRESS) --chain-id="injective-1" --yes --gas-prices=500000000inj --gas=20000000 --admin=$(echo $INJ_ADDRESS) --node=https://sentry.tm.injective.network:443
```

### Staking Init
```sh


# Updates distribution schedule
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONTRACT=inj1ewdcesmpdq5vz67a2rmfr682gjqecedymqm74f
export CONFIG='{"update_config": {"distribution_schedule": [[1703653200, 1704171600, "100000000000000000000000"]]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json


# sends reward tokens to contract
export INJ_ADDRESS=inj1x26aln79hxrfm8c6v00208wlumrc0l6qtrtn8y
export CONTRACT=inj1l73x8hh6du0h8upp65r7ltzpj5twadtp5490n0
export CONFIG='{"transfer":{"recipient": "inj1ewdcesmpdq5vz67a2rmfr682gjqecedymqm74f", "amount": "1000000000000000000000000000000"}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json

# sends reward tokens to contract
export CONTRACT=inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy
export CONFIG='{"send":{"contract": "inj18j3tn5hrf3uex5lw2egp5epl6xuwnmu2rt2k0z", "amount": "10000000000000000000000", "msg": ""}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json

```

### Create New Pair
```sh
# Denom-CW20 pair
export CONTRACT=inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"create_pair":{"assets":[{"info":{"token":{"contract_addr":"peggy0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"}},"amount":"0"},{"info":{"native_token":{"denom":"peggy0xdAC17F958D2ee523a2206206994597C13D831ec7"}},"amount":"0"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json

# Denom-Denom pair
export CONTRACT=inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"create_pair":{"assets":[{"info":{"native_token":{"denom":"peggy0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"}},"amount":"0"},{"info":{"native_token":{"denom":"peggy0xdAC17F958D2ee523a2206206994597C13D831ec7"}},"amount":"0"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json

# updating of configuration
export CONTRACT=inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"update_config": {"owner": "inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt", "token_code_id": 4441, "pair_code_id": 4697}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json
```


### Provide Liquidity
```sh
export CONTRACT=inj1h0mpv48ctcsmydymh2hnkal7hla5gl4gftemqv
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{"provide_liquidity":{"assets":[{"info":{"native_token":{"denom":"peggy0xdAC17F958D2ee523a2206206994597C13D831ec7"}},"amount":"40200"},{"info":{"native_token":{"denom":"inj"}},"amount":"1000000000000000"}]}}'
injectived tx wasm execute $CONTRACT "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--amount="40200peggy0xdAC17F958D2ee523a2206206994597C13D831ec7,1000000000000000inj" \
--output json
```

### Addresses
Testnet
Factory - inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
Router - inj18em4c6qvzc6kymk98p5j7dxdm4at6d9qy8wxqv
Staking - 
Multicall - inj1q54g9rhprqd5v87we3rrjhp2etpxx2w6qys6ku
PointToken - inj1375v9e5awxf340cgv2pzh4seu074lxw0d092gd
INJ-USDT Staking - inj1624rmgycjv8xfyu4xeprguju2smcq46c94x8hd

DojoToken - inj16l3txcsmjcs6nrc3s0se0388r39j8wn73n45cy
TestToken - inj1e8ppkcdttmvqywcx84rjqf0l2x9gcutlmft4l0

Mainnet

Factory - inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
PointToken - inj1l73x8hh6du0h8upp65r7ltzpj5twadtp5490n0

#### Get Config
```
injectived query wasm contract-state smart inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk '{"config": {}}' --node=https://sentry.tm.injective.network:443


injectived query wasm contract-state smart inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk '{"pairs": {}}' --node=https://sentry.tm.injective.network:443

injectived query wasm contract-state smart inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk '{"native_token_decimals": {"denom": "peggy0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"}}' --node=https://sentry.tm.injective.network:443

injectived query wasm contract-state smart inj1l73x8hh6du0h8upp65r7ltzpj5twadtp5490n0 '{"minter": {}}' --node=https://sentry.tm.injective.network:443
```


### Migrate
```sh
export CONTRACT=inj1pc2vxcmnyzawnwkf03n2ggvt997avtuwagqngk
export INJ_ADDRESS=inj12qy3algm6e0zdpv8zxvauzquumuvd39ccdcdjt
export CONFIG='{}'
export CODE_ID=305
injectived tx wasm migrate $CONTRACT $CODE_ID "$CONFIG" --from=$(echo $INJ_ADDRESS) \
--chain-id="injective-1" \
--yes --fees=1000000000000000inj --gas=2000000 \
--node="https://sentry.tm.injective.network:443" \
--output json
```