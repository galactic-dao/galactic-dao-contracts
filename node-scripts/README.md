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

That's normal! Now you can write a non-empty mnemonic phrase in your `.env` file, and that phrase can be anything.
> ... **for now**. Until we start using scripts to deploy the smart contracts, at which point we'll need to supply a proper mnemonic
I was very secure and chose:
```dotenv
WALLET_MNEMONIC="abc"
```