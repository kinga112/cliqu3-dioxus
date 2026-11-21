use dioxus::prelude::*;

use crate::{modules::xmtp::xmtp::XMTP_STATE, state::data_types::server::TextChannelState};

#[component]
pub fn DirectMessageNavButton(id: String, from: String, message: String) -> Element {
    rsx! {
        button {
            class: "bg-off-black-500 px-2 py-1 w-full shrink hover:bg-off-black-400 rounded-lg",
            onclick: move |_| {
                set_current_dm(id.clone())
                // let id_clone = id.clone();
                // spawn(async move {
                //     set_current_dm(id_clone).await
                // });
            },
            div {
                class: "flex gap-2 place-items-center",
                div {
                    class: "w-12 h-12 bg-blue-300 rounded-md shrink-0 object-cover select-none",
                }
                // img {
                //     class: "w-12 h-12 rounded-md shrink-0 object-cover select-none",
                //     src: ""
                // }
                div {
                    class: "flex flex-col w-32",
                    p { class: "truncate w-full text-xl text-left",
                        "{from}"
                    }
                    p { class: "truncate font-light w-full text-white text-opacity-50 text-left",
                        "{message}"
                    }
                }
            }
        }
    }
}

async fn set_current_dm(id: String) {
    let mut channel_state = use_context::<TextChannelState>();
    // let app_state = APP_STATE.lock().await;
    // let xmtp = app_state.xmtp.as_ref().expect("xmtp is none??");
    let xmtp_opt = XMTP_STATE.lock().await;
    let xmtp = xmtp_opt.as_ref().expect("xmtp is none as option here 3?");
    // let xmtp_handler = app_state.xmtp_handler.as_ref().expect("xmtp is none??");
    // let convo = xmtp
    //     .client
    //     .conversation(&id)
    //     .expect("failed to get convo by id")
    //     .expect("convo with id does not exist");
    let dm = xmtp
        .get_conversation(&id)
        .await
        .expect("failed to get dm text channel");
    // let (output_tx, output_rx) = crossbeam::channel::unbounded::<TextChannel>();
    // let send_result = xmtp_handler
    //     .cmd_tx
    //     .send(XmtpCommand::GetConversation { output_tx, id });

    // let dm = output_rx
    //     .recv()
    //     .expect("failed to get output result for dm convo");

    // Spawn Blocking is needed so main thread doesnt freeze
    // dioxus spawned tasks are not on seperate thread or thread safe
    // let dm = tokio::task::spawn_blocking(move || {
    //     output_rx
    //         .recv()
    //         .expect("failed to get output result for dm convo")
    // })
    // .await
    // .expect("failed to get output for dm conversaion");
    // match output_result {
    //     Ok(dm) => dm,
    //     Err(e) => Vec::new(),
    // }

    // println!("NEW DM MEMBERS: {:?}", dm.members.clone());
    *channel_state.outbox_messages.write() = Vec::new();
    *channel_state.reply.write() = None;
    *channel_state.messages.write() = dm.messages.clone();
    *channel_state.members.write() = dm.members.clone();
    *channel_state.metadata.write() = dm.metadata.clone();
    // *DM_MESSAGES.write() = dm.messages.clone();
    // *CURRENT_DIRECT_MESSAGE.write() = Some(dm);
}
