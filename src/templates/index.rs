use derive_more::Display;
use gloo_timers::future::TimeoutFuture;
use perseus::{spawn_local_scoped, Html, SsrNode, Template};
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

// use gloo_timers::future::TimeoutFuture;
// use rand::Rng;
use std::error::Error as SysError;
use std::fmt::{Display, Formatter};
// use sycamore::suspense::{use_transition, Suspense};

// use sycamore::prelude::{view, View};

#[derive(Debug, Clone, Display, Copy)]
enum Block {
    One,
    Two,
    Three,
}

// impl Display for Block {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         Ok(view! { p { "display"}})
//     }
// }

// impl Display for Block {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         dbg!("---")
//     }
// }

impl Block {
    fn content<G: Html>(self, cx: Scope<'_>) -> Result<View<G>, Box<dyn SysError>> {
        match self {
            Block::One => Ok(view! {cx, div{"content one"}}),
            Block::Two => Ok(view! {cx, div{"content Two"}}),
            Block::Three => Ok(view! {cx, div{"content Three"}}),
        }
    }
}

// #[component]
// fn Child<G: Html>(cx: Scope<'_>, block: Block) -> View<G> {
//     let delay_ms = rand::thread_rng().gen_range(200..500);
//     // TimeoutFuture::new(delay_ms).await;

//     let content = if let Ok(content) = block.content::<G>(cx) {
//         view! { cx, (content) }
//     } else {
//         view! { cx, "Oh no!" }
//     };

//     // spawn_local_scoped(cx, async move {
//     //     TimeoutFuture::new(delay_ms).await;
//     //     let content = if let Ok(content) = block.content::<G>(cx) {
//     //         view! { cx, (content) }
//     //     } else {
//     //         view! { cx, "Oh no!" }
//     //     };
//     // });

// spawn_local_scoped(cx, async move {
//     TimeoutFuture::new(delay_ms).await;
//     view! { cx,
//         div {
//             p { (if let Ok(content) = block.content(cx) { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
//         }
//     }
// });
// }

#[perseus::template_rx]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    let block = create_signal(cx, Block::One);
    // let transition = use_transition(cx);
    // let update = move |x| transition.start(move || block.set(x), || ());
    let update = move |x| block.set(x);

    // let content = if let Ok(content) = block.content(cx) {
    //     view! { cx, (content) }
    // } else {
    //     view! { cx, "Oh no!" }
    // };

    // let new_block = spawn_local_scoped(cx, async move {
    //     TimeoutFuture::new(delay_ms).await;
    //     view! { cx,
    //         div {
    //             p { (if let Ok(content) = block.get(cx) { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
    //         }
    //     }
    // });

    // let get_content = move |x: Block| {
    //     spawn_local_scoped(cx, async move {
    //         match x.content().await {
    //             Ok(cnt) => {}
    //             Err(e) => log::info!("Could not get wallet info: {}", e),
    //         }
    //     });
    // };

    let get_content = move |x: Block| {
        spawn_local_scoped(cx, async move {
            if let Ok(content) = x.content::<G>(cx) {
                view! { cx, (content) }
            } else {
                view! { cx, p {"Oh no!"} }
            };
        });
    };

    view! { cx,
        div {

            // p { "Transition state: " (transition.is_pending().then_some("pending").unwrap_or("done")) }
            button(on:click=move |_| update(Block::One)) { "One" }
            button(on:click=move |_| update(Block::Two)) { "Two" }
            button(on:click=move |_| update(Block::Three)) { "Three" }
            // p { (if let Ok(content) = block.get() { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }


            // div{(&get_content(*block.get())) }
            div{(block.get())}



            // Suspense(fallback=view! { cx, p { "Loading..." } }) {
            //     ({
            //         let block = *block.get();
            //         view! {
            //             cx, Child(block)
            //                                 p { (if let Ok(content) = block.content(cx) { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
            //         }
            //     })
            // }
        }
    }
}

#[perseus::head]
pub fn head(cx: Scope) -> View<SsrNode> {
    view! { cx,
        title { "Welcome to Perseus!" }
    }
}

pub fn get_template<G: Html>() -> Template<G> {
    Template::new("index").template(index_page).head(head)
}
