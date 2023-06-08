use chrono::{DateTime, Utc, TimeZone};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, CustomQuery, QuerierWrapper, Decimal256, Coin, Binary, Timestamp, StdResult};
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
    #[returns(AllianceResponse)]
    Alliance { denom: String },

    // TODO: inconsistent naming?
    #[returns(AlliancesResponse)]
    Alliances { pagination: Option<Pagination> },

    #[returns(AlliancesDelegationsResponse)]
    AlliancesDelegations { pagination: Option<Pagination> },

    #[returns(AlliancesDelegationsResponse)]
    AlliancesDelegationByValidator {
        delegator_addr: Addr,
        validator_addr: Addr,
        pagination: Option<Pagination>,
    },

    // TODO: Does this return type have the right name?
    #[returns(DelegationResponse)]
    Delegation {
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    },

    #[returns(DelegationRewardsResponse)]
    DelegationRewards {
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    },

    #[returns(ParamsResponse)]
    Params {},

    #[returns(ValidatorResponse)]
    Validator { validator_addr: Addr },

    #[returns(ValidatorsResponse)]
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
pub struct ValidatorsResponse {
    pub validators: Vec<ValidatorResponse>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct ParamsResponse {
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

fn serialize_time_stamp<S>(
    time_stamp: &Timestamp,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let date: DateTime<Utc> = Utc.timestamp_nanos(time_stamp.nanos() as i64);
    date.serialize(serializer)
}

fn deserialize_time_stamp<'de, D>(
    deserializer: D,
) -> Result<Timestamp, D::Error>
where
    D: Deserializer<'de>,
{
    let date = DateTime::<Utc>::deserialize(deserializer)?;
    Ok(Timestamp::from_nanos(date.timestamp_nanos() as u64))
}

#[cw_serde]
pub struct AllianceResponse {
    pub alliance: AllianceAsset,
}

#[cw_serde]
pub struct AlliancesResponse {
    pub alliances: Vec<AllianceAsset>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct AlliancesDelegationsResponse {
    pub delegations: Option<Vec<SingleDelegationResponse>>,
    pub pagination: Option<PaginationResponse>,
}

#[cw_serde]
pub struct DelegationRewardsResponse {
    pub rewards: Vec<Coin>,
}

#[cw_serde]
pub struct DelegationResponse {
    pub delegation: SingleDelegationResponse,
}

#[cw_serde]
pub struct SingleDelegationResponse {
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
    fn alliance_delegate(
        delegator_address: Addr,
        validator_address: Addr,
        amount: Coin,
    ) -> Self {
        AllianceMsg::Delegate { delegator_address, validator_address, amount }.into()
    }

    fn alliance_undelegate(
        delegator_address: Addr,
        validator_address: Addr,
        amount: Coin,
    ) -> Self {
        AllianceMsg::Undelegate { delegator_address, validator_address, amount }.into()
    }

    fn alliance_redelegate(
        delegator_address: Addr,
        validator_src_address: Addr,
        validator_dst_address: Addr,
        amount: Coin,
    ) -> Self {
        AllianceMsg::Redelegate { delegator_address, validator_src_address, validator_dst_address, amount }.into()
    }

    fn alliance_claim_deligation_rewards(
        delegator_address: Addr,
        validator_address: Addr,
        denom: String,
    ) -> Self {
        AllianceMsg::ClaimDelegationRewards { delegator_address, validator_address, denom }.into()
    }

}

impl<T> CreateAllianceMsg for T where T: From<AllianceMsg> {}

pub trait AllianceQuerier {
    fn query_alliance_alliance(
        &self,
        denom: String,
    ) -> StdResult<AllianceResponse>;

    fn query_alliance_alliances(
        &self,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesResponse>;

    fn query_alliance_alliances_delegations(
        &self,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesDelegationsResponse>;

    fn query_alliance_alliances_delegation_by_validator(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesDelegationsResponse>;

    fn query_alliance_delegation(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    ) -> StdResult<DelegationResponse>;

    fn query_alliance_delegation_rewards(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    ) -> StdResult<DelegationRewardsResponse>;

    fn query_alliance_params(
        &self,
    ) -> StdResult<ParamsResponse>;

    fn query_alliance_validator(
        &self,
        validator_addr: Addr,
    ) -> StdResult<ValidatorResponse>;

    fn query_alliance_validators(
        &self,
        pagination: Option<Pagination>,
    ) -> StdResult<ValidatorsResponse>;
}

impl<'a, T> AllianceQuerier for QuerierWrapper<'a, T>
where
    T: CustomQuery + From<AllianceQuery>,
{
    fn query_alliance_alliance(
        &self,
        denom: String,
    ) -> StdResult<AllianceResponse> {
        let custom_query: T = AllianceQuery::Alliance { denom }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_alliances(
        &self,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesResponse> {
        let custom_query: T = AllianceQuery::Alliances { pagination }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_alliances_delegations(
        &self,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesDelegationsResponse> {
        let custom_query: T = AllianceQuery::AlliancesDelegations { pagination }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_alliances_delegation_by_validator(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        pagination: Option<Pagination>,
    ) -> StdResult<AlliancesDelegationsResponse> {
        let custom_query: T = AllianceQuery::AlliancesDelegationByValidator { delegator_addr, validator_addr, pagination }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_delegation(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    ) -> StdResult<DelegationResponse> {
        let custom_query: T = AllianceQuery::Delegation { delegator_addr, validator_addr, denom }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_delegation_rewards(
        &self,
        delegator_addr: Addr,
        validator_addr: Addr,
        denom: String,
    ) -> StdResult<DelegationRewardsResponse> {
        let custom_query: T = AllianceQuery::DelegationRewards { delegator_addr, validator_addr, denom }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_params(
        &self,
    ) -> StdResult<ParamsResponse> {
        let custom_query: T = AllianceQuery::Params { }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_validator(
        &self,
        validator_addr: Addr,
    ) -> StdResult<ValidatorResponse> {
        let custom_query: T = AllianceQuery::Validator { validator_addr }.into();
        self.query(&custom_query.into())
    }

    fn query_alliance_validators(
        &self,
        pagination: Option<Pagination>
    ) -> StdResult<ValidatorsResponse> {
        let custom_query: T = AllianceQuery::Validators { pagination }.into();
        self.query(&custom_query.into())
    }

}

// This export is added to all contracts that import this package, signifying that they require
// "alliance" support on the chain they run on.
#[no_mangle]
extern "C" fn requires_alliance() {}
