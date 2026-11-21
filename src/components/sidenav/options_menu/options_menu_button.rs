use crate::{
    components::sidenav::options_menu::{CreateServerModal, options_menu::OptionsMenuButtonType},
    state::app_state::{AppState, Screen},
};
use dioxus::prelude::*;
use xmtp::Recipient;

#[component]
pub fn OptionsMenuButton(
    mut open: Signal<bool>,
    button_type: OptionsMenuButtonType,
    icon: Asset,
    icon_size: i32,
    mut open_modal_opt: Option<Signal<bool>>,
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

    async fn on_click(
        button_type: OptionsMenuButtonType,
        mut open: Signal<bool>,
        open_modal_opt: Option<Signal<bool>>,
    ) {
        let mut app_state = use_context::<AppState>();

        match button_type {
            OptionsMenuButtonType::Create => {
                if let Some(mut open_modal) = open_modal_opt {
                    *open_modal.write() = true;
                }
            }
            OptionsMenuButtonType::Join => {
                if let Some(mut open_modal) = open_modal_opt {
                    *open_modal.write() = true;
                }
            }
            OptionsMenuButtonType::Settings => {
                *app_state.current_screen.write() = Screen::Settings;
            }
        }
        open.set(false);
        println!("Clicked a menu button");
    }

    rsx! {
        button {
            class: "flex gap-2 p-2 place-items-center h-12 bg-deep-purple-300 rounded-lg hover:bg-deep-purple-400 select-none",
            onclick: move |_| on_click(button_type.clone(), open, open_modal_opt),
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
