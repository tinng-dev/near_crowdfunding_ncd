use crate::*;

//Implement a simple DAO for supporters

#[near_bindgen]
impl Contract {
    pub fn force_stop(&mut self, project_id: ProjectId) {
        assert!(!self.is_force_stop(project_id.clone()), "Project already force stop");
        let supporters = self
            .supporters_per_project
            .get(&project_id.clone())
            .expect("Project not found");

        let _ = supporters
            .get(&env::predecessor_account_id())
            .expect("You are not a supporter!");

        let mut force_stop = self
            .force_stop_project
            .get(&project_id)
            .unwrap_or(UnorderedSet::new(StorageKey::ForceStopProjectInner {
                id: project_id.clone(),
            }));

        force_stop.insert(&env::predecessor_account_id());
        self.force_stop_project.insert(&project_id, &force_stop);

        //NOTE: Update force_stop timestamp
        if self.is_force_stop(project_id.clone()) {
            let mut project_metadata = self.project_metadata.get(&project_id).expect("Project not found");
            project_metadata.force_stop_ts = Some(env::block_timestamp());
            self.project_metadata.insert(&project_id, &project_metadata);
        }
    }

    pub fn drawdown(&mut self, project_id: ProjectId) -> U128 {
        assert!(self.is_force_stop(project_id.clone()), "Project is running");
        //NOTE: Get remaining money 
        let project_metadata = self.project_metadata.get(&project_id).expect("Project not found");
        let remaining_fund: Balance = project_metadata.internal_get_funded() - project_metadata.internal_get_claimed() - self.internal_get_claimable_amount(project_id.clone());

        let mut supporters = self
            .supporters_per_project
            .get(&project_id.clone())
            .expect("Project not found");
        let my_invest = supporters
            .get(&env::predecessor_account_id())
            .expect("You are not a supporter!");

        assert!(my_invest > 0, "You invest is Zero!!");

        let amount = (my_invest / project_metadata.internal_get_funded()) * remaining_fund;

        //Transfer to investor
        Promise::new(env::predecessor_account_id()).transfer(amount);
        supporters.insert(&env::predecessor_account_id(), &0);
        self.supporters_per_project.insert(&project_id, &supporters);
        U128::from(amount)
    }
}
