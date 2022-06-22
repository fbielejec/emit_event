## Prerequisites
cargo install --git https://github.com/paritytech/cargo-contract.git --rev 23f62e93a398750d4bd6e8f34ba1100c395ec72a  --force

## Reproduce

```bash
cd contract-a/
cargo contract build --release

export CODE_HASH=$(cargo contract upload --url ws://127.0.0.1:9943 --suri //Alice)
export CODE_HASH=$(echo "$CODE_HASH" | grep hash | tail -1 | cut -c 15-)

echo $CODE_HASH

cargo contract instantiate --url ws://127.0.0.1:9943 --constructor new --suri //Alice --code-hash $CODE_HASH
```

Gives:

```
ERROR: Metadata: Node metadata is not fully compatible

Caused by:
    Node metadata is not fully compatible
```

but:

```bash
cargo contract instantiate --url ws://127.0.0.1:9943 --constructor new --suri //Alice
```

```
Contract 5HLVN694hK6jjwCsCYxtDeV3wKi52hK9YFLUngUr4PxpjAYm
```
