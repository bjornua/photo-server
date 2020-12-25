use crate::timeline;
use juniper::FieldResult;
use juniper::RootNode;

use juniper::GraphQLObject;
use timeline::get_pictures;

#[derive(GraphQLObject)]
struct Photo {
    id: String,
    hash: String,
    width: f64,
    height: f64,
    date: chrono::DateTime<chrono::Utc>,
    size: f64,
    added: chrono::DateTime<chrono::Utc>,
}

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn photos(id: String) -> FieldResult<Vec<Photo>> {
        let paths_native = get_pictures();

        Ok(timeline::get_pictures()
            .into_iter()
            .filter_map(|p| p.to_str().map(|p| p.to_string()))
            .map(|p| Photo {
                id: p,
                hash: String::from("Test Hash"),
                width: 123.0,
                height: 123.0,
                date: chrono::MIN_DATETIME,
                size: 123.0,
                added: chrono::MIN_DATETIME,
            })
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
