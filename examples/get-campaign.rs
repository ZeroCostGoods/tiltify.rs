use std::env;

use anyhow::anyhow;
use clap::{App, AppSettings, Arg};

struct MyArgs {
    campaign_id: String,
}

fn get_args() -> tiltify::Result<MyArgs> {
    let args = App::new("get-campaign")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("campaign-id")
                .long("campaign-id")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    Ok(MyArgs {
        campaign_id: args
            .value_of("campaign-id")
            .ok_or(anyhow!("Invalid campaign-id"))?
            .into(),
    })
}

#[tokio::main]
async fn main() -> tiltify::Result<()> {
    let token = match env::var("TILTIFY_ACCESS_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            return Err(anyhow!(
                "Must pass TILTIFY_ACCESS_TOKEN environment variable."
            ))
        }
    };

    let args = get_args()?;

    let client = tiltify::client::TiltifyClient::new(token)?;

    let campaign = client.campaign(args.campaign_id).get().await?;
    dbg!(campaign);
    Ok(())
}
