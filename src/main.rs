#![allow(dead_code)]

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use discord_rich_presence::{DiscordIpc, DiscordIpcClient};
use discord_rich_presence::activity::{Activity, Assets, Timestamps};
use log::{error, info, LevelFilter, trace};

use crate::config::RPCProfile;
use crate::prelude::*;

mod config;
mod prelude;
mod error;

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Welcome to Discord Rich Presence!");

    discord_rich_presence();
}


fn discord_rich_presence() {
    /* Load the profiles */
    let mut rpc_config = config::RPCConfig::new();

    trace!("Loaded profiles: {:?}", rpc_config);

    let line = "===============================";

    info!("What profile would you like to use?");
    info!("{}", line);

    for (index, (profile_name, _)) in rpc_config.profiles.iter().enumerate() {
        info!("{}) {}", index+1, profile_name);
    }

    info!("{}", line);

    let mut selected_profile: String;
    loop {
        selected_profile = select_profile();

        if let Ok(index) = selected_profile.parse::<usize>() {
            if index > 0 && index <= rpc_config.profiles.len() {
                selected_profile = rpc_config.profiles.keys().nth(index - 1).unwrap_or_else(|| {
                    error!("There was an error getting the profile");
                    std::process::exit(99);
                }).to_string();
                break;
            }
        }

        if rpc_config.profiles.contains_key(&selected_profile) {
            info!("Selected profile: {}", selected_profile);
            break;
        } else {
            info!("Profile not found, please try again");
        }
    }

    let profile = rpc_config.profiles.remove(&selected_profile).unwrap_or_else(|| {
        error!("There was error getting the profile");
        std::process::exit(99);
    });

    info!("You selected the following profile: {}", selected_profile);

    make_client(profile).unwrap_or_else(|e| {
        error!("Error creating client: {:?}", e);
    });
}

fn select_profile() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap_or_else(|e| {
        log::error!("Fatal Error reading input: {}", e);
        std::process::exit(99);
    });

    input.trim().to_string()
}

fn make_client(profile: RPCProfile) -> Result<()> {
    let mut client = DiscordIpcClient::new(profile.client_id.as_str())?;

    trace!("Connecting to Discord...");

    client.connect()?;

    info!("Connected to Discord!");
    info!("If you want to stop the program, press Ctrl+C");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        info!("Closing connection to Discord...");
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })?;


    while running.load(std::sync::atomic::Ordering::SeqCst) {
        log::debug!("Updating activity...");

        let activity = get_activity(&profile);
        client.set_activity(activity)?;

        std::thread::sleep(std::time::Duration::from_secs(5));
    }

    client.close()?;

    Ok(())
}

fn get_activity(rpc_profile: &RPCProfile) -> Activity {
    let mut activity = Activity::new();
    let twenty_three_hours_ago = chrono::Utc::now() - (chrono::Duration::hours(23) + chrono::Duration::minutes(59) + chrono::Duration::seconds(54));
    let timestamp = Timestamps::new().start(twenty_three_hours_ago.timestamp());
    activity = activity.timestamps(timestamp);

    let mut assets = Assets::new();

    if let Some(large_image) = rpc_profile.large_image.as_ref() {
        assets = assets.large_image(large_image.as_str());
    }
    if let Some(small_image) = rpc_profile.small_image.as_ref() {
        assets = assets.small_image(small_image.as_str());
    }
    if let Some(large_text) = rpc_profile.large_text.as_ref() {
        assets = assets.large_text(large_text.as_str());
    }
    if let Some(small_text) = rpc_profile.small_text.as_ref() {
        assets = assets.small_text(small_text.as_str());
    }

    activity = activity.assets(assets);

    if let Some(details) = rpc_profile.details.as_ref() {
        activity = activity.details(details.as_str());
    }
    if let Some(state) = rpc_profile.state.as_ref() {
        activity = activity.state(state.as_str());
    }

    activity
}
