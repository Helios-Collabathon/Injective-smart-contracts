# Injective-smart-contracts

The `inj-interchain-persona` smart contract is designed for integration with **Helios Connect**, enabling advanced cross-chain identity management. This decentralized contract allows users to link their wallets from multiple blockchain ecosystems, starting with Injective and MultiversX, under a single unified persona. It ensures secure, transparent, and efficient management of cross-chain identities.

## Features
- **Cross-Chain Identity Linking**: Seamlessly link wallets from various blockchains under a single persona.
- **Decentralized**: Ensures security, privacy, and transparency through a fully decentralized architecture.
- **Modular Design**: Built to be modular, making it easy to extend and adapt with future blockchain integrations and functionalities.
- **Open-Source**: The code is publicly available, fostering community collaboration and contributions.

![Software architecture](https://github.com/Helios-Collabathon/Injective-smart-contracts/blob/main/software_architecture.drawio.png?raw=true)

## Usage of the contracts

The `inj-interchain-persona` smart contract is intended for use in Helios Connect and similar applications requiring cross-chain identity management. Its modular nature allows for straightforward integration and customization for future blockchain ecosystems and new features.

### 1. Compile CosmWasm Contracts

Non ARM (Non-Apple silicon) devices
```cargo run-script non_arm_optimize```

Apple silicon devices (M1, M2, etc.)
```cargo run-script optimize```

### 2. Download Dockerised Injective Chain Binary

```
docker run --name="injective-core-v1.13.2" \
-v=<directory_to_which_you_cloned_injective_smart_contracts>/artifacts:/var/artifacts \
--entrypoint=sh public.ecr.aws/l9h3g6c6/injective-core:v1.13.2 \
-c "tail -F anything"-
```

### 3. Open a new terminal and access the Docker container

access the docker container
```docker exec -it injective-core-v1.13.2 sh```

add a test user (when prompted use 1234567890 as password)
```injectived keys add testuser```

To confirm, search for your address on the Injective Testnet Explorer to check your balance.

Fund your recently generated test address with the Injective test faucet.
https://testnet.faucet.injective.network

Check balance
https://testnet.sentry.lcd.injective.network:443/cosmos/bank/v1beta1/balances/<your inj address>

set INJ_ADDRESS inside the "injective-core-v1.13.2" container:
```export INJ_ADDRESS="<your inj address>"```

### 4. Upload the wasm contracts

inside the "injective-core-v1.13.2" container:
```
yes 1234567890 | injectived tx wasm store /var/artifacts/inj_interchain_persona.wasm \
--from=$(echo $INJ_ADDRESS) \
--instantiate-anyof-addresses=$(echo $INJ_ADDRESS) \
--chain-id="injective-888" \
--yes --gas-prices=500000000inj --gas=20000000 \
--node=https://testnet.sentry.tm.injective.network:443
```
store txhash eg. 5B549A1CA77B125CF2EEFFB624647A7E9A22F5585D9366D0D3D1BB2923417108

### 5. Get the wasm contract CODE ID

```injectived query tx <txhash> --node=https://testnet.sentry.tm.injective.network:443```

search in the output for:

key: code_id
value: '"12431"'

store them inside "injective-core-v1.13.2" container:

```export CODE_ID=12431```

### 6. Instantiate the contract
```yes | apt install jq```

Execute inside "injective-core-v1.13.2" container to instantiate the controller contract:
```export INIT='{}'```

```yes 1234567890 | injectived tx wasm instantiate $CODE_ID $INIT --label="Test" --from=$(echo $INJ_ADDRESS) --admin=$(echo $INJ_ADDRESS) --chain-id="injective-888" --yes --gas-prices=500000000inj --gas=20000000 --node=https://testnet.sentry.tm.injective.network:443```

Execute inside "injective-core-v1.13.2" container to get contract address:
```export CONTRACT=$(injectived query wasm list-contract-by-code $CODE_ID --node=https://testnet.sentry.tm.injective.network:443 --output json | jq -r '.contracts[-1]')```

```injectived query wasm contract $CONTRACT --node=https://testnet.sentry.tm.injective.network:443```