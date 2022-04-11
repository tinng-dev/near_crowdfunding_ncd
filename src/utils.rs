use crate::*;
use url::Url;

pub fn gen_proj_id() -> ProjectId {
    env::predecessor_account_id() + "#" + &env::block_index().to_string()
}

pub fn valid_url(maybe_url: String) -> bool {
    return Url::parse(&maybe_url).is_ok();
}
