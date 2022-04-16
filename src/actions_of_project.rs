use crate::*;

#[near_bindgen]
impl Contract {
    //attach 1 Near to create a project (services fee)
    #[payable]
    pub fn new_project(&mut self, metadata: Option<ProjectMetadata>) -> ProjectId {
        let owner: AccountId = env::predecessor_account_id().into();
        let project = Project {
            owner_id: owner.clone(),
        };

        let mut metadata = {
            if metadata.is_some() {
                metadata.unwrap()
            } else {
                ProjectMetadata::default()
            }
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
            valid_url(metadata.clone().description),
            "Submit description as an ipfs url"
        );

        assert!(
            metadata.ended_at > env::block_timestamp(),
            "Endtime is not valid"
        );

        metadata.started_at = env::block_timestamp();

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

        //TODO: assert metadata
        assert!(
            metadata.title.len() <= MAX_TITLE_LENGTH,
            "Title's too long!"
        );

        assert!(
            valid_url(metadata.clone().description),
            "Submit description as an ipfs url"
        );

        assert!(
            metadata.ended_at > env::block_timestamp(),
            "Endtime is not valid"
        );

        metadata.started_at = env::block_timestamp();

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
        let mut funded = u128::from(metadata.funded);
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

        funded += amount;
        metadata.funded = U128(funded);
        self.project_metadata.insert(&project_id, &metadata);

        //Update balance of supporter
        let supporter: AccountId = env::predecessor_account_id().into();
        let mut supporters =
            self.supporters_per_project
                .get(&project_id)
                .unwrap_or(UnorderedMap::new(StorageKey::SupporterPerProjectInner {
                    id: supporter.clone(),
                }));

        let mut my_balance = supporters.get(&supporter).unwrap_or(0_u128);
        my_balance += amount;
        supporters.insert(&supporter.into(), &my_balance);
        self.supporters_per_project.insert(&project_id, &supporters);

        my_balance
    }
}
