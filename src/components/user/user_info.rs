use dioxus::prelude::*;

use crate::states::user_states::MemberProfile;

#[component]
pub fn UserInfo(profile: MemberProfile, open: Option<Signal<bool>>) -> Element {
    let mut menu_rect = use_signal(|| None::<(f64, f64, f64, f64)>);

    if open.is_some() {
        use_effect(move || {
            if open.unwrap()() {
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
                                        open.unwrap().set(false);
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

        if !open.unwrap()() {
            return rsx! {};
        }
    }

    let avatar = "https://png.pngtree.com/thumb_back/fh260/background/20230727/pngtree-aesthetic-liquid-purple-background-image_12761619.jpg";

    rsx! {
        div {
            class: "relative h-36 w-96 shadow-lg bg-off-black-400 rounded-xl z-50 overflow-hidden",
            img {
                class: "absolute w-full h-full blur-md select-none -z-10",
                // src: profile.avatar.clone(),
                src: avatar,
            }
            div {
                class: "w-full h-full rounded-xl overflow-hidden pointer-events-none z-50",
                onmounted: move |data| {
                    spawn(async move {
                        if let Ok(rect) = data.get_client_rect().await {
                            menu_rect.set(Some((rect.origin.x, rect.origin.y, rect.size.width, rect.size.height)));
                        }
                    });
                },
                div {
                    class: "flex p-4 gap-4 place-items-center justify-evenly h-full text-left",
                    img {
                        class: "w-28 h-28 rounded-lg shrink-0 object-cover select-none",
                        // src: profile.avatar,
                        src: avatar,
                    }
                    div {
                        class: "h-28 p-2 w-56 rounded-lg text-deep-purple-500 bg-white/20 shadow-lg ring-1 ring-black/5",
                        div {
                            class: "flex flex-col gap-0.5 pointer-events-auto",
                            p {
                                class: "text-xl font-semibold line-clamp-1 text-ellipsis",
                                "{profile.name}"
                            }
                            p {
                                class: "text-base font-light line-clamp-2 text-ellipsis",
                                "{profile.description}"
                            }
                            button {
                                class: "text-xs hover:underline",
                                p {
                                    class: "truncate",
                                    "{profile.address}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
