# 2.9.0

* [25e24dd](https://github.com/dojoswap/dojoswap/pull/65/commits/25e24dd96edbe9b2c08523f581def313190feda5) feat: refund & desired asset to provide liqudity
* [f0b0a30](https://github.com/dojoswap/dojoswap/pull/65/commits/f0b0a30816ffed3d8aa7c702f2438a364e967c56) refactor: decimal loss is paid by the user
* [69dbc52](https://github.com/dojoswap/dojoswap/pull/65/commits/69dbc5231192ccac2916564b191039f7d476d309) feat: min_assets support when withdraw
* [3c433b9](https://github.com/dojoswap/dojoswap/pull/65/commits/3c433b9bca22609ffd388b3f2c29f134221005b0) feat: deadline support for swap, provide and withdraw
* [9700469](https://github.com/dojoswap/dojoswap/pull/65/commits/9700469218cff99d236a6e91bca48dd0ec7aae4d) refactor: maturity
* [56c28b0](https://github.com/dojoswap/dojoswap/pull/65/commits/56c28b0cf5400bba58bfa781c67ce209531309cb) refactor: define commission
* [37e83a0](https://github.com/dojoswap/dojoswap/pull/65/commits/37e83a0f54fd64c79ee937039e7e12fe079cf280) fix: revert slippage tolerance in provide liquidity
* [08da803](https://github.com/dojoswap/dojoswap/pull/65/commits/08da803eba5fc22c5b9859471feb1c448cb7e900) feat: initial liquidity deduction for pair protection
* [31b79c4](https://github.com/dojoswap/dojoswap/pull/65/commits/31b79c48621205c0db55c61e2b5237c0f0babd78) feat: create pair with provide liquidity

# 2.8.0

* [9ac5067](https://github.com/dojoswap/dojoswap/pull/55/commits/9ac50670e8bd20e00950e00b66a687e0a9d4fef9) Support to migrate pair

# 2.7.0

* [4e3e80e](https://github.com/dojoswap/dojoswap/pull/52/commits/4e3e80ea04b44e0396f8d03c306adf096a864573) Add native token decimals

# 2.6.2

* [5337ee7](https://github.com/dojoswap/dojoswap/pull/51/commits/5337ee7b54833346d7d50820b9f7236786c8329c) Clean up generic
* [c1ffc5d](https://github.com/dojoswap/dojoswap/pull/51/commits/c1ffc5dc87ecf63ba22e7f3afea6eb90d6be96c4) Use package in simulate

# 2.6.1

* [a422601](https://github.com/dojoswap/dojoswap/pull/49/commits/a4226011240a761f1fd7396263745c5cd6bc12b1) Refactoring querier

# 2.6.0

Terra V2

* [7c34483](https://github.com/dojoswap/dojoswap/pull/48/commits/7c344838a7fee8d3ff071b45ca5d27a3fe543379) Actual decimals in belief_price
* [ca549ae](https://github.com/dojoswap/dojoswap/pull/48/commits/ca549ae2ea36a83e81f5bc151b10ec8e4064be56) Add validation when create_pair
* [efe2ea0](https://github.com/dojoswap/dojoswap/pull/48/commits/efe2ea07d6cbcd0cfd34e8954cff24627d7406f2) Support to wasm 1.0.0
* [68b69cb](https://github.com/dojoswap/dojoswap/pull/48/commits/68b69cbb239b9e5a250127f562a32ec5bdceff81) Add to decimal in PairInfo
* [40ae47e](https://github.com/dojoswap/dojoswap/pull/48/commits/40ae47eed3daa223e06d1f6602fe34d6c23c17e1) Remove native_swap

# 2.5.1

* [2bfb0bb](https://github.com/dojoswap/dojoswap/pull/20/commits/82954c0aa289f12a3fe66df30cf1a65ce7bd4a4e) LOOP and ASTROPORT support on router

# 2.5.0

* [cd3cf2b](https://github.com/dojoswap/dojoswap/pull/30/commits/cd3cf2bb8d2438f5de4f5c1859b91fa46be85bf3) Support reverse simulate in router contract

# 2.4.1

* [191c1fb](https://github.com/dojoswap/dojoswap/pull/20/commits/191c1fb11e84771a022d793b70b9fe70988e50d3) Append `sender` and `receiver` event attributes to response.
* [f696e3b](https://github.com/dojoswap/dojoswap/pull/20/commits/f696e3b94d996ddf7fd10333519b82a904b834b1) Change github action's rust-optimizer to workspace-optimizer 

## Improvements 
InitHooks are removed from the contracts instantiate msg, instead it uses new reply feature of CosmWasm. 

# 2.4.0

Terraswap Initial Release
