use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::server::channel::emojis::{EmojiElement, emoji_enum::Emoji};

#[component]
// pub fn EmojiModal(open: Signal<bool>, x_pos: String, y_pos: String) -> Element {
// pub fn EmojiModal(on_close: EventHandler<()>) -> Element {
pub fn EmojiModal(open: Signal<bool>, display_currently_selected_message: Signal<bool>) -> Element {
    // trying memoization here
    let emoji_list = use_memo(|| {
        Emoji::iter()
            .map(|emoji| {
                rsx! { EmojiElement { emoji: emoji } }
            })
            .collect::<Vec<_>>()
    });

    let read_list = emoji_list.read();

    let mut menu_rect = use_signal(|| None::<(f64, f64, f64, f64)>);

    if *open.read() {
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
                                display_currently_selected_message.set(false);
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
    } else {
        return rsx! {};
    }

    rsx! {
        // div {
        //     class: "fixed inset-0 z-40",
        //     // onclick: move |_| on_close.call(()),
        //     onmounted: move |data| {
        //         spawn(async move {
        //             if let Ok(rect) = data.get_client_rect().await {
        //                 menu_rect.set(Some((rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)));
        //             }
        //         });
        //     },
        // }
        div{
            class: "z-50 absolute right-0 bottom-10 bg-off-black-100 rounded-lg w-fit min-w-max border-2 border-deep-purple-100",
            onmounted: move |data| {
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        menu_rect.set(Some((rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)));
                    }
                });
            },
            div{
                class: "inline-grid gap-1 grid-cols-8 place-items-center text-3xl p-0.5 h-96 max-w-full overflow-y-auto overflow-x-hidden no-scrollbar",
                {read_list.iter()}
            }
        }
    }
}
