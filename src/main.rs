mod user;
mod connection_info;

use crate::connection_info::ConnectionInfo;
use crate::user::User;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:1111@localhost/postgres").await?;

    // 查询所有列
    let users: Vec<User> = sqlx::query_as("select * from  tmp.user")
        .fetch_all(&pool).await?;

    let connection_infos: Vec<ConnectionInfo> = sqlx::query_as("SELECT pid, usename, datname, client_addr, client_port, query FROM pg_stat_activity WHERE datname = 'tmp'")
        .fetch_all(&pool)
        .await?;

    // 打印查询结果
    for user in users {
        println!("{:?}", user);
    }

    println!("{:?}", connection_infos.len());

    Ok(())
}