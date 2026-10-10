use dioxus::prelude::*;

use crate::{components::user::UserInfo, smart_contract, states::user_states::MemberProfile};

#[component]
pub fn UpdateProfile() -> Element {
    let gas_estimate = use_signal(|| String::new());
    let mut username = use_signal(|| String::new());
    let mut bio = use_signal(|| String::new());
    let mut avatar = use_signal(|| String::new());

    let tmp_profile = MemberProfile {
        address: String::new(),
        name: username.read().clone(),
        description: bio.read().clone(),
        avatar: avatar.read().clone(),
    };

    rsx! {
        div {
            class: "flex flex-col gap-1 bg-off-black-500 h-full w-full p-5",
            div {
                class: "flex text-3xl py-5",
                "Update Profile"
            }
            input {
                class: "h-12 p-2 rounded-lg focus:outline-none min-w-0 border-b-1 border-deep-purple-300",
                placeholder: "username",
                oninput: move |evt| username.set(evt.value()),
            }
            input {
                class: "h-12 p-2 rounded-lg focus:outline-none min-w-0 border-b-1 border-deep-purple-300",
                placeholder: "bio",
                oninput: move |evt| bio.set(evt.value()),
            }
            input {
                class: "h-12 p-2 rounded-lg focus:outline-none min-w-0 border-b-1 border-deep-purple-300",
                placeholder: "avatar",
                oninput: move |evt| avatar.set(evt.value()),
            }
            button {
                class: "w-fit p-2 h-12 bg-deep-purple-300 rounded",
                onclick: move |_| get_gas_estimate(username.read().clone(), bio.read().clone(), avatar.read().clone(), gas_estimate),
                "Get Gas Estimate"
            }
            div {
                class: "h-12",
                if gas_estimate.read().clone() != String::new() {
                    "Gas Estimate: ${&gas_estimate.read()[0..8]} USD"
                }
            }
            UserInfo{profile: tmp_profile}
            button {
                class: "w-fit p-2 h-12 bg-deep-purple-300 rounded",
                onclick: move |_| update_profile(username.read().clone(), bio.read().clone(), avatar.read().clone()),
                "Update Profile"
            }
        }
    }
}

async fn update_profile(username: String, bio: String, avatar: String) {
    let result = smart_contract::interact::create_profile(username, bio, avatar).await;

    match result {
        Ok(()) => {
            println!("SUCCESSFULLY UPDATED PROFILE");
        }
        Err(e) => {
            eprint!("FAILED TO UPDATE PROFILE WITH ERROR: {:?}", e);
        }
    }
}

async fn get_gas_estimate(
    username: String,
    bio: String,
    avatar: String,
    mut gas_estimate: Signal<String>,
) {
    let result = smart_contract::interact::create_profile_gas_price(username, bio, avatar).await;

    match result {
        Ok(estimate) => {
            println!("GOT GAS ESTIMATE");
            gas_estimate.set(estimate);
        }
        Err(e) => {
            eprint!("GET GAS ESTIMATE FAILED WITH ERROR: {:?}", e);
        }
    }
}
