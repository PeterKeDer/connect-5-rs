mod room;
mod state;
mod error;
mod user;
mod game_serde;

use std::sync::Mutex;
use actix_web::{HttpResponse, web::Data};

pub use state::*;
pub use room::*;
pub use error::*;
pub use user::*;

/// Web data, representing state of the app.
pub type State = Data<Mutex<AppState>>;

/// Result to an API call.
pub type ApiResult = Result<HttpResponse, JsonError>;
