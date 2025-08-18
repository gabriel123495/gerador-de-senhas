use dioxus::prelude::*;
use rand::Rng;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
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

            h1 {
                style: "font-size: 48px; color: #000000ff;",
                "🔐 Gerador de Senhas Fortes"
            },

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
                    const CHARSET_1: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%&*()_+-=[]{}|;:,.<>?/";
                    const CHARSET_2: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";
                    const CHARSET_3: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
                    const CHARSET_4: &[u8] = b"abcdefghijklmnopqrstuvwxyz";

                    let uppercase = true;
                    let special = true;
                    let numbers = true;

                    let charset = if uppercase && special && numbers {
                        CHARSET_1
                    } else if uppercase && numbers {
                        CHARSET_2
                    } else if uppercase {
                        CHARSET_3
                    } else {
                        CHARSET_4
                    };
                    
                    let mut rng = rand::thread_rng();
                    let password: String = (0..30)
                        .map(|_| {
                            let idx = rng.gen_range(0..charset.len());
                            CHARSET_1[idx] as char
                        })
                        .collect();
                    senha.set(password);
                },
                "Gerar Nova Senha"
            },

            h2 {
                style: "font-size: 24px; color: #000000ff;",
                "Senha Gerada:"
            },

            h2 { "{senha.read()}" }
        }
    }
}
