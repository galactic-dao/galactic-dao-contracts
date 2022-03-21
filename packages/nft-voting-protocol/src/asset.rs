use cosmwasm_std::{
    to_binary, Addr, BankMsg, Coin, CosmosMsg, MessageInfo, StdError, StdResult, Uint128, WasmMsg,
};
use cw20::Cw20ExecuteMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Asset {
    pub amount: Uint128,
    pub info: AssetInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssetInfo {
    Token { contract_addr: String },
    NativeToken { denom: String },
}

impl Asset {
    /// Into an outgoing payment message to recipient
    pub fn into_outgoing_msg(self, recipient: Addr) -> StdResult<CosmosMsg> {
        let amount = self.amount;

        match &self.info {
            AssetInfo::Token { contract_addr } => Ok(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: recipient.to_string(),
                    amount,
                })?,
                funds: vec![],
            })),
            AssetInfo::NativeToken { denom } => Ok(CosmosMsg::Bank(BankMsg::Send {
                to_address: recipient.to_string(),
                amount: vec![Coin {
                    denom: denom.to_string(),
                    amount,
                }],
            })),
        }
    }

    /// Into an incoming payment message / check from sender to recipient
    pub fn check_sent_or_into_request_msg(
        self,
        message_info: &MessageInfo,
        recipient: Addr,
    ) -> StdResult<Option<CosmosMsg>> {
        let amount = self.amount;

        match &self.info {
            AssetInfo::Token { contract_addr } => Ok(Some(CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                    owner: message_info.sender.to_string(),
                    recipient: recipient.to_string(),
                    amount,
                })?,
                funds: vec![],
            }))),
            AssetInfo::NativeToken { denom } => {
                match message_info.funds.iter().find(|x| x.denom == *denom) {
                    Some(coin) => {
                        if amount >= coin.amount {
                            Ok(None)
                        } else {
                            Err(StdError::generic_err(format!(
                                "Insufficient funds sent. Required: {} {}.",
                                amount, coin.denom
                            )))
                        }
                    }
                    None => {
                        if amount.is_zero() {
                            Ok(None)
                        } else {
                            Err(StdError::generic_err(format!(
                                "No funds sent. Required: {} {}.",
                                amount, denom
                            )))
                        }
                    }
                }
            }
        }
    }
}
