use juniper::GraphQLObject;

#[derive(GraphQLObject)]
pub struct Photo {
    pub id: String,
    pub hash: String,
    pub width: f64,
    pub height: f64,
    pub date: chrono::DateTime<chrono::Utc>,
    pub size: f64,
    pub added: chrono::DateTime<chrono::Utc>,
}
