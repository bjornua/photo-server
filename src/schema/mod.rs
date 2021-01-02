mod mutations;
mod query;
mod types;

use crate::app_state::LockedAppState;
use juniper::RootNode;
use mutations::MutationRoot;
use query::QueryRoot;

pub type Schema<'a> = RootNode<'a, query::QueryRoot, mutations::MutationRoot>;

pub fn new<'a>(app_state: LockedAppState) -> Schema<'a> {
    Schema::new(
        QueryRoot(app_state.clone()),
        MutationRoot(app_state.clone()),
    )
}
