use crate::schema;

#[derive(Clone)]
pub struct State {
    pub schema: std::sync::Arc<schema::Schema>,
}

impl State {
    pub fn new() -> Self {
        let state = State {
            schema: std::sync::Arc::new(schema::create_schema()),
        };
        return state;
    }
}
