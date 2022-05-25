use crate::*;

#[near_bindgen]
impl Contract {
    //attach 1 Near to create a project (services fee)
    #[payable]
    pub fn new_project(&mut self, mut metadata: ProjectMetadata) -> ProjectId {
        let owner = env::predecessor_account_id();
        let project = Project {
            owner_id: owner.clone(),
        };

        assert!(
            env::attached_deposit() == NEAR_DECIMAL,
            "Attach exactly 1 Near to create new project"
        );

        assert!(
            metadata.title.len() <= MAX_TITLE_LENGTH,
            "Title's too long!"
        );

        assert!(
            metadata.ended_at > env::block_timestamp(),
            "Endtime is not valid"
        );

        let project_id = gen_proj_id();
        self.project.insert(&project_id, &project);
        self.project_metadata.insert(&project_id, &metadata);

        //Add project to owner
        let mut owner_projects =
            self.project_per_owner
                .get(&owner.clone())
                .unwrap_or(UnorderedSet::new(StorageKey::ProjectPerOwnerInner {
                    id: owner.clone(),
                }));
        owner_projects.insert(&project_id);
        self.project_per_owner.insert(&owner, &owner_projects);

        metadata.claimed = U128(0);
        metadata.funded = U128(0);

        let project_id = gen_proj_id();
        self.project.insert(&project_id, &project);
        self.project_metadata.insert(&project_id, &metadata);

        //Add project to owner
        let mut owner_projects =
            self.project_per_owner
                .get(&owner.clone())
                .unwrap_or(UnorderedSet::new(StorageKey::ProjectPerOwnerInner {
                    id: owner.clone(),
                }));
        owner_projects.insert(&project_id);
        self.project_per_owner.insert(&owner, &owner_projects);
        project_id
    }

    #[payable]
    //Return current balance of sender
    pub fn support_project(&mut self, project_id: ProjectId) -> Balance {
        let amount = env::attached_deposit();

        let mut metadata = self
            .project_metadata
            .get(&project_id)
            .expect("Project doesn't exist!");
        let minimum_deposit = u128::from(metadata.minimum_deposit);

        assert!(
            amount >= minimum_deposit,
            "Donation must greater than {}",
            minimum_deposit
        );

        assert!(
            metadata.ended_at >= env::block_timestamp(),
            "Donation time is ended"
        );

        let mut funded = u128::from(metadata.funded);
        funded += amount;
        metadata.funded = U128(funded);
        self.project_metadata.insert(&project_id, &metadata);

        //Update balance of supporter
        let supporter = env::predecessor_account_id();
        let mut supporters =
            self.supporters_per_project
                .get(&project_id)
                .unwrap_or(UnorderedMap::new(StorageKey::SupporterPerProjectInner {
                    id: project_id.clone(),
                }));

        let mut my_balance = supporters.get(&supporter).unwrap_or(0_u128);
        my_balance += amount;
        supporters.insert(&supporter, &my_balance);
        self.supporters_per_project.insert(&project_id, &supporters);

        my_balance
    }
}
