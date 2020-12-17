use crate::timeline;
use juniper::FieldResult;
use juniper::RootNode;

#[derive(GraphQLEnum)]
enum Episode {
    NewHope,
    Empire,
    Jedi,
}

use juniper::{GraphQLEnum, GraphQLObject};
use timeline::get_pictures;

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Photo {
    id: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn photos(id: String) -> FieldResult<Vec<Photo>> {
        let paths_native = get_pictures();

        Ok(timeline::get_pictures()
            .into_iter()
            .filter_map(|p| p.to_str().map(|p| p.to_string()))
            .map(|p| Photo { id: p })
            .collect())
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
