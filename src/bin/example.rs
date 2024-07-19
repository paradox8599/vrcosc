use std::{str::FromStr, sync::Arc, time::Duration};

use eyre::Result;
use vrcosc::vrchat::client::VrchatClient;

#[tokio::main]
async fn main() -> Result<()> {
    // let mut raw_client = VrchatClient::default();
    //
    // raw_client.bind().await?;
    // println!("ports bound");
    //
    // let client = Arc::new(raw_client);
    // let client_listen = client.clone();
    //
    // let listen_handler = tokio::spawn(async move {
    //     fn on_message(msg: &VrcMessage) {
    //         println!("{:?}", msg);
    //     }
    //     client_listen.listen(on_message).await;
    // });
    //
    // tokio::time::sleep(Duration::from_secs(3)).await;
    // listen_handler.abort();
    //
    // let msg = VrcMessage {
    //     addr: "/test".to_string(),
    //     value: VrcType::Bool(true),
    // };
    // client.send(msg).await?;
    // println!("message sent");

    Ok(())
}
