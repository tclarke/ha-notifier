use rumqttc::v5::{
    mqttbytes::QoS,
    AsyncClient, Event, Incoming, MqttOptions,
};
use std::{env, error::Error, time::Duration};

fn usage() -> &'static str {
    "usage: mqtt-v5-publish [--url URL] [--username USER] [--password PASS] <subtopic> <message...>"
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mut url = env::var("HA_NOTIFIER_URL").ok();
    let mut username = env::var("HA_NOTIFIER_USERNAME").ok();
    let mut password = env::var("HA_NOTIFIER_PASSWORD").ok();
    let mut positional = Vec::new();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => return Err(usage().into()),
            "--url" | "-u" => url = Some(args.next().ok_or(usage())?),
            "--username" | "-n" => username = Some(args.next().ok_or(usage())?),
            "--password" | "-p" => password = Some(args.next().ok_or(usage())?),
            "--" => break,
            _ if arg.starts_with('-') => return Err(format!("unknown option: {arg}\n{}", usage()).into()),
            _ => positional.push(arg.clone()),
        }
    }
    positional.extend(args);

    println!("{:?}", url);
    let mut url = url.ok_or("missing connection URL; use --url or HA_NOTIFIER_URL")?;
    let subtopic = positional.first().ok_or(usage())?;
    let message = positional[1..].join(" ");

    if message.is_empty() {
        return Err(usage().into());
    }

    if url.find("client_id").is_none() {
        url += "?client_id=ha-notifier";
    }
    let topic = format!("notification/{subtopic}");
    let mut options = MqttOptions::parse_url(&url)?;
    options.set_keep_alive(Duration::from_secs(30))
            .set_client_id("ha-notifier".to_string());

    if let Some(username) = username {
        options.set_credentials(username, password.unwrap_or_default());
    }

    let (client, mut eventloop) = AsyncClient::new(options, 10);

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(async {
        client
            .publish(topic, QoS::AtLeastOnce, false, message.into_bytes())
            .await?;

        loop {
            if let Event::Incoming(Incoming::PubAck(puback)) = eventloop.poll().await? {
                if puback.reason == rumqttc::v5::mqttbytes::v5::PubAckReason::Success {
                    println!("Publish acknowledged successfully: {:?}", puback);
                    break;
                }

                return Err(format!("Publish rejected: {:?}", puback.reason).into());
            }
        }
        Ok::<(), Box<dyn Error>>(())
    })?;

    Ok(())
}
