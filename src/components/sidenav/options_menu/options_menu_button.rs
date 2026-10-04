use crate::{
    components::sidenav::options_menu::options_menu::OptionsMenuButtonType,
    states::global_states::OPEN_CREATE_SERVER_MODAL,
};
use dioxus::prelude::*;
use xmtp::Recipient;

use crate::{
    app_state::APP_STATE,
    states::{
        global_states::{CURRENT_SCREEN, Screen},
        user_states::USER,
    },
};

#[component]
pub fn OptionsMenuButton(
    mut open: Signal<bool>,
    button_type: OptionsMenuButtonType,
    icon: Asset,
    icon_size: i32,
) -> Element {
    let text;
    let mut icon_style = "";
    match button_type {
        OptionsMenuButtonType::Create => {
            text = "Create";
        }
        OptionsMenuButtonType::Join => {
            text = "Join";
            icon_style = "-m-2 rotate-180"
        }
        OptionsMenuButtonType::Settings => {
            text = "Settings";
        }
    }

    async fn on_click(button_type: OptionsMenuButtonType, mut open: Signal<bool>) {
        // let address = ADDRESS.read().clone().unwrap();
        // let address = USER
        //     .read()
        //     .profile
        //     .as_ref()
        //     .expect("No User Profile")
        //     .address
        //     .clone();
        // let app_state = APP_STATE.lock().await;

        match button_type {
            OptionsMenuButtonType::Create => {
                // open_create_modal.expect("no open modal signal").set(true);
                *OPEN_CREATE_SERVER_MODAL.write() = true;
                // let db_arc = app_state
                //     .db
                //     .as_ref()
                //     .expect("Database no initialized")
                //     .clone();
                // let db = db_arc.lock().await;
                // let metadata = db
                //     .create_server("Test Server 1", "Picture", &address)
                //     .await
                //     .map_err(|e| format!("could not create server: {e}"))
                //     .expect("");
                // if app_state.xmtp.is_some() {
                //     let members: Vec<Recipient> = vec![];
                //     let new_group_id = app_state
                //         .xmtp
                //         .as_ref()
                //         .expect("failed to get xmtp instance")
                //         .create_group(&members, Some("general".to_string()), None, None)
                //         .expect("failed to create new group");
                //     db.add_text_channel(&metadata.id, "general", &new_group_id)
                //         .await
                //         .expect("failed to add text channel to server");
                // }
            }
            OptionsMenuButtonType::Join => {}
            OptionsMenuButtonType::Settings => {
                *CURRENT_SCREEN.write() = Screen::Settings;
            }
        }
        open.set(false);
        println!("Clicked a menu button");
    }

    rsx! {
        button {
            class: "flex gap-2 p-2 place-items-center h-12 bg-deep-purple-300 rounded-lg hover:bg-deep-purple-400 select-none",
            onclick: move |_| on_click(button_type.clone(), open),
            img {
                class: "{icon_style}",
                src: icon,
                height: "{icon_size}",
                width: "{icon_size}",
            }
            div {
                class: "font-semibold",
                "{text}"
            }
        }
    }
}
