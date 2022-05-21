use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::json_types::{U128, WrappedBalance};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, Balance, BorshStorageKey, Gas, PanicOnDefault,
    Promise, Timestamp,
};

pub use crate::actions_of_project::*;
pub use crate::actions_of_reward::*;
pub use crate::actions_of_supporters::*;
pub use crate::constants::*;
pub use crate::project::*;
pub use crate::utils::*;

mod actions_of_project;
mod actions_of_reward;
mod actions_of_supporters;
mod constants;
mod project;
mod utils;

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub project_per_owner: LookupMap<AccountId, UnorderedSet<ProjectId>>,
    pub project: LookupMap<ProjectId, Project>,
    pub project_metadata: UnorderedMap<ProjectId, ProjectMetadata>,
    pub supporters_per_project: LookupMap<ProjectId, UnorderedMap<AccountId, Balance>>,
    pub force_stop_project: LookupMap<ProjectId, UnorderedSet<AccountId>>, //TODO: Add creator to blacklist
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            owner_id: env::predecessor_account_id(),
            project_per_owner: LookupMap::new(StorageKey::ProjectPerOwner),
            project: LookupMap::new(StorageKey::Project),
            project_metadata: UnorderedMap::new(StorageKey::ProjectMetadata),
            supporters_per_project: LookupMap::new(StorageKey::SupportersPerProject),
            force_stop_project: LookupMap::new(StorageKey::ForceStopProject),
        }
    }
}
