//! This library provides easy access to the Discord IPC.
//!
//! It provides implementations for both Unix and Windows
//! operating systems, with both implementations using the
//! same API. Thus, this crate can be used in a platform-agnostic
//! manner.
//!
//! # Hello world
//! ```
//! use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut client = DiscordIpcClient::new("<some client id>")?;
//!     client.connect()?;
//!
//!     let payload = activity::Activity::new().state("Hello world!");
//!     client.set_activity(payload)?;
//! }
//! ```
pub use discord_ipc::*;
pub use ipc::DiscordIpcClient;
#[cfg(unix)]
use ipc_unix as ipc;
#[cfg(windows)]
use ipc_windows as ipc;
use prelude::*;

mod discord_ipc;
mod pack_unpack;
pub mod activity;
pub mod error;
pub mod prelude;

#[cfg(unix)]
mod ipc_unix;
#[cfg(windows)]
mod ipc_windows;

#[deprecated(since = "0.2.0", note = "use DiscordIpcClient::new() instead")]
/// Creates a new client to connect to the Discord IPC. Functionally
/// identical to [`DiscordIpcClient::new()`].
///
/// # Examples
/// ```
/// let ipc_client = discord_ipc_client::new_client("<some client id>")?;
/// ```
pub fn new_client(client_id: &str) -> Result<impl DiscordIpc> {
    ipc::DiscordIpcClient::new(client_id)
}
