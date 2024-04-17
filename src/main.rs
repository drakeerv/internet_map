use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

const DB_NAME: &str = "db";
const PUBLIC_DIR: &str = "public";

#[derive(Clone)]
struct AppState {
    db: sled::Db,
}

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    lat: f64,
    lon: f64,
    date: u64,
}

#[tokio::main]
async fn main() {
    // get env vars for ip and port
    let ip = std::env::var("IP").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", ip, port);

    // set up sled
    let db_options = sled::Config::default().path(DB_NAME);
    let db = db_options.open().unwrap();

    // create our application state
    let state = AppState { db: db.clone() };

    // build our application with a route
    let app = Router::new()
        // serve static files
        .nest_service("/", ServeDir::new("./".to_owned() + PUBLIC_DIR))
        // add routes
        .nest(
            "/api",
            Router::new().nest(
                "/v1",
                Router::new()
                    .route("/add-location", post(add_location))
                    .route("/get-locations", get(get_locations)),
            ),
        )
        // create a state that holds our database
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on: http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn add_location(State(state): State<AppState>, location: Json<Location>) -> Json<Location> {
    let locations = state.db.open_tree("locations").unwrap();
    let key = uuid::Uuid::new_v4().to_string();
    let value = bincode::serialize(&location.0).unwrap();
    locations.insert(key, value).unwrap();

    location
}

async fn get_locations(State(state): State<AppState>) -> Json<Vec<Location>> {
    let locations = state.db.open_tree("locations").unwrap();
    let mut result = Vec::with_capacity(locations.len());

    for location in locations.iter() {
        let (_, value) = location.unwrap();
        let location: Location = bincode::deserialize(&value).unwrap();
        result.push(location);
    }

    Json(result)
}
