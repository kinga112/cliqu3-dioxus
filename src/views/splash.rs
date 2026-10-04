use dioxus::prelude::*;

#[component]
pub fn Splash() -> Element {
    rsx! {
        div {
            class: "flex justify-center place-items-center w-screen h-screen bg-off-black-500 text-deep-purple-100 text-8xl",
            "C L I Q U 3"
        }
    }
}
