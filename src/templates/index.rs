use derive_more::Display;
use gloo_timers::future::TimeoutFuture;
use perseus::{spawn_local_scoped, Html, SsrNode, Template};
// use serde::{Deserialize, Serialize};
use sycamore::prelude::*;

use rand::Rng;
use std::error::Error as SysError;
// use std::fmt::{Display, Formatter};
// use sycamore::suspense::{use_transition, Suspense};

// use sycamore::prelude::{view, View};

#[derive(Debug, Display, Clone, Copy)]
enum Block {
    One,
    Two,
    Three,
}

// impl Display for Block {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Ok(self.fmt("test"))
//         Ok(println!("----"))
//     }
// }

// impl Display for View<G> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // Ok(self.fmt("test"))
//         view! {}
//     }
// }

// impl Display for Block {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         dbg!("---")
//     }
// }

// impl Block {
//     fn content<G: Html>(self, cx: Scope<'_>) -> Result<View<G>, Box<dyn SysError>> {
//         match self {
//             Block::One => Ok(view! {cx, h1{"content one"}}),
//             Block::Two => Ok(view! {cx, h1{"content Two"}}),
//             Block::Three => Ok(view! {cx, h1{"content Three"}}),
//         }
//     }
// }

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

// #[component]
// fn BlockView<G: Html, 'a>(
//     cx: Scope<'a>,
//     block: &'a Signal<Block>,
// ) -> Result<View<G>, Box<dyn SysError>> {
//     // let delay_ms = rand::thread_rng().gen_range(200..500);
//     // TimeoutFuture::new(delay_ms);

//     match *block.get() {
//         Block::One => Ok(view! {cx, h1{"content one"}}),
//         Block::Two => Ok(view! {cx, h1{"content Two"}}),
//         Block::Three => Ok(view! {cx, h1{"content Three"}}),
//     }
//     // let b = *block.get();

//     // view! { cx,
//     //     div {
//     //         // p { "Content loaded after " (delay_ms) "ms" }
//     //         // p { (if let Ok(content) = block.content::<G>(cx) { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
//     //         // p { (if let Ok(content) = block.get().as_ref() { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
//     //         // p {(block.get().as_ref()) }
//     //                     p {(b.content(cx)) }
//     //     }
//     // }
// }

#[component]
fn BlockView<G: Html, 'a>(cx: Scope<'a>, block: &'a Signal<Block>) -> View<G> {
    // let delay_ms = rand::thread_rng().gen_range(200..500);
    // TimeoutFuture::new(delay_ms);

    // let b = *block.get();
    let UpdateNode = move |x| {
        spawn_local_scoped(cx, async move { block.set(x) });
    };

    view! { cx,
            div {
                // p { "Content loaded after " (delay_ms) "ms" }
                // p { (if let Ok(content) = block.content::<G>(cx) { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
                // p { (if let Ok(content) = block.get().as_ref() { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }
                // p {(block.get().as_ref()) }
                            p {(

                                    match *block.get() {
            Block::One => view! {cx, h1{"content one"} button(on:click=move |_| UpdateNode(Block::One)) { "One" }  },
            Block::Two => view! {cx, h1{"content Two"}},
            Block::Three => view! {cx, h1{"content Three"}},
        }
    )

                            }
            }
        }
}

#[perseus::template_rx]
#[component(IndexPage<G>)]
pub fn index_page<G: Html>(cx: Scope) -> View<G> {
    let block = create_signal(cx, Block::One);
    // let transition = use_transition(cx);
    // let update = move |x| transition.start(move || block.set(x), || ());
    // let update = move |x| block.set(x);

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

    // let get_content = move |x: Block| {
    //     spawn_local_scoped(cx, async move {
    //         if let Ok(content) = x.content::<G>(cx) {
    //             // view! { cx, (content) }
    //             block.set(x)
    //         } else {
    //             view! { cx, p {"Oh no!"} }
    //         };
    //     });
    // };

    let UpdateNode = move |x| {
        spawn_local_scoped(cx, async move { block.set(x) });
    };

    view! { cx,
        div {

            // p { "Transition state: " (transition.is_pending().then_some("pending").unwrap_or("done")) }
            // button(on:click=move |_| update_node(Block::One)) { "One" }
            button(on:click=move |_| UpdateNode(Block::Two)) { "Two" }
            button(on:click=move |_| UpdateNode(Block::Three)) { "Three" }
            // p { (if let Ok(content) = block.get() { view! { cx, (content) } } else { view! { cx, "Oh no!" } }) }


            // div{(&get_content(*block.get())) }
            BlockView(block)



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
