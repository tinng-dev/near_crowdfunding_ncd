use crate::*;
use near_sdk::Duration;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Project {
    pub owner_id: AccountId,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct ProjectMetadata {
    pub title: String,
    pub description: String,
    pub target: U128,
    pub minimum_deposit: U128,
    pub started_at: Timestamp,
    pub ended_at: Timestamp,
    pub funded: U128,

    //vesting
    pub vesting_start_time: Timestamp,
    pub vesting_end_time: Timestamp,
    pub vesting_interval: Duration,
    claimed: U128,
}

#[near_bindgen]
impl Contract {
    //TODO: implement view method for project

    pub fn get_claimable_amount(&self, project_id: ProjectId) -> Balance {
        let current_ts = env::block_timestamp();

        let metadata = self
            .project_metadata
            .get(&project_id)
            .expect("Project doesn't exists!");

        let claimable_amount = {
            if current_ts < metadata.vesting_start_time {
                0
            } else if current_ts >= metadata.vesting_end_time {
                u128::from(metadata.funded) - u128::from(metadata.claimed)
            } else {
                let cur_intervals: u64 =
                    (current_ts - metadata.vesting_start_time) / metadata.vesting_interval;
                let total_intervals: u64 = (metadata.vesting_end_time
                    - metadata.vesting_start_time)
                    / metadata.vesting_interval;

                u128::from(metadata.funded) / u128::from(total_intervals)
                    * u128::from(cur_intervals)
                    - u128::from(metadata.claimed)
            }
        };
        claimable_amount
    }
}
