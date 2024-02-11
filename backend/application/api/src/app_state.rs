use mysql::Pool;

pub struct AppState {
    pub db: Pool,
}
