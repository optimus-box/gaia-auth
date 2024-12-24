use axum::Router;
use controller::{auth_controller, group_controller, profile, user_controller};
use dotenvy::dotenv;
use model::{GroupDto, UserCreateDto};
use repository::{GroupRepository, UserRepository};
use sqlx::{Pool, Postgres};
use tokio::net::TcpListener;

mod controller;
mod model;
mod repository;
mod security;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // check password salt
    let hex = std::env::var("PASSWORD_SALT").expect("PASSWORD_SALT must be set");
    // try to convert hex to bytes
    security::password::hex_to_bytes(&hex);
    // connect to database
    let db = database().await;
    // run migrations
    migrate(db.clone()).await;
    // seed database
    seed(db.clone()).await;
    // start http server
    http(db).await;
}

async fn database() -> Pool<Postgres> {
    let host = std::env::var("DATABASE_HOST").expect("DATABASE_HOST must be set");
    let port = std::env::var("DATABASE_PORT").unwrap_or(String::from("5432"));
    let user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set");
    let pass = std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set");
    let name = std::env::var("DATABASE_NAME").unwrap_or(String::from("postgres"));
    let db_url = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, name);
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to database")
}

async fn migrate(db: Pool<Postgres>) {
    sqlx::migrate!()
        .run(&db)
        .await
        .expect("failed to run migrations");
}

async fn seed(db: Pool<Postgres>) {
    init_system(db).await;
}

async fn init_system(db: Pool<Postgres>) {
    let repo = GroupRepository::new(db.clone());
    let urepo = UserRepository::new(db);
    if repo.count().await.unwrap() > 0 {
        return;
    }
    let dto = GroupDto {
        name: String::from("root"),
        description: Some(String::from("super users group")),
        permissions: vec![String::from("root")],
        visible: Some(false),
        editable: Some(false),
        locked: Some(true),
    };
    match repo.create(dto).await {
        Ok(group) => {
            let hash = security::password::hash("root").expect("failed to hash password");
            let dto = UserCreateDto {
                name: String::from("root"),
                phone: None,
                role: None,
                email: String::from("root@change.me"),
                username: String::from("root"),
                password: String::from("root"),
                password_hash: hash,
                visible: false,
                editable: false,
                locked: true,
                groups: vec![group.id],
            };
            if let Err(e) = urepo.create(dto).await {
                panic!("failed to create root user: {}", e);
            }
        }
        Err(e) => {
            panic!("failed to create root group: {}", e);
        }
    }

    let dto = GroupDto {
        name: String::from("admin"),
        description: Some(String::from("admin users group")),
        permissions: vec![String::from("admin")],
        visible: Some(true),
        editable: Some(false),
        locked: Some(true),
    };
    match repo.create(dto).await {
        Ok(group) => {
            let hash = security::password::hash("admin").expect("failed to hash password");
            let dto = UserCreateDto {
                name: String::from("admin"),
                phone: None,
                role: None,
                email: String::from("admin@change.me"),
                username: String::from("admin"),
                password: String::from("admin"),
                password_hash: hash,
                visible: true,
                editable: false,
                locked: true,
                groups: vec![group.id],
            };
            if let Err(e) = urepo.create(dto).await {
                panic!("failed to create admin user: {}", e);
            }
        }
        Err(e) => {
            panic!("failed to create admin group: {}", e);
        }
    }

    let dto = GroupDto {
        name: String::from("nobody"),
        description: Some(String::from("nobody users group")),
        permissions: vec![String::from("nobody")],
        visible: Some(false),
        editable: Some(false),
        locked: Some(true),
    };
    if let Err(e) = repo.create(dto).await {
        panic!("failed to create nobody group: {}", e);
    }
}

async fn http(db: Pool<Postgres>) {
    let group_repo = GroupRepository::new(db.clone());
    let user_repo = UserRepository::new(db);

    let host = std::env::var("HTTP_HOST").unwrap_or(String::from("0.0.0.0"));
    let port = std::env::var("HTTP_PORT").unwrap_or(String::from("4000"));
    let addr = format!("{}:{}", host, port);

    let tcp = TcpListener::bind(addr)
        .await
        .expect("failed to bind to address");
    let app = Router::new()
        .nest("/groups", group_controller::routes(group_repo))
        .nest("/users", user_controller::routes(user_repo.clone()))
        .nest("/profile", profile::routes(user_repo.clone()))
        .nest("/", auth_controller::router(user_repo));
    axum::serve(tcp, app).await.expect("failed to start server");
}
