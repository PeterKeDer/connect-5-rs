mod room;
mod state;
mod error;
mod user;
mod game_serde;
mod role;

use std::sync::Mutex;
use actix_web::{HttpResponse, web::Data};

pub use state::*;
pub use room::*;
pub use error::*;
pub use user::*;
pub use role::*;

/// Web data, representing state of the app.
pub type AppData = Data<Mutex<AppState>>;

pub type Result<T> = std::result::Result<T, Error>;

/// Result to an API call.
pub type ApiResult = Result<HttpResponse>;
