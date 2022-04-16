use crate::*;

//Implement a simple DAO for supporters

#[near_bindgen]
impl Contract {
    pub fn force_stop(&mut self, project_id: ProjectId) {
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
    }
}
