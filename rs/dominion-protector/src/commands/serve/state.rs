use tokio_postgres::Client;

pub struct ServerState {
    pub db: Client,
}