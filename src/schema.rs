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

#[derive(GraphQLObject)]
#[graphql(description = "A humanoid creature in the Star Wars universe")]
struct Photo {
    id: String,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn phtotos(id: String) -> FieldResult<Human> {
        Ok(Photo {
            id: "1234".to_owned(),
        })
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
