# Stylus Uniswap Workshop

## Deploy

```bash
cargo stylus deploy \
  -e=$RPC_URL \
  --private-key=$PRIV_KEY \
  --wasm-file=$WASM_FILE \
  --no-verify \
  --deployer-address=$DEPLOYER_ADDRESS \
  --constructor-signature 'constructor(string)' \
  --constructor-args <VERSION>
```

## Version Call

```bash
cast call <CONTRACT_ADDRESS> "version()(string)" --rpc-url $RPC_URL
```

## Get Amount In For Exact Output

```bash
cast call <CONTRACT_ADDRESS> "getAmountInForExactOutput(uint256,address,address,bool)(uint256)" <amountOut> <input> <output> <zeroForOne> --rpc-url $RPC_URL
```

## Get Amount Out For Exact Input

```bash
cast call <CONTRACT_ADDRESS> "getAmountOutFromExactInput(uint256,address,address,bool)(uint256)" <amountIn> <input> <output>  <zeroForOne> --rpc-url $RPC_URL
```
