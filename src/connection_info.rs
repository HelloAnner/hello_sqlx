use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct ConnectionInfo {
    pid: i32,
    username: String,
    database_name: String,
    client_addr: String,
    client_port: i32,
    query: String,
}