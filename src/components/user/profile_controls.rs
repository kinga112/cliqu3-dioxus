use dioxus::prelude::*;

use crate::{components::user::UserInfo, state::app_state::AppState};

const MIC: Asset = asset!("/assets/icons/mic.svg");
const MUTED_MIC: Asset = asset!("/assets/icons/muted-mic.svg");
const HEADPHONES: Asset = asset!("/assets/icons/headphones.svg");
const MUTED_HEADPHONES: Asset = asset!("/assets/icons/muted-headphones.svg");

#[component]
pub fn ProfileControls(open: Signal<bool>) -> Element {
    let mut app_state = use_context::<AppState>();
    let profile = app_state.user.read().clone().profile;
    // let mut user = app_state.user.write();
    // let mut audio = use_signal(|| true);
    // let mut silence = use_signal(|| false);

    let mut menu_rect = use_signal(|| None::<(f64, f64, f64, f64)>);
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

    if !open() {
        return rsx! {};
    }

    // let profile = app_state.user.read().clone().profile;
    rsx! {
        div {
            class: "absolute w-[450px] left-[75px] bottom-0 z-50",
        div {
            class: "flex p-2 gap-1 bg-off-black-400 border border-off-black-300 rounded-2xl w-fit",
            onmounted: move |data| {
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        menu_rect.set(Some((rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)));
                    }
                });
            },
            UserInfo{profile, open: Signal::new(true), can_close: false}
            div{
                class: "flex flex-col justify-evenly p-1 bg-off-black-300 rounded-xl",
                button {
                    onclick: move |_| app_state.user.write().audio ^= true,
                    class: "p-1 hover:bg-off-black-200 rounded-md",
                    if app_state.user.read().audio {
                        img{
                            src: MIC,
                            height: 25,
                            width: 25,
                        }
                    }else{
                        img{
                            src: MUTED_MIC,
                            height: 25,
                            width: 25,
                        }
                    }
                }
                button {
                    class: "p-1 hover:bg-off-black-200 rounded-md",
                    onclick: move |_| app_state.user.write().silence ^= true,
                    if app_state.user.read().silence {
                        img{
                            src: MUTED_HEADPHONES,
                            height: 25,
                            width: 25,
                        }
                    }else{
                        img{
                            src: HEADPHONES,
                            height: 25,
                            width: 25,
                        }
                    }
                }
            }
        }
        }
    }
}
