use std::sync::Arc;

use axum::{
    Router,
    http::{Method, header::CONTENT_TYPE},
};
use dotenvy::dotenv;
use shaku::HasComponent;
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod models;
mod modules;

use modules::notes::{
    AppState, NoteRepositoryImpl, NoteRepositoryImplParameters, NoteService, NotesModule,
};
use tower_http::cors::{Any, CorsLayer};

#[derive(OpenApi)]
#[openapi(
    paths(
        modules::commons::handler::health,
        modules::notes::handler::get_list_note_handler,
        modules::notes::handler::create_note_handler,
        modules::notes::handler::update_note_handler
    ),
    components(schemas(
        models::model::NoteModel,
        models::model::NoteModelResponse,
        modules::notes::CreateNoteSchema,
        modules::notes::UpdateNoteSchema,
        modules::notes::FilterOptions
    )),
    info(
        title = "Vibe Rust API",
        description = "A simple note-taking API built with Rust, Axum, and PostgreSQL",
        version = "0.1.0"
    )
)]
struct ApiDoc;

use modules::commons::routes::create_commons_router;
use modules::notes::routes::create_notes_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
    {
        Ok(pool) => pool,
        Err(e) => {
            eprintln!("Failed to connect to database: {:?}", e);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let pool = Arc::new(pool);

    let notes_module = NotesModule::builder()
        .with_component_parameters::<NoteRepositoryImpl>(NoteRepositoryImplParameters {
            pool: Arc::clone(&pool),
        })
        .build();

    let note_service: Arc<dyn NoteService> = notes_module.resolve();
    let app_state = Arc::new(AppState { note_service });

    let api_docs = ApiDoc::openapi();

    let app = Router::new()
        .nest(
            "/api/v1",
            create_commons_router().merge(create_notes_router(Arc::clone(&app_state))),
        )
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api_docs));

    println!("Server is running on port 8080");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
