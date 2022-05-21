use near_sdk::{ext_contract, PromiseResult};

use crate::*;

#[near_bindgen]
impl Contract {
    pub fn claim_reward(&mut self, project_id: ProjectId) -> Balance {
        let _project = self
            .project
            .get(&project_id)
            .expect("Project doesn't exists!");

        let timestamp = if self.is_force_stop(project_id.clone()) {
            let project_metadata = self.project_metadata.get(&project_id).expect("Project not found");
            project_metadata.force_stop_ts.unwrap()
        } else {
            env::block_timestamp()
        };

        assert!(
            !self.is_force_stop(project_id.clone()),
            "The reward is forced stop by the community!!"
        );

        let beneficiary = env::predecessor_account_id();

        let amount = self.get_claimable_amount(project_id.clone());
        env::log(format!("Amount to claim = {}", amount).as_bytes());

        assert!(amount > 0, "There is nothing to claim at the moment");

        Promise::new(beneficiary)
            .transfer(amount)
            .then(ext_self::on_reward_transfer(
                project_id,
                amount,
                &env::current_account_id(),
                0,
                DEFAULT_GAS_FEE,
            ));
        amount
    }

    //Calculate percent of each contributor after event ended
    //TODO: Send reward back to supporters after project forced stop
}

#[ext_contract(ext_self)]
pub trait ExtProjectReward {
    fn on_reward_transfer(&mut self, project_id: ProjectId, amount: Balance) -> bool;
}

#[near_bindgen]
impl Contract {
    #[private]
    pub fn on_reward_transfer(&mut self, project_id: ProjectId, amount: Balance) -> bool {
        match env::promise_result(0) {
            PromiseResult::Successful(_) => {
                let mut metadata = self.project_metadata.get(&project_id).unwrap();
                metadata.claimed = U128::from(u128::from(metadata.claimed) + amount);
                self.project_metadata.insert(&project_id, &metadata);
                true
            }
            _ => false,
        }
    }
}
