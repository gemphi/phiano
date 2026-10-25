//! Shared request/response types for all API routes.

mod requests;
mod response;
mod defaults;
pub use requests::*;
pub use response::*;
#[allow(unused_imports)]
use defaults::*;

use serde::{Deserialize, Serialize};
