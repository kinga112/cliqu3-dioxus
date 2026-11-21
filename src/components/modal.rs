use dioxus::prelude::*;

#[component]
pub fn Modal(open: Signal<bool>, children: Element) -> Element {
    if !open() {
        return rsx! {};
    }

    rsx! {
        div {
            class: "relative z-50 select-none",
            // backdrop
            div {
                class: "fixed inset-0 bg-black/50",
                onclick: move |_| open.set(false),
            }
            // center wrapper
            div {
                class: "fixed inset-0 flex w-screen items-center justify-center",
                onclick: move |_| open.set(false),
                // modal
                div {
                    class: "flex flex-col max-w-lg items-center gap-5 bg-deep-purple-300 p-10 rounded-2xl overflow-hidden",
                    onclick: move |evt| evt.stop_propagation(),
                    {children}
                }
            }
        }
    }
}
