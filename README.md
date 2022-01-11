# Galactic DAO Smart Contracts

A technical doc for on-chain proposals with NFT-based voting.

## Introduction

### Overall Goals

- Enable NFT holders to vote with their NFT, where 1 NFT entails 1 vote
- Allow for flexible proposals that have multiple options, enabling proposals *beyond* simple yes/no decisions.
- Enabling *transparent* on-chain poll results that are associated with the NFTs
- Enabling a source of revenue for the DAO, where the cost to create a proposal is deposited into the DAO treasury

### Technical Summary

There are two contracts:

- **Proposal Contract:** A contract to hold the details of the specific proposal, as well as the logic for the voting mechanism
- **Proposal Factory Contract:** A factory that enables creating of proposals for a fee. All fees are routed to the owner of the factory

The proposal factory holds limited logic, and is easier to understand in the context of the proposal contract. So, we'll review the proposal contract first.

<details>
<summary>Proposal Contract</summary>

### State & Configuration

A proposal has different *options*, described by:

```rust
pub struct ProposalOption {
    /// ID of the option
    pub id: u16,
    /// Display name
    pub name: String,
}
```

Each option has a *status*:

```rust
pub struct ProposalOptionStatus {
    /// ID of the option
    pub id: u16,
    /// Number of votes associated with the option
    pub votes: u16,
}
```

So, a proposal can be initially configured by:

```rust
pub struct ProposalConfig {
    /// The NFT contract associated with NFT voting
    pub nft_contract: String,
    /// A URI with details for the proposal
    pub proposal_uri: String,
    /// The allowed options for voting
    pub options: Vec<ProposalOption>,
    /// The time at which the proposal closes, in seconds
    pub close_time: u64,
    /// Address that initiated the proposal
    pub proposer: String,
}
```

And holds the given state config:

```rust
pub struct ProposalState {
    /// Address of the factory contract that created this proposal
    pub creator: String,
    /// Whether the proposal has been revoked by the proposer
    pub is_revoked: bool,
}
```

Notice the difference between `creator` and `proposer`:

- **Creator** is the address of the wallet that created the proposal, this would be the factory contract
    > Why have this?
    > We can verify a "valid" proposal by checking that the `creator` field is in a set of whitelisted factory contract addresses
- **Proposer** is the actual NFT holder that wants to make the proposal
    > Why have this?
    > We allow proposers to *revoke* their proposal (if it is not already closed)

To keep track of the current proposal status, we need to manage 2 items:

- The specific vote for a given NFT
- A global count of # votes for each option

This is done by the following:

```rust
/// Map of token ID -> voted option ID
pub const VOTE_BY_TOKEN_ID: Map<String, u16> = Map::new("vote_by_token_id");

/// Map of option ID -> number of votes
pub const TALLY: Map<U16Key, u16> = Map::new("tally");
```

### Instantiation

This is simple, we just need the `ProposalConfig`.

### Execution

We allow 2 unique messages:

```rust
pub enum ProposalExecuteMsg {
    /// Casts a vote with the given token ID, if null, indicates a no-vote, revoking any existing votes
    Vote {
        option_id: Option<u16>,
        token_id: String,
    },
    /// Revokes the proposal, can only be called by the proposer
    Revoke {},
}
```

### Queries

We have 2 available queries:

```rust
pub enum ProposalQueryMsg {
    /// Retrieves general configuration & status for the proposal
    Status {},
    /// Retrieves the current votes for the given token IDs
    Votes { token_ids: Vec<String> },
}
```

**Status** retrieves the overall status of the proposal, including the current vote counts:

```rust
pub struct ProposalStatusResponse {
    /// Current state of the proposal
    pub state: ProposalState,
    /// Original configuration for the proposal
    pub config: ProposalConfig,
    /// The current statuses for each option
    pub tally: Vec<ProposalOptionStatus>,
}
```

**Votes** enables querying votes made by specific NFTs:

```rust
pub struct VotesQueryResponse {
    /// Voted option IDs in the same order as the token IDs provided in the queryNft, null if no vote exists
    pub votes: Vec<Option<u16>>,
}
```

</details>

<details>
<summary>Proposal Factory Contract</summary>

Why have a factory contract?

- Allows whitelisting of proposals → only those created by a trusted "factory" are valid proposals
- Allows a fee to be collected for creating a proposal
- Allows listeners to listen to certain attributes (ex. the creation of a proposal)

### State & Configuration

In short. We hold the NFT collection associated with the proposals, the cost for each proposal, and the owner of the proposal factory. We also need to hold the `code_id` of the Proposal contract to instantiate

```rust
pub struct ProposalFactoryConfig {
    /// The NFT contract associated with NFT voting
    pub nft_contract: String,
    /// Cost, in uluna, for creating a proposal
    pub proposal_cost: u64,
    /// Code ID of the proposal to instantiate
    pub proposal_code_id: u64,
}

pub struct ProposalFactoryState {
    /// Owner of the proposal factory - all funds are routed to the owner
    pub owner: String,
}
```

### Instantiation

Just need `ProposalFactoryConfig` to be passed in

### Execution

We allow for 3 things:

- Creating a proposal → the creator must hold an NFT
- Modifying the per-proposal cost and/or the code ID → can only be called by the factory owner
- Withdrawing funds → allows the owner of the factory to withdraw collected fees

```rust
pub enum ProposalFactoryExecuteMsg {
    /// Creates a proposal with the given config
    CreateProposal {
        proposal_uri: String,
        options: Vec<ProposalOption>,
        close_time: u64,
    },
    /// Updates the configuration with any specified items
    ModifyConfig {
        proposal_cost: Option<u64>,
        proposal_code_id: Option<u64>,
    },
    /// Withdraws funds within the contract to the owner
    WithdrawFunds { amount_uluna: Uint128 },
}
```

### Queries

Just allow retrieval of the current configuration

</details>

## Test Cases

### Happy Paths

- Create Factory with owner
- Call factory with NFT holder to create a proposal
- Somehow (?) get the proposal contract
- Call vote on proposal with owned token_id and an option
- Check that tallying updates
- Call vote on proposal with no option
- Check that tallying updates

### Factory Access Control

- Anyone can create a proposal **iff** they hold an NFT + they send the required funds
    - *TODO: Enable owner to create proposals for free*
- Only owner can change config
- Anyone can withdraw funds, but funds are given to the owner

### Factory Withdrawals

- Test that withdrawals work properly

### Proposal Access Control

- Anyone can vote with their NFT **iff** they own the token_id
- Only the proposer can revoke a proposal
- Can't vote if past `close_time` or `revoke` has been called

### Proposal Voting Logic

- New votes update voting state keyed by the token_id & the total tally
- Switching a vote does the same
- Removing a vote does the same
- Can't vote on non-existent option
