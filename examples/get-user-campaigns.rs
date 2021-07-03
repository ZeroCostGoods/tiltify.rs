use std::env;

use anyhow::anyhow;
use clap::{App, AppSettings, Arg};

struct MyArgs {
    user_id: String,
}

fn get_args() -> tiltify::Result<MyArgs> {
    let args = App::new("get-user-campaigns")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("user-id")
                .long("user-id")
                .required(true)
                .takes_value(true),
        )
        .get_matches();

    Ok(MyArgs {
        user_id: args
            .value_of("user-id")
            .ok_or(anyhow!("Invalid user-id"))?
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
    dbg!(client.user(args.user_id).campaigns().await?);

    Ok(())
}
