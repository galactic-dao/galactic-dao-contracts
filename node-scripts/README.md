# Node Scripts

Set of Node.js scripts to interact with the smart contracts.

## Local Setup

The following environment variables are expected:
> Given that some information in the .env is private (mnemonic phrase) and can give access to your funds if committed by mistake, it is recommended to use a test wallet with no funds on mainnet to avoid publishing private info by accident.

<details>
<summary>Create a .env file in this same folder and save the following content</summary>

```dotenv
# One of TESTNET / MAINNET / LOCAL
TERRA_CHAIN_TYPE=TESTNET
# Mnemonic used for executing all transactions, see below section
WALLET_MNEMONIC=""


#
# CW721
#
# Path to compiled CW721 (NFT) contract - from https://github.com/CosmWasm/cw-nfts
CW_721_WASM_FILEPATH=/Users/frankjia/Desktop/Programming/galaxy-labs/galaxy-labs-contracts/contracts/artifacts/cw721_metadata_onchain.wasm
# Code ID of CW721 after uploading the WASM
CW_721_CODE_ID=12429

#
# Airdrop (run `build.sh` for compiled WASM)
#
AIRDROP_WASM_FILEPATH=/Users/frankjia/Desktop/Programming/galaxy-labs/galaxy-labs-contracts/contracts/artifacts/nft_airdrop.wasm
# 13773 on testnet, 592 on mainnet
AIRDROP_CODE_ID=592
AIRDROP_CONTRACT_ADDR=(to be changed after deploying airdrop contract)

#
# Distribution (run `build.sh` for compiled WASM)
#
DISTRIBUTION_WASM_FILEPATH=/Users/frankjia/Desktop/Programming/galaxy-labs/galaxy-labs-contracts/contracts/artifacts/nft_distribution.wasm
# 13772 on testnet, ??? on mainnet
DISTRIBUTION_CODE_ID=13772
DISTRIBUTION_CONTRACT_ADDR=(to be changed after deploying distribution contract)

WEB3_STORAGE_API_KEY=(get one for free from web3.storage, but not needed unless you are uploading to IPFS)
NFT_STORAGE_API_KEY=(same as above, but for nft.storage)
```

Here are some test mnemonics that are funded on Testnet. You can use the [faucet](https://faucet.terra.money/) to reload these.:

```dotenv
# Galaxy Labs Test 1: terra104xmc66he33fsxjwkaj6arq04szftn3kwh48hr
TEST_1_WALLET_MNEMONIC="inherit myself term crystal grain butter lyrics inhale scene round border ramp radar just breeze razor bus meat allow swear sentence village rug fragile"
# Galaxy Labs Test 2: terra1ktd2esx85k4k7vf5n82py2k5m4ghaeukj04l2w
TEST_2_WALLET_MNEMONIC="expose apple ugly sick robust throw large citizen oxygen theme coil material old scorpion scorpion truth ignore sure very opera tilt dwarf day treat"
```
</details>

### Node installation and run

```bash
> cd /path/to/galactic-dao-contracts/node-scripts/
> npm install
> npm run integration-test/full
...
Error: wallet env var not defined
```

That's normal! Now you can write a non-empty mnemonic phrase in your `.env` file, and that phrase can be anything. I was very secure and chose:
```dotenv
WALLET_MNEMONIC="abc"
```
## Deployment

**Heroku** is used only to run a minting script during a distribution.

### Useful Commands

#### Deploy

```shell
git subtree push --prefix node-scripts heroku main

# Then need to scale down the default web process
heroku ps:scale web=0
# Scale up the minting process
heroku ps:scale mint=1
```

#### View Logs

```shell
heroku logs --dyno=mint --tail
```

#### Change Env Vars

Install [heroku-config](https://github.com/xavdid/heroku-config) to easily set env vars from a file.

Then, just run: `heroku config:push -f .env.production -o` (Make sure you're in the `node-scripts` directory though!)

## Airdrop Checklist

- Create mint JSON file of MintNftMessage[] & double check!
  - Need:
  ```
  - collection_name
  - collection_symbol
  - mint_cap
  - Per NFT:
    - Name + Description
    - required metadata + IPFS URI's
  ```

### Testnet Run

- Check env vars:
    - `DEPLOY_WALLET_MNEMONIC`
- Check `deployAirdropContract` for correct params
- Update `AIRDROP_CONTRACT_ADDR` with new contract
- Modify `airdropNfts` as needed
- Deploy & check:
  - All success
  - Query `all_tokens`:
  ```json
  {
    "all_tokens": {}  
  }
    ```
  
  - Query tokens for a specific recipient, for example:
  
  ```json
  {
    "tokens": {
      "owner": "terra1he9pf67wkwle78l72h8qraxck2agweafm430eh"
    }
  }
  ```

  - Query NFT info:

  ```json
  {
    "all_nft_info": {
      "token_id": "1"
    }
  }
  ```

### Mainnet Run
- Repeat above but switching `DEPLOY_WALLET_MNEMONIC` and `TERRA_CHAIN_TYPE` 