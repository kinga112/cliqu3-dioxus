use alloy::hex;
use dioxus::prelude::*;
use iroh_live::media::AudioBackend;

use crate::{
    app_state::APP_STATE,
    modules::docs::transfer::{receive_bytes_archive, send_bytes_archive},
};

#[component]
pub fn Archives() -> Element {
    let ticket_key_signal = use_signal(|| String::new());
    let mut ticket_key_input_signal = use_signal(|| String::new());
    let sender_text = use_signal(|| String::new());
    let receiver_text = use_signal(|| String::new());
    let mut send = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col bg-off-black-500 w-full p-20 gap-5 overflow-y-auto",
            div {
                class: "text-3xl",
                "Transfer Message History"
            }
            div {
                class: "text-xl",
                "Transfer XMTP messages to another installation or save an Archive to a local file."
            }
            div {
                class: "flex flex-col bg-off-black-400 p-5 gap-10 rounded-lg overflow-hidden shrink-0",
                div {
                    class: "flex justify-center place-items-center gap-4 w-full",
                    button {
                        class: "p-2 w-full rounded",
                        class: if *send.read() {"bg-off-black-200"} else {"hover:bg-off-black-300"},
                        onclick: move |_| send.set(true),
                        "Send"
                    }
                    button {
                        class: "p-2 w-full rounded",
                        class: if !*send.read() {"bg-off-black-200"} else {"hover:bg-off-black-300"},
                        onclick: move |_| send.set(false),
                        "Receive"
                    }
                }
                div {
                    class: "flex overflow-hidden duration-300 w-[200%]",
                    class: if !*send.read() {"-translate-x-1/2"},
                    div {
                        class: "flex flex-col justify-center place-items-center w-full gap-5 p-5",
                        div {
                            class: "text-xl",
                            "Transfer Archive to another device via internet sync"
                        }
                        div {
                            class: "text-sm",
                            "Eventually Add Send Options"
                        }
                        div {
                            class: "px-20 h-full text-2xl text-deep-purple-300 font-light break-all",
                            if *ticket_key_signal.read() != "" {
                                "Give this Code to your other device: {*ticket_key_signal.read()}"
                            }
                        }
                        div {
                            class: "px-20 h-full text-2xl text-deep-purple-300 font-light break-all",
                            if *sender_text.read() != "" {
                                "{*sender_text.read()}"
                            }
                        }
                        button {
                            class: "bg-deep-purple-100 text-deep-purple-300 font-semibold rounded p-2",
                            onclick: move |_| send_archive(ticket_key_signal, sender_text),
                            "Start Transfer"
                        }
                    }
                    div {
                        class: "flex flex-col justify-center place-items-center w-full gap-5 p-5",
                        div {
                            class: "text-xl",
                            "Receive an archive from another device"
                        }
                        div {
                            class: "text-sm",
                            "Input your receive code below then start the receive!"
                        }
                        input {
                            class: "h-14 bg-off-black-100 w-96 p-2 rounded-lg",
                            placeholder: "Input CODE from sender device here",
                            oninput: move |evt| ticket_key_input_signal.set(evt.value()),
                        }
                        div {
                            class: "px-20 h-full text-2xl text-deep-purple-300 font-light break-all",
                            if *receiver_text.read() != "" {
                                "{*receiver_text.read()}"
                            }
                        }
                        button {
                            class: "bg-deep-purple-100 text-deep-purple-300 font-semibold rounded p-2",
                            onclick: move |_| receive_archive(ticket_key_input_signal.read().clone(), receiver_text),
                            "Receive Transfer"
                        }
                    }
                }
            }
        }
    }
}

async fn send_archive(mut ticket_key_signal: Signal<String>, mut sender_text: Signal<String>) {
    sender_text.set("Starting Send Process!".to_string());
    let app_state = APP_STATE.lock().await;
    let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
    let (result_bytes, key) = xmtp.create_bytes_archive();
    // key_signal.set(key);
    // let key_str = str::from_utf8(&key).expect("Failed to get key as string");
    // let key_hex_string: String = key.iter().map(|b| format!("{:02x}", b)).collect();
    let key_hex_string = hex::encode(key);
    match result_bytes {
        Ok(bytes) => {
            println!("GOT ARCHIVE BYTES");
            let (ui_tx, mut ui_rx) = tokio::sync::mpsc::channel(16);
            let ticket_string = send_bytes_archive(bytes, ui_tx)
                .await
                .expect("failed to start send bytes archive");
            let ticket_key = format!("{}archivekey{}", ticket_string, key_hex_string);
            // let endpoint2 = sender.endpoint;
            ticket_key_signal.set(ticket_key);
            // tokio::spawn(async move {
            //     while let Some(connected) = ui_rx.recv().await {
            // if connected == false {
            //     break;
            // } else {
            //     println!("Connected!");
            // }
            //     }
            // });

            while let Some(connected) = ui_rx.recv().await {
                if connected == false {
                    sender_text.set("Archive Successfully Sent!".to_string());
                    // break;
                } else {
                    sender_text.set("Receiver Connected".to_string());
                    // println!("Connected!");
                }
            }
        }
        Err(e) => {
            println!("Error creating bytes archive: {:?}", e);
        }
    }
}

async fn receive_archive(ticket_key: String, mut receiver_text: Signal<String>) {
    receiver_text.set("Waiting to connect to Sender!".to_string());
    // let (ticket_str, hex_key_str) = ticket_key
    //     .rsplit_once("archivekey")
    //     .expect("failed to parse ticket_key");
    let mut ticket_key_split = ticket_key.splitn(2, "archivekey");
    let ticket_string = ticket_key_split
        .next()
        .ok_or("Missing ticket")
        .expect("failed 1")
        .to_string();
    let key_hex_str = ticket_key_split
        .next()
        .ok_or("Missing ticket key")
        .expect("failed to get key hex");
    let mut key = [0u8; 32];
    match hex::decode_to_slice(key_hex_str, &mut key) {
        Ok(()) => println!("Successfully decoded back to fixed array: {:?}", key),
        Err(e) => println!("Decoding error: {}", e),
    }
    let data = match receive_bytes_archive(ticket_string, receiver_text).await {
        Ok(d) => d,
        Err(e) => {
            println!("receive failed: {e:?}");
            return;
        }
    };
    let app_state = APP_STATE.lock().await;
    let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
    let result = xmtp.import_bytes_archive(&data, &key);
    match result {
        Ok(()) => {
            println!("RECEVIVED ARCHIVE!!");
        }
        Err(e) => {
            println!("Error receiving bytes archive: {:?}", e);
        }
    }
}
