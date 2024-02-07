use std::error::Error;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

#[test]
fn test_reconnect() -> Result<(), Box<dyn Error>> {
    let mut client = DiscordIpcClient::new("771124766517755954")?;
    loop {
        if client.connect().is_ok() {
            break;
        }
    }

    loop {
        let payload = activity::Activity::new()
            .state("part 1 (test)")
            .details("a placeholder")
            .assets(
                activity::Assets::new()
                    .large_image("large-image")
                    .large_text("a thing"),
            );

        if client.set_activity(payload).is_err() && client.reconnect().is_ok() {
            continue;
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    #[allow(unreachable_code)]
    Ok(())
}
