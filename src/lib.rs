use chrono::{DateTime, Utc, TimeZone};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, Decimal256, Coin, Binary, Timestamp};
use serde::{Serializer, Deserializer, Serialize, Deserialize};

/// A number of Custom messages that can call into the Alliance bindings
#[cw_serde]
pub enum AllianceMsg {
    Delegate {
        delegator_address: Addr,
        validator_address: Addr,
        amount: Coin,
    },
    Undelegate {
        delegator_address: Addr,
        validator_address: Addr,
        amount: Coin,
    },
    Redelegate {
        delegator_address: Addr,
        validator_src_address: Addr,
        validator_dst_address: Addr,
        amount: Coin,
    },
    ClaimDelegationRewards {
        delegator_address: Addr,
        validator_address: Addr,
        denom: String,
    },
}

/// Alliance-specific queries
#[cw_serde]
#[derive(QueryResponses)]
pub enum AllianceQuery {
    #[returns(AllianceAllianceResponse)]
    Alliance { denom: String },

    #[returns(AllianceAlliancesResponse)]
    Alliances { pagination: Option<Pagination> },

    #[returns(AllianceAlliancesDelegationsResponse)]
    AlliancesDelegations { pagination: Option<Pagination> },

    #[returns(AllianceAlliancesDelegationsResponse)]
    AlliancesDelegationByValidator {
        delegator_addr: Addr,
        validator_addr: Addr,
        pagination: Option<Pagination>,
    },

    #[returns(SingleDelegationResponse)]
    Delegation {
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    },

    #[returns(RewardsResponse)]
    DelegationRewards {
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    },

    #[returns(AllianceParamsResponse)]
    Params {},

    #[returns(ValidatorResponse)]
    Validator { validator_addr: Addr },

    #[returns(AllValidatorsResponse)]
    Validators { pagination: Option<Pagination> },
}

#[cw_serde]
pub struct Pagination {
    pub key: Option<Binary>,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
    pub count_total: Option<bool>,
    pub reverse: Option<bool>,
}

#[cw_serde]
pub struct PaginationResponse {
    pub next_key: Option<Binary>,
    pub total: Option<u64>,
}

#[cw_serde]
pub struct AllianceParams {
    pub reward_delay_time: u64,
    pub take_rate_claim_interval: u64,
    pub last_take_rate_claim_time: String,
}

#[cw_serde]
pub struct DecCoin {
    // TODO: why is this an option
    pub denom: Option<String>,
    pub amount: Decimal256,
}

#[cw_serde]
pub struct ValidatorResponse {
    pub validator_addr: Addr,
    pub total_delegation_shares: Vec<DecCoin>,
    pub validator_shares: Vec<DecCoin>,
    pub total_staked: Vec<DecCoin>,
}

#[cw_serde]
pub struct AllValidatorsResponse {
    pub validators: Vec<ValidatorResponse>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct AllianceParamsResponse {
    pub params: AllianceParams,
}

#[cw_serde]
pub struct WeightRange {
    pub min: Decimal256,
    pub max: Decimal256,
}

#[cw_serde]
pub struct AllianceAsset {
    pub denom: String,
    pub reward_weight: Decimal256,
    pub consensus_weight: Decimal256,
    pub take_rate: Decimal256,
    pub total_tokens: Decimal256,
    pub total_validator_shares: Decimal256,
    #[serde(serialize_with = "serialize_time_stamp", deserialize_with = "deserialize_time_stamp")]
    pub reward_start_time: Timestamp, // "2023-06-06T18:37:29.956787974Z"
    pub reward_change_rate: Decimal256,
    pub reward_change_interval: u64,
    pub last_reward_change_time: String,
    pub reward_weight_range: WeightRange,
    pub is_initialized: Option<bool>,
}

// const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize_time_stamp<S>(
    time_stamp: &Timestamp,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date: DateTime<Utc> = Utc.timestamp_nanos(time_stamp.nanos() as i64);
    date.serialize(serializer)
    // let s = format!("{}", date.format(FORMAT));
    // serializer.serialize_str(&s)
}

pub fn deserialize_time_stamp<'de, D>(
    deserializer: D,
) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let date = DateTime::<Utc>::deserialize(deserializer)?;
    // let s = String::deserialize(deserializer)?;
    // let date = Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
    Ok(Timestamp::from_nanos(date.timestamp_nanos() as u64))
}

#[cw_serde]
pub struct AllianceAllianceResponse {
    pub alliance: AllianceAsset,
}

#[cw_serde]
pub struct AllianceAlliancesResponse {
    pub alliances: Vec<AllianceAsset>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct AllianceAlliancesDelegationsResponse {
    pub delegations: Option<Vec<DelegationResponse>>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct RewardsResponse {
    pub rewards: Vec<Coin>,
}

#[cw_serde]
pub struct SingleDelegationResponse {
    pub delegation: DelegationResponse,
}

#[cw_serde]
pub struct DelegationResponse {
    pub delegation: Delegation,
    pub balance: Coin,
}

#[cw_serde]
pub struct Delegation {
    pub delegator_address: Option<Addr>,
    pub validator_address: Option<Addr>,
    pub denom: Option<String>,
    pub shares: Decimal256,
    pub reward_history: Option<Vec<Option<Reward>>>,
    pub last_reward_claim_height: Option<u64>,
}

#[cw_serde]
pub struct Reward {
    pub denom: Option<String>,
    pub index: Decimal256,
}

pub trait CreateAllianceMsg: From<AllianceMsg> {
    // fn token_factory_create_denom(
    //     subdenom: String,
    //     metadata: Option<DenomMetadata>,
    // ) -> StdResult<Self> {
    //     Ok(AllianceMsg::CreateDenom { subdenom, metadata }.into())
    // }
    // fn token_factory_change_admin(denom: String, new_admin_address: Addr) -> StdResult<Self> {
    //     Ok(AllianceMsg::ChangeAdmin {
    //         denom,
    //         new_admin_address,
    //     }
    //     .into())
    // }
    // fn token_factory_mint_tokens(
    //     denom: String,
    //     amount: Uint256,
    //     mint_to_address: Addr,
    // ) -> StdResult<Self> {
    //     Ok(AllianceMsg::MintTokens {
    //         denom,
    //         amount,
    //         mint_to_address,
    //     }
    //     .into())
    // }
    // fn token_factory_burn_tokens(
    //     denom: String,
    //     amount: Uint256,
    //     burn_from_address: Addr,
    // ) -> StdResult<Self> {
    //     Ok(AllianceMsg::BurnTokens {
    //         denom,
    //         amount,
    //         burn_from_address,
    //     }
    //     .into())
    // }
    // fn token_factory_set_metadata(metadata: DenomMetadata) -> StdResult<Self> {
    //     Ok(AllianceMsg::SetMetadata { metadata }.into())
    // }
    // fn token_factory_force_transfer(
    //     denom: String,
    //     from_address: Addr,
    //     to_address: Addr,
    //     amount: Uint256,
    // ) -> StdResult<Self> {
    //     Ok(AllianceMsg::ForceTransfer {
    //         denom,
    //         from_address,
    //         to_address,
    //         amount,
    //     }
    //     .into())
    // }
}

impl<T> CreateAllianceMsg for T where T: From<AllianceMsg> {}

pub trait AllianceQuerier {
    // fn query_token_factory_full_denom(
    //     &self,
    //     subdenom: String,
    //     creator_addr: Addr,
    // ) -> StdResult<FullDenomResponse>;

    // fn query_token_factory_admin(&self, denom: String) -> StdResult<AdminResponse>;

    // fn query_token_factory_metadata(&self, denom: String) -> StdResult<MetadataResponse>;

    // fn query_token_factory_denoms_by_creator(
    //     &self,
    //     creator: Addr,
    // ) -> StdResult<DenomsByCreatorResponse>;

    // fn query_token_factory_params(&self) -> StdResult<TokenParamsResponse>;
}

impl<'a, T> AllianceQuerier for QuerierWrapper<'a, T>
where
    T: CustomQuery + From<AllianceQuery>,
{
    // fn query_token_factory_full_denom(
    //     &self,
    //     subdenom: String,
    //     creator_addr: Addr,
    // ) -> StdResult<FullDenomResponse> {
    //     let custom_query: T = AllianceQuery::FullDenom {
    //         subdenom,
    //         creator_addr,
    //     }
    //     .into();
    //     self.query(&custom_query.into())
    // }

    // fn query_token_factory_admin(&self, denom: String) -> StdResult<AdminResponse> {
    //     let custom_query: T = AllianceQuery::Admin { denom }.into();
    //     self.query(&custom_query.into())
    // }

    // fn query_token_factory_metadata(&self, denom: String) -> StdResult<MetadataResponse> {
    //     let custom_query: T = AllianceQuery::Metadata { denom }.into();
    //     self.query(&custom_query.into())
    // }

    // fn query_token_factory_denoms_by_creator(
    //     &self,
    //     creator: Addr,
    // ) -> StdResult<DenomsByCreatorResponse> {
    //     let custom_query: T = AllianceQuery::DenomsByCreator { creator }.into();
    //     self.query(&custom_query.into())
    // }

    // fn query_token_factory_params(&self) -> StdResult<TokenParamsResponse> {
    //     let custom_query: T = AllianceQuery::Params {}.into();
    //     self.query(&custom_query.into())
    // }
}

// This export is added to all contracts that import this package, signifying that they require
// "token_factory" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_alliance() {}
