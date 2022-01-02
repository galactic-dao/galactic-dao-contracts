use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas};
use schemars::schema_for;

use galacticdao_nft_voting_protocol::proposal::{
    ProposalExecuteMsg, ProposalInstantiateMsg, ProposalQueryMsg, ProposalStatusResponse,
    VotesQueryResponse,
};

fn main() {
    let mut out_dir = current_dir().unwrap();
    out_dir.push("schema");
    create_dir_all(&out_dir).unwrap();
    remove_schemas(&out_dir).unwrap();

    export_schema(&schema_for!(ProposalInstantiateMsg), &out_dir);
    export_schema(&schema_for!(ProposalExecuteMsg), &out_dir);
    export_schema(&schema_for!(ProposalQueryMsg), &out_dir);
    export_schema(&schema_for!(ProposalStatusResponse), &out_dir);
    export_schema(&schema_for!(VotesQueryResponse), &out_dir);
}
