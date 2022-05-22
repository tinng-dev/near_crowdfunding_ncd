use crate::*;
use near_sdk::{json_types::ValidAccountId, CryptoHash, Duration};

pub enum ProjectStatus {
    NotStarted,
    CrowdFunding,
    Vesting,
    Ended,
}

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
    pub claimed: U128,
    pub force_stop_ts: Option<u64>,
}

impl Default for ProjectMetadata {
    fn default() -> Self {
        Self {
            title: "Default title".to_string(),
            description: "".to_string(),
            target: U128(NEAR_DECIMAL * 100),    //100 Near
            minimum_deposit: U128(NEAR_DECIMAL), //1 Near
            started_at: env::block_timestamp(),
            ended_at: env::block_timestamp() + 1_000_000_000 /*1s*/ * 60, // 60 seconds
            funded: U128(0),
            vesting_start_time: env::block_timestamp() + 1_000_000_000 * 60,
            vesting_end_time: env::block_timestamp() + 1_000_000_000 * 180, // 180 seconds
            vesting_interval: 1_000_000_000 * 30,                           // 30 seconds
            claimed: U128(0),
            force_stop_ts: None
        }
    }
}

impl ProjectMetadata {
    pub fn internal_get_funded(&self) -> u128 {
        self.funded.into()
    }

    pub fn internal_get_claimed(&self) -> u128 {
        self.claimed.into()
    }
}

#[near_bindgen]
impl Contract {
    //TODO: implement view method for project
    pub fn get_projects(&self, from_index: u64, limit: u64) -> Vec<(ProjectId, ProjectMetadata)> {
        let keys = self.project_metadata.keys_as_vector();

        let from = if keys.len() > (limit + from_index) {
            keys.len() - limit - from_index
        } else {
            0
        };

        let to = if keys.len() > from_index {
            keys.len() - from_index
        } else {
            0
        };
        (from..to)
            .map(|index| {
                let project_id = keys.get(index).unwrap();
                (
                    project_id.clone(),
                    self.project_metadata.get(&project_id).unwrap(),
                )
            })
            .rev()
            .collect()
    }

    pub fn get_project(&self, project_id: ProjectId) -> ProjectMetadata {
        self.project_metadata
            .get(&project_id)
            .expect("Project not found")
    }

    pub fn get_my_projects(
        &self,
        account_id: ValidAccountId,
        from_index: u64,
        limit: u64,
    ) -> Vec<(ProjectId, ProjectMetadata)> {
        if let Some(projects) = self.project_per_owner.get(&account_id.into()) {
            let project_ids = projects.as_vector();
            (from_index..std::cmp::min(from_index + limit, project_ids.len()))
                .map(|index| {
                    let project_id = project_ids.get(index).unwrap();
                    (
                        project_id.clone(),
                        self.project_metadata.get(&project_id).unwrap(),
                    )
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn internal_get_claimable_amount(&self, project_id: ProjectId) -> Balance {
        let metadata = self
            .project_metadata
            .get(&project_id)
            .expect("project doesn't exists!");

        let from_ts = if self.is_force_stop(project_id.clone()) {
            metadata.force_stop_ts.unwrap()
        } else {
            env::block_timestamp()
        };

        {
            if from_ts < metadata.vesting_start_time {
                0
            } else if from_ts >= metadata.vesting_end_time {
                u128::from(metadata.funded) - u128::from(metadata.claimed)
            } else {
                let cur_intervals: u64 =
                    (from_ts - metadata.vesting_start_time) / metadata.vesting_interval;
                let total_intervals: u64 = self.get_number_of_miletones(project_id.clone());
                u128::from(metadata.funded) / u128::from(total_intervals)
                    * u128::from(cur_intervals)
                    - u128::from(metadata.claimed)
            }
        }
    }

    pub fn get_claimable_amount(&self, project_id: ProjectId) -> WrappedBalance{
        U128::from(self.internal_get_claimable_amount(project_id))
    }

    pub fn is_force_stop(&self, project_id: ProjectId) -> bool {
        if let Some(force_stop) = self.force_stop_project.get(&project_id.clone()) {
            let supporters_len = self.supporters_per_project.get(&project_id).unwrap().len();
            let force_stop_len = force_stop.len();

            if force_stop_len > supporters_len / 2 {
                return true;
            }
        }
        false
    }

    pub fn get_supporters(&self, project_id: ProjectId) -> Vec<(AccountId, Balance)> {
        self.supporters_per_project.get(&project_id).expect("Project not found").to_vec()
    }

    pub fn get_number_of_miletones(&self, project_id: ProjectId) -> u64 {
        let metadata = self
            .project_metadata
            .get(&project_id)
            .expect("Project doesn't exists!");

        (metadata.vesting_end_time - metadata.vesting_start_time) / metadata.vesting_interval
    }

    pub fn get_force_stop_accounts(&self, project_id: ProjectId) -> Vec<AccountId> {
        self.force_stop_project.get(&project_id).unwrap_or(return vec![]).to_vec()
    }

    pub fn get_project_status(&self, project_id: ProjectId) {}
}
