mod amqp;
mod config;
mod gateway;

use anyhow::Result;
use futures_util::StreamExt;
use poketwo_protobuf_rust::poketwo::gateway::v1::MessageCreate;
use prost::Message;
use twilight_gateway::Event;

use crate::amqp::Amqp;
use crate::config::CONFIG;
use crate::gateway::Gateway;

#[tokio::main]
async fn main() -> Result<()> {
    let amqp = Amqp::connect(&CONFIG).await?;
    let mut gateway = Gateway::connect(&CONFIG).await?;

    while let Some(event) = gateway.events.next().await {
        gateway.cache.update(&event);

        let (payload, routing_key) = match event {
            Event::MessageCreate(data) => {
                (MessageCreate::from(*data).encode_to_vec(), "MESSAGE_CREATE")
            }
            _ => continue,
        };

        if let Err(x) = amqp.publish(&payload, routing_key).await {
            dbg!(x);
        }
    }

    Ok(())
}
