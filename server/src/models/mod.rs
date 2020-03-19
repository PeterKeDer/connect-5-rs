mod room;
mod state;
mod user;
mod game_serde;
mod role;
mod errors;

use std::sync::Mutex;
use actix_web::{HttpResponse, web::Data};

pub use state::*;
pub use room::*;
pub use user::*;
pub use role::*;
pub use errors::*;

/// Web data, representing state of the app.
pub type AppData = Data<Mutex<AppState>>;

pub type AppResult<T> = std::result::Result<T, AppError>;

/// Result to an API call.
pub type ApiResult = AppResult<HttpResponse>;
