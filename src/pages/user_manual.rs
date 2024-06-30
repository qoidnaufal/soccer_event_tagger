use crate::components::HomeButton;
use leptos::*;

const USER_MANUAL: &str = include_str!("../../USER_MANUAL.md");

#[component]
pub fn UserManual() -> impl IntoView {
    let user_manual = USER_MANUAL.lines().collect::<Vec<_>>();

    view! {
        <div
            class="absolute m-auto right-0 left-0 top-0 bottom-0 size-full flex flex-row"
        >
            <HomeButton/>
            <div class="mx-auto p-4 overflow-y-scroll">
                <article class="max-w-[1020px]">
                    {
                        move || {
                            user_manual
                                .iter()
                                .map(|lines| {
                                    match lines {
                                        header if header.starts_with("## ") => {
                                            let header = header.strip_prefix("## ").unwrap_or("");
                                            view! { <h1><b>{ move || header }</b></h1> }.into_view()
                                        },
                                        subheader if subheader.starts_with("### ") => {
                                            let subheader = subheader.strip_prefix("### ").unwrap_or("");
                                            view! { <h2><b>{ move || subheader }</b></h2> }.into_view()
                                        },
                                        img if img.starts_with("![") => {
                                            let img = img.strip_prefix('!').unwrap_or("");
                                            let mut img_token = img.split(']');

                                            let alt_text = img_token.next().unwrap_or("");
                                            let alt_text = alt_text.strip_prefix('[').unwrap_or("");

                                            let img_src = img_token.next().unwrap_or("");
                                            let img_src = img_src.strip_prefix('(').unwrap().strip_suffix(')').unwrap();

                                            view! {
                                                <img
                                                    alt=move || alt_text
                                                    src=move || img_src
                                                    width="1000"
                                                />
                                            }.into_view()
                                        },
                                        &"" => {
                                            view! { <br/> }.into_view()
                                        },
                                        text if text.contains('<') => {
                                            let mut token = text.split(&['<', '>']);

                                            let txt1 = token.next().unwrap_or("");
                                            let img_token = token.next().unwrap_or("");
                                            let txt2 = token.next().unwrap_or("");

                                            let mut img_token = img_token.strip_prefix("img").unwrap_or("").split_whitespace();

                                            let img_src = img_token.next().unwrap_or("");
                                            let img_src = img_src.strip_prefix("src=\"").unwrap_or("").strip_suffix('"').unwrap_or("");

                                            let alt_text = img_token.next().unwrap_or("");
                                            let alt_text = alt_text.strip_prefix("alt=\"").unwrap_or("").strip_suffix('"').unwrap_or("");

                                            let img_width = img_token.next().unwrap_or("");
                                            let img_width = img_width.strip_prefix("width=\"").unwrap_or("").strip_suffix('"').unwrap_or("");

                                            let img_height = img_token.next().unwrap_or("");
                                            let img_height = img_height.strip_prefix("height=\"").unwrap_or("").strip_suffix('"').unwrap_or("");

                                            view! {
                                                <p class="text-wrap">
                                                    <span>{ move || txt1 }</span>
                                                    <span>" [ "</span>
                                                    <span>
                                                        <img
                                                            src=move || img_src
                                                            alt=move || alt_text
                                                            width=move || img_width
                                                            height=move || img_height
                                                        />
                                                    </span>
                                                    <span>" ] "</span>
                                                    <span>{ move || txt2 }</span>
                                                </p>
                                            }.into_view()
                                        },
                                        normal => {
                                            let normal = *normal;
                                            view! { <p class="text-wrap">{ move || normal }</p> }.into_view()
                                        }
                                    }
                                }
                            ).collect_view()
                        }
                    }
                </article>
            </div>
        </div>
    }
}
