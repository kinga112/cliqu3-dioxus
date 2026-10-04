use dioxus::prelude::*;

use crate::{
    components::{
        dms::DirectMessages,
        server::Server,
        settings::{Call, Settings},
        sidenav::{SideNav, options_menu::CreateServerModal},
    },
    states::{
        global_states::{CURRENT_SCREEN, Screen},
        server_states::CURRENT_SERVER,
    },
};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    // let server_opt = CURRENT_SERVER.read().as_ref().clone();
    rsx! {
        div {
            class: "flex relative bg-off-black-700 h-screen w-screen text-deep-purple-100 overflow-hidden",
            SideNav{}
            div {
                class: "w-full h-full",
                match *CURRENT_SCREEN.read() {
                    Screen::Server => rsx! {
                        Server{}
                        // if let Some(server) = CURRENT_SERVER.read().clone() {
                        //     Server { server: server }
                        // }
                    },
                    Screen::DirectMessages => rsx! { DirectMessages{} },
                    Screen::Settings => rsx! { Settings{} },
                }
            }
            CreateServerModal{}
        }

    }
}
