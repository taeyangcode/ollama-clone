use yew::prelude::*;
use stylist::{css, StyleSource};

#[function_component]
fn NavBar() -> Html {
    let header_style: StyleSource = css!(
        r#"
        width: 100%;
        height: 64px;

        display: grid;
        place-items: center;
        "#
    );

    let navbar_style: StyleSource = css!(
        r#"
        width: 98%;
        height: 100%;

        display: flex;
        justify-content: space-between;
        align-items: center;
        "#
    );

    let ollama_icon_container: StyleSource = css!(
        r#"
        width: 64px;
        aspect-ratio: 1 / 1;

        display: grid;
        place-items: center;
        "#
    );

    let ollama_icon_style: StyleSource = css!(
        r#"
        width: auto;
        height: 40px;
        "#
    );

    let navbar_router_container: StyleSource = css!(
        r#"
        width: auto;
        height: 100%;

        display: grid;
        grid-auto-flow: column;
        grid-template-columns: repeat(5, minmax(90px, auto));
        grid-template-rows: minmax(0, 1fr);
        column-gap: 2px;
        justify-content: end;
        "#
    );

    let navbar_route_container_style: StyleSource = css!(
        r#"
        width: 100%;
        height: 100%;

        display: grid;
        place-items: center;
        "#
    );

    let navbar_router_link_style: StyleSource = css!(
        r#"
        font-size: 18px;
        color: inherit;
        text-decoration: none;
        "#
    );

    struct NavBarRoute {
        route: &'static str,
        name: &'static str,
    }

    const NAVBAR_ROUTES: [NavBarRoute; 5] = [
        NavBarRoute { route: "/library", name: "Models" },
        NavBarRoute { route: "/blog", name: "Blog" },
        NavBarRoute { route: "https://discord.com/invite/ollama", name: "Discord" },
        NavBarRoute { route: "https://github.com/morganca/ollama", name: "GitHub" },
        NavBarRoute { route: "/download", name: "Download" },
    ];

    html! {
        <header class={header_style}>
            <nav class={navbar_style}>
                <div class={ollama_icon_container}>
                    <img class={ollama_icon_style} src="images/ollama.png" />
                </div>

                <div class={navbar_router_container}>
                    {
                        NAVBAR_ROUTES
                            .into_iter()
                            .map(|NavBarRoute { route, name }| {
                                 html! {
                                     <div class={navbar_route_container_style.clone()}>
                                         <a key={name} class={navbar_router_link_style.clone()} href={route}>{ name }</a>
                                     </div>
                                 }
                            } ).collect::<Html>()
                    }
                </div>
            </nav>
        </header>
    }
}

#[function_component]
fn Main() -> Html {
    let main_container_style: StyleSource = css!(
        r#"
        width: 100vw;
        height: 50vh;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        row-gap: 30px;
        "#
    );

    let ollama_icon_style: StyleSource = css!(
        r#"
        width: 64px;
        height: auto;
        "#
    );

    let ollama_introduction_container: StyleSource = css!(
        r#"
        width: 40%;
        height: auto;

        display: grid;
        justify-items: center;
        row-gap: 30px;
        "#
    );

    let title_text_container: StyleSource = css!(
        r#"
        width: 100%;
        height: auto;

        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;

        row-gap: 30px;
        "#
    );

    let title_text_style: StyleSource = css!(
        r#"
        font-size: 28px;
        font-weight: bold;

        text-align: center;
        "#
    );

    let TITLE_TEXT: Html = html! { <> <div>{ "Get up and running with large" }</div><div>{ "language models, locally." }</div> </> };

    let subtitle_text_style: StyleSource = css!(
        r#"
        font-size: 18px;
        color: #737373;

        text-align: center;
        line-height: 28px;
        "#
    );

    let SUBTITLE_TEXT: Html = html!{ <> <div>{ "Run Llama 2, Code Llama, and other models. " }</div> <div>{ "Customize and create your own." }</div> </> };

    let button_container: StyleSource = css!(
        r#"
        width: 175px;
        height: auto;

        display: flex;
        justify-content: center;
        align-items: center;
        flex-direction: column;

        row-gap: 20px;
        "#
    );

    let download_button_style: StyleSource = css!(
        r#"
        width: 100%;
        height: 45px;

        font-size: 20px;

        text-decoration: none;
        outline: none;
        border: none;

        background: #262626;
        color: white;
        border-radius: 8px;
        "#
    );

    const DOWNLOAD_BUTTON_TEXT: &'static str = "Download ↓";

    let button_note_style: StyleSource = css!(
        r#"
        font-size: 12px;

        color: #737373;
        text-align: center;
        "#
    );

    const BUTTON_NOTE: &'static str = "Available for macOS & Linux Windows coming soon";

    html! {
        <main class={main_container_style}>
            <img class={ollama_icon_style} src="images/ollama.png" />

            <div class={ollama_introduction_container}>
                <div class={title_text_container}>
                    <span class={title_text_style}>{ TITLE_TEXT }</span>
                    <span class={subtitle_text_style}>{ SUBTITLE_TEXT }</span>
                </div>

                <div class={button_container}>
                    <button class={download_button_style}>{ DOWNLOAD_BUTTON_TEXT }</button>
                    <span class={button_note_style}>{ BUTTON_NOTE }</span>
                </div>
            </div>
        </main>
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <>
            <NavBar />
            <Main />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
