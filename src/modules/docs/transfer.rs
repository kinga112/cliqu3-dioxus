use anyhow::{Error, Result, anyhow};
use dioxus::signals::{Signal, WritableExt};
use futures::StreamExt;
use iroh::endpoint::presets;
use iroh::{Endpoint, SecretKey, protocol::Router};
use iroh_blobs::api::remote::GetProgressItem;
use iroh_blobs::provider::events::{ConnectMode, EventMask, EventSender, ProviderMessage};
use iroh_blobs::store::mem::MemStore;
use iroh_blobs::ticket::BlobTicket;
use iroh_blobs::{ALPN as BLOBS_ALPN, BlobsProtocol};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::mpsc::Sender;

pub async fn send_bytes_archive(bytes: Vec<u8>, mut ui_tx: Sender<bool>) -> anyhow::Result<String> {
    let secret_key = SecretKey::generate();
    let endpoint = Endpoint::builder(presets::N0)
        .alpns(vec![iroh_blobs::protocol::ALPN.to_vec()])
        .secret_key(secret_key)
        .relay_mode(iroh::RelayMode::Default)
        .bind()
        .await?;
    let store = MemStore::default();

    // let (ui_tx, mut ui_rx) = tokio::sync::mpsc::channel(16);
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);
    let events = EventSender::new(
        tx,
        EventMask {
            connected: ConnectMode::Notify,
            // get: iroh_blobs::provider::events::RequestMode::NotifyLog,
            // get_many: RequestMode::NotifyLog,
            ..EventMask::DEFAULT
        },
    );
    let blobs = BlobsProtocol::new(&store, Some(events));
    let router = Router::builder(endpoint).accept(BLOBS_ALPN, blobs).spawn();

    let ep = router.endpoint();
    tokio::time::timeout(Duration::from_secs(30), async move {
        let _ = ep.online().await;
    })
    .await?;

    let addr = router.endpoint().addr();
    let tag_info = store.add_slice(&bytes).await?;
    let ticket = BlobTicket::new(addr, tag_info.hash, tag_info.format);

    tokio::spawn(async move {
        let ep = router.endpoint();
        let _ = ep.online().await;
        loop {
            tokio::select! {
                item = rx.recv() => {
                    let Some(item) = item else {
                        println!(
                            "
                            *
                            *
                            *
                            ITEM ELSE {:?}
                            *
                            *
                            *", item
                        );
                        // tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        // continue;
                        break;
                    };
                    match item {
                        ProviderMessage::ClientConnectedNotify(msg) => {
                            // sender_text.set("Reciver Connected, Sending Archive!".to_string());
                            ui_tx.send(true).await.expect("failed to send closed message");
                            // println!("*********sender: client connected {:?}", msg.endpoint_id);
                            // println!(
                            //     "
                            //     *
                            //     *
                            //     *
                            //     1
                            //     *
                            //     *
                            //     *"
                            // );
                        }
                        ProviderMessage::ConnectionClosed(msg) => {
                            ui_tx.send(false).await.expect("failed to send closed message");
                            // let a = ui_tx
                            // sender_text.set(format!("Finished Send"));
                            // sender_text.write() = format!("Finished Send")
                            // println!("*********sender: connection closed {:?}", msg.connection_id);
                            // println!(
                            //     "
                            //     *
                            //     *
                            //     *
                            //     2
                            //     *
                            //     *
                            //     *"
                            // );
                            ep.close().await;
                            let shutdown_result = router.shutdown().await;
                            break;
                        }
                        _ => {
                            println!("OTHER MESSAGE");
                        }
                    }

                }
            }
        }
    });
    Ok(ticket.to_string())
}

pub async fn receive_bytes_archive(
    ticket_string: String,
    mut receiver_text: Signal<String>,
) -> anyhow::Result<Vec<u8>> {
    let ticket = BlobTicket::from_str(&ticket_string)?;
    let addr = ticket.addr().clone();
    let endpoint = Endpoint::builder(presets::N0)
        .alpns(vec![BLOBS_ALPN.to_vec()])
        .bind()
        .await
        .expect("fail 3");
    let store = MemStore::new();
    let connection = endpoint.connect(addr, BLOBS_ALPN).await.expect("fail 1");
    let get = store.remote().fetch(connection, ticket.hash_and_format());
    let mut stream = get.stream();
    while let Some(item) = stream.next().await {
        match item {
            GetProgressItem::Progress(offset) => {
                receiver_text.set(format!("Downloading: {} bytes", offset));
            }
            GetProgressItem::Done(value) => {
                receiver_text.set(format!(
                    "Completed: {} bytes in {:?}",
                    value.payload_bytes_read, value.elapsed
                ));
                break;
            }
            GetProgressItem::Error(cause) => {
                println!("*********receiver: ERROR {:?}", cause);
                println!(
                    "
                    *
                    *
                    *
                    3
                    *
                    *
                    *"
                );
            }
        }
    }
    let bytes = store
        .blobs()
        .get_bytes(ticket.hash())
        .await
        .expect("fail 2");
    endpoint.close().await;
    Ok(bytes.to_vec())
}
