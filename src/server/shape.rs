//! Shared request/response types for all API routes.

mod request;
mod response;
mod default;
pub use request::*;
pub use response::*;
#[allow(unused_imports)]
use default::*;

use serde::{Deserialize, Serialize};
