use crate::components::sidenav::options_menu::{
    CreateServerModal, JoinServerModal, OptionsMenuButton,
};
use dioxus::prelude::*;

const ADD: Asset = asset!("/assets/icons/add-cropped.svg");
const SETTINGS: Asset = asset!("/assets/icons/settings.svg");
const ARROW: Asset = asset!("/assets/icons/arrow.svg");

#[derive(PartialEq, Clone)]
pub enum OptionsMenuButtonType {
    Create,
    Join,
    Settings,
}

#[component]
pub fn OptionsMenu(open: Signal<bool>) -> Element {
    let open_create_server_modal = use_signal(|| false);
    let open_join_server_modal = use_signal(|| false);

    // Claude Assist here: Check this for better options? Working for now
    // Understand document eval and the function how it works in dioxus
    let mut menu_rect = use_signal(|| None::<(f64, f64, f64, f64)>); // x, y, width, height
    use_effect(move || {
        if open() {
            spawn(async move {
                let mut eval = document::eval(
                    r#"
                    function handler(e) {
                        dioxus.send([e.clientX, e.clientY]);
                    }
                    window.addEventListener('mousedown', handler, true);
                    "#,
                );
                loop {
                    match eval.recv::<(f64, f64)>().await {
                        Ok((x, y)) => {
                            if let Some((mx, my, mw, mh)) = menu_rect() {
                                let inside = x >= mx && x <= mx + mw && y >= my && y <= my + mh;
                                if !inside {
                                    open.set(false);
                                    break;
                                }
                            } else {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            });
        }
    });

    rsx! {
        if open() {
            div {
                class: "absolute flex flex-col p-2 gap-1 left-14 w-32 bg-off-black-400 border border-off-black-300 rounded-xl z-20",
                onmounted: move |data| {
                    spawn(async move {
                        if let Ok(rect) = data.get_client_rect().await {
                            menu_rect.set(Some((rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)));
                        }
                    });
                },
                OptionsMenuButton { open: open, button_type: OptionsMenuButtonType::Create, icon: ADD, icon_size: 25, open_modal_opt: Some(open_create_server_modal) }
                OptionsMenuButton { open: open, button_type: OptionsMenuButtonType::Join, icon: ARROW, icon_size: 40, open_modal_opt: Some(open_join_server_modal) }
                OptionsMenuButton { open: open, button_type: OptionsMenuButtonType::Settings, icon: SETTINGS, icon_size: 25 }
            }
        }
        CreateServerModal {
            open_modal: open_create_server_modal,
        }
        JoinServerModal {
            open_modal: open_join_server_modal,
        }
    }
}
