use dioxus::prelude::*;
use fastrand;

#[allow(non_snake_case)]
fn main() {
    launch(app);
}

#[component]
fn app() -> Element {
    let mut senha = use_signal(|| String::new());

    rsx! {
        div {
            style: "
                display: flex;
                flex-direction: column;
                justify-content: center;
                align-items: center;
                height: 100vh;
                gap: 20px;
                font-family: monospace;
            ",
            button {
                style: "
                    background-color: #0984ffff;
                    color: white;
                    border: none;
                    padding: 10px 20px;
                    font-size: 16px;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: background-color 0.3s ease;
                ",
                onclick: move |_| {
                    let nova_senha = (0..12)
                        .map(|_| (fastrand::u8(33..127)) as char)
                        .collect::<String>();
                    senha.write().replace_range(.., &nova_senha);
                },
                "🔐 Gerar Senha"
            },

            h1 {
                style: "
                    font-size: 24px;
                    color: #000000ff;
                ",
                "Senha Gerada:"
            },

            h2 { "{senha.read()}" }
        }
    }
}