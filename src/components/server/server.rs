use dioxus::prelude::*;

use crate::{
    components::server::{channel::TextChannel, sidebar::SideBar},
    modules::docs::db::Server as ServerType,
    states::server_states::{CURRENT_SERVER, CURRENT_TEXT_CHANNEL},
};

#[component]
// pub fn Server(server: ServerType) -> Element {
pub fn Server() -> Element {
    // let server_state = use_context_provider(|| server);
    // let server = use_context_provider(|| CURRENT_SERVER.read().clone().expect("server is none?"));
    // println!("SERVER ID INSIDE SERVER: {:?}", server.metadata.id);
    // let text_channel = CURRENT_TEXT_CHANNEL
    //     .read()
    //     .clone()
    //     .expect("current text channel is none");
    // let server = CURRENT_SERVER.read().clone();
    // if server.is_none() {
    //     println!("SERVER IS NONE??");
    //     return rsx! {};
    // }
    // println!(
    //     "SERVER IN SERVER COMP: {:?}",
    //     server.clone().unwrap().metadata.name
    // );
    rsx! {
        div {
            class: "flex h-screen w-full bg-off-black-400",
            // SideBar{server: server.clone().unwrap()}
            SideBar{}
            TextChannel{}
        }
    }
}
