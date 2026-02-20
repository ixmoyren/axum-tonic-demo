use axum::Router;
use axum_tonic_demo::{
    database::{check_and_create_db, migrate},
    grpc::TodoGrpcService,
    pb::todo_server::TodoServer,
    rest::{ApiDoc, todo_router},
};
use scalar_warrper::{Scalar, Servable};
use sqlx::{Sqlite, sqlite::SqlitePoolOptions};
use std::path::Path;
use tokio::net::TcpListener;
use tower::{make::Shared, steer::Steer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = "sqlite://my_db.sqlite?mode=rwc&cache=shared";
    check_and_create_db::<Sqlite>(db)
        .await
        .expect("Check for database exceptions");
    let pool = SqlitePoolOptions::new()
        .connect(db)
        .await
        .expect("Failed to create the database connection pool");
    let conn = pool
        .try_acquire()
        .expect("Unable to get the connection from pool");
    let path = Path::new("migrations");
    migrate(conn, path)
        .await
        .expect("During the migrate phase, the execution of the database script failed");

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/todo", todo_router(pool.clone()))
        .split_for_parts();

    let rest: Router<()> = router.merge(Scalar::with_url("/openapi", api));

    let grpc: Router<()> = tonic::service::Routes::builder()
        .add_service(TodoServer::new(TodoGrpcService::new(pool.clone())))
        .clone()
        .routes()
        .prepare()
        .into_axum_router();

    let service = Steer::new([rest, grpc], |req: &hyper::Request<_>, _services: &[_]| {
        req.headers()
            .get(hyper::header::CONTENT_TYPE)
            .and_then(|content_type| {
                content_type
                    .as_bytes()
                    .starts_with(b"application/grpc")
                    .then_some(1)
            })
            .unwrap_or(0)
    });

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, Shared::new(service)).await?;

    Ok(())
}
