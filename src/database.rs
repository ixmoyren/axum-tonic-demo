use sqlx::{
    Database,
    migrate::{Migrate, MigrateDatabase, Migrator},
    pool::PoolConnection,
};
use std::path::Path;

pub async fn check_and_create_db<MD>(url: &str) -> anyhow::Result<()>
where
    MD: MigrateDatabase,
{
    // 判断数据库存不存在
    let exists = MD::database_exists(url).await?;
    if !exists {
        // 不存在则创建
        MD::create_database(url).await?
    }
    Ok(())
}

pub async fn migrate<T>(mut conn: PoolConnection<T>, path: impl AsRef<Path>) -> anyhow::Result<()>
where
    T: Database,
    <T as Database>::Connection: Migrate,
{
    let path = path.as_ref();
    let migrator = Migrator::new(path).await?;
    // 开始执行 SQL 脚本
    migrator.run(&mut conn).await?;
    Ok(())
}

pub mod service {
    use futures::stream::BoxStream;
    use serde::{Deserialize, Serialize};
    use sqlx::{FromRow, SqlitePool};

    pub(crate) async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
        let mut conn = pool.acquire().await?;
        let id = sqlx::query("insert into todos (description) values ($1)")
            .bind(description)
            .execute(&mut *conn)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    pub(crate) async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
        let rows_affected = sqlx::query("update todos set done = true where id = ?1")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected > 0)
    }

    #[derive(Debug, Eq, PartialEq, FromRow, Deserialize, Serialize)]
    pub(crate) struct Todo {
        pub(crate) id: i64,
        pub(crate) description: String,
        pub(crate) done: bool,
    }

    pub(crate) async fn list_todos(pool: &SqlitePool) -> anyhow::Result<Vec<Todo>> {
        let recs = sqlx::query_as::<_, Todo>(
            "select t.id, t.description, t.done from todos t order by id",
        )
        .fetch_all(pool)
        .await?;
        Ok(recs)
    }

    pub(crate) async fn stream_todos(
        pool: &SqlitePool,
    ) -> anyhow::Result<BoxStream<'_, Result<Todo, sqlx::Error>>> {
        let recs = sqlx::query_as::<_, Todo>(
            "select t.id, t.description, t.done from todos t order by id",
        )
        .fetch(pool);
        Ok(recs)
    }
}
