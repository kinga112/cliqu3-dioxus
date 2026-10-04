use dioxus::prelude::*;
use strum::IntoEnumIterator;

use crate::components::server::channel::emojis::{EmojiElement, emoji_enum::Emoji};

#[component]
pub fn EmojiModal(open: Signal<bool>) -> Element {
    // let emoji_list = Emoji::iter().map(|emoji| {
    //     rsx! { EmojiElement{emoji: emoji}}
    // });

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
    use_effect(move || {
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
                        if open() {
                            if let Some((mx, my, mw, mh)) = menu_rect() {
                                let inside = x >= mx && x <= mx + mw && y >= my && y <= my + mh;
                                if !inside {
                                    open.set(false);
                                }
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    });

    if !open() {
        return rsx! {};
    }

    rsx! {
        div{
            class: if open() { "z-50 absolute right-0 bottom-10 bg-off-black-100 rounded-lg w-fit min-w-max border-2 border-deep-purple-100"} else {"invisible"},
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
