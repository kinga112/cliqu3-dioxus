// use crate::components::sidenav::options_menu_button::OptionsMenuButton;
// use crate::components::sidenav::profile_controls::ProfileControls;
use crate::{
    components::{
        sidenav::{ServerButton, options_menu::OptionsMenu},
        user::ProfileControls,
    },
    modules::docs::db::ServerMetadata,
    states::{
        global_states::{CURRENT_SCREEN, Screen},
        server_states::SERVER_LIST,
        user_states::USER,
    },
};
use dioxus::prelude::*;

const MESSAGE_BUBBLE: Asset = asset!("/assets/icons/message_bubble.svg");
const OPTIONS: Asset = asset!("/assets/icons/options.svg");

#[component]
pub fn SideNav() -> Element {
    let mut open_options_menu = use_signal(|| false);
    let mut open_profile_controls = use_signal(|| false);

    let servers = SERVER_LIST
        .read()
        .clone()
        .into_iter()
        .rev()
        .map(|metadata| {
            rsx! {ServerButton { metadata: metadata }}
        });

    let profile = USER.read().profile.clone().expect("failed to get profile");
    // let profile_option = USER.read().profile.clone();
    // match profile_option {
    //     Some(profile) => {
    //         println!("GOT MEMBER PROFILE IN SIDENAV: {:?}", profile);
    //     }
    //     None => {
    //         println!("FAILED TO GET MEMBER PROFILE IN SIDENAV");
    //     }
    // }

    let avatar = "https://png.pngtree.com/thumb_back/fh260/background/20230727/pngtree-aesthetic-liquid-purple-background-image_12761619.jpg";

    rsx! {
        div {
            class: "flex relative",
            div {
                id: "no-scrollbar",
                class: "w-20 overflow-y-scroll shrink-0 pb-20 pt-[132px] overflow-hidden",

                div {
                    class: "p-1 absolute top-2 left-3",
                    div {
                        class: "flex flex-col gap-1.5",
                        button {
                            class: "flex flex-col w-12 h-12 p-2.5 bg-deep-purple-300 rounded-xl justify-center place-items-center duration-200 hover:scale-105 z-10 select-none",
                            onclick: move |_| on_dm_click(),
                            img {
                                src: MESSAGE_BUBBLE,
                                height: "35",
                                width: "35",
                            }
                        }
                        div{
                            class: "flex flex-col relative",
                            button {
                                class: "w-12 h-12 bg-deep-purple-300 rounded-xl justify-center place-items-center duration-200 hover:scale-105 z-10 overflow-visible select-none",
                                onclick: move |_| open_options_menu.set(true),
                                img {
                                    src: OPTIONS,
                                    height: "75",
                                    width: "75",
                                }
                            }
                            OptionsMenu{open: open_options_menu}
                        }
                    }
                }
                {servers}
                div {
                    class: "p-1 absolute bottom-2 left-3",
                    div {
                        class: "flex flex-col relative",
                        button {
                            class: "flex flex-col w-12 h-12 bg-deep-purple-300 rounded-xl justify-center place-items-center duration-200 hover:scale-105 z-10",
                            onclick: move |_| open_profile_controls.toggle(),
                            img {
                                class: "object-cover w-12 h-12 rounded-xl",
                                // src: avatar,
                                src: if profile.avatar.clone() != "" {
                                    "{profile.avatar}"
                                }else{
                                    "{avatar}"
                                }
                            }
                        }
                        // ProfileControls{open: open_profile_controls}
                    }
                    ProfileControls{open: open_profile_controls}
                }
            }

            // FIX BELOW SO NOT SUPER SPECIFIC WITH THE DIMENSIONS BASED ON OTHER SCREENS
            div {
                class: "overflow-hidden absolute top-0 w-20 h-52 select-none pointer-events-none",
                div {
                    class: "absolute top-0 w-20 bg-off-black-700 border-b-2 border-off-black-400 h-[126px] shadow-off-black-700 shadow-lg",
                }
            }
            div {
                class: "overflow-hidden absolute bottom-0 w-20 h-52 select-none pointer-events-none",
                div {
                    class: "absolute bottom-0 w-20 bg-off-black-700 border-t-2 border-off-black-400 h-[72px] shadow-[0px_-10px_15px_-3px_rgba(0,0,0,0.1)] shadow-off-black-700",
                }
            }
        }
    }
}

fn on_dm_click() {
    let profile = USER.read().profile.clone().expect("failed to get profile");
    println!("Clicked DM: {:?}", profile.address);
    *CURRENT_SCREEN.write() = Screen::DirectMessages;
}
