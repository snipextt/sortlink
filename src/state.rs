#[derive(Clone)]
pub struct AppState {
    pub db: rbatis::RBatis,
}

impl AppState {
    pub fn new(db: rbatis::RBatis) -> Self {
        AppState { db }
    }
}
