use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    components::Modal,
    states::{
        global_states::OPEN_CREATE_SERVER_MODAL, server_states::SERVER_LIST, user_states::USER,
    },
};
// const LOADER: Asset = asset!("/assets/loader.svg"); // adjust path to your actual loader asset

#[component]
pub fn CreateServerModal() -> Element {
    let mut name_text = use_signal(|| String::new());
    let mut description_text = use_signal(|| String::new());
    let mut image_text = use_signal(|| String::new());
    let loading = use_signal(|| false);

    rsx! {
        Modal {
            open: OPEN_CREATE_SERVER_MODAL.signal(),
            div {
                class: "font-light text-4xl",
                "Create New Server"
            }
            div {
                class: "flex gap-2",
                div {
                    class: "bg-deep-purple-100 rounded-xl w-32 h-32",
                    if *image_text.read() == "".to_string() {
                        div { }
                    } else {
                        img {
                            class: "w-32 h-32 rounded-xl object-cover",
                            src: "{image_text}"
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    input {
                        class: "bg-deep-purple-400 p-2 rounded-md",
                        placeholder: "server name",
                        type: "text",
                        value: "{name_text}",
                        oninput: move |evt| name_text.set(evt.value()),
                    }
                    input {
                        class: "bg-deep-purple-400 p-2 rounded-md",
                        placeholder: "description",
                        type: "text",
                        value: "{description_text}",
                        oninput: move |evt| description_text.set(evt.value()),
                    }
                    input {
                        class: "bg-deep-purple-400 p-2 rounded-md",
                        placeholder: "server image link address",
                        type: "text",
                        value: "{image_text}",
                        oninput: move |evt| image_text.set(evt.value()),
                    }
                }
            }
            div {
                class: "flex gap-2 pt-2",
                button {
                    class: "bg-deep-purple-400 text-deep-purple-100 p-2 w-32 rounded-md duration-100 border-2 border-deep-purple-400 hover:border-deep-purple-100",
                    onclick: move |_| create_server(name_text, description_text, image_text, loading),
                    "Create Server"
                }
                button {
                    class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
                    "Cancel"
                }
            }
        }
    }
}

// if !*OPEN_CREATE_SERVER_MODAL.read() {
//     return rsx! {};
// }
// rsx! {
//     div {
//         class: "relative z-50 select-none",
//         // backdrop
//         div {
//             class: "fixed inset-0 bg-black/50",
//             onclick: move |_| *OPEN_CREATE_SERVER_MODAL.write() = false,
//         }
//         // center wrapper
//         div {
//             class: "fixed inset-0 flex w-screen items-center justify-center",
//             onclick: move |_| *OPEN_CREATE_SERVER_MODAL.write() = false,
//             // modal
//             div {
//                 class: "flex flex-col p-10 max-w-lg space-y-1 bg-deep-purple-300 text-deep-purple-100 select-none rounded-xl",
//                 onclick: move |evt| evt.stop_propagation(),
//                 div {
//                     class: "font-light text-4xl",
//                     "Create New Server"
//                 }
//                 div {
//                     class: "flex gap-2",
//                     div {
//                         class: "bg-deep-purple-100 rounded-xl w-32 h-32",
//                         if *image_text.read() == "".to_string() {
//                             div { }
//                         } else {
//                             img {
//                                 class: "w-32 h-32 rounded-xl object-cover",
//                                 src: "{image_text}"
//                             }
//                         }
//                     }
//                     div {
//                         class: "flex flex-col gap-1",
//                         input {
//                             class: "bg-deep-purple-400 p-2 rounded-md",
//                             placeholder: "server name",
//                             type: "text",
//                             value: "{name_text}",
//                             oninput: move |evt| name_text.set(evt.value()),
//                         }
//                         input {
//                             class: "bg-deep-purple-400 p-2 rounded-md",
//                             placeholder: "description",
//                             type: "text",
//                             value: "{description_text}",
//                             oninput: move |evt| description_text.set(evt.value()),
//                         }
//                         input {
//                             class: "bg-deep-purple-400 p-2 rounded-md",
//                             placeholder: "server image link address",
//                             type: "text",
//                             value: "{image_text}",
//                             oninput: move |evt| image_text.set(evt.value()),
//                         }
//                     }
//                 }
//                 div {
//                     class: "flex gap-2 pt-2",
//                     button {
//                         class: "bg-deep-purple-400 text-deep-purple-100 p-2 w-32 rounded-md duration-100 border-2 border-deep-purple-400 hover:border-deep-purple-100",
//                         onclick: move |_| create_server(name_text, description_text, image_text, loading),
//                         "Create Server"
//                     }
//                     button {
//                         class: "bg-slate-900 p-2 w-20 rounded-md p-2 w-20 border-2 border-slate-900 hover:border-red-800",
//                         "Cancel"
//                     }
//                 }
//             }
//         }
//     }
// }

async fn create_server(
    name_text: Signal<String>,
    description_text: Signal<String>,
    image_text: Signal<String>,
    mut loading: Signal<bool>,
) {
    loading.set(true);
    let name = name_text.read().clone();
    let description = description_text.read().clone();
    let image = image_text.read().clone();
    if name != "".to_string() {
        let address = USER
            .read()
            .profile
            .as_ref()
            .expect("No User Profile")
            .address
            .clone();
        let app_state = APP_STATE.lock().await;
        if app_state.xmtp.is_some() {
            let db_arc = app_state
                .db
                .as_ref()
                .expect("Database no initialized")
                .clone();
            let db = db_arc.lock().await;
            let metadata = db
                .create_server(&name, &image, &address)
                .await
                .map_err(|e| format!("could not create server: {e}"))
                .expect("");
            let members: Vec<Recipient> = vec![Recipient::Address(address)];
            let new_group_id = app_state
                .xmtp
                .as_ref()
                .expect("failed to get xmtp instance")
                .create_group(&members, Some("general".to_string()), None, None)
                .expect("failed to create new group");
            match db
                .add_text_channel(&metadata.id, "general", &new_group_id)
                .await
            {
                Ok(result) => {
                    println!("Created server!!!");
                    SERVER_LIST.write().insert(0, metadata);
                    // let server_list =
                    // SERVER_LIST.write()
                }
                Err(e) => {
                    println!("failed to create server");
                }
            }
        }
        loading.set(false);
        *OPEN_CREATE_SERVER_MODAL.write() = false;
    }
}
