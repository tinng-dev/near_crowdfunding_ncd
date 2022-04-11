use crate::*;

pub const MAX_TITLE_LENGTH: usize = 100;
pub const DEFAULT_GAS_FEE: Gas = 20_000_000_000_000;
pub const NEAR_DECIMAL: Balance = 1_000_000_000_000_000_000_000_000;

pub type ProjectId = String;

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    ProjectPerOwner,
    Project,
    ProjectPerOwnerInner { id: AccountId },
    ProjectMetadata,
    SupportersPerProject,
}
