use wasm_bindgen::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use serde_json::json;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Idioma {
    Portugues,
    Ingles,
}

impl Idioma {
    pub fn traduzir(&self, texto: &str) -> String {
        match self {
            Idioma::Portugues => match texto {
                "senha_gerada" => "Senha Gerada".to_string(),
                "copiada" => "Copiada!".to_string(),
                "tamanho" => "Comprimento da senha".to_string(),
                "maiusculas" => "Letras Maiúsculas".to_string(),
                "numeros" => "Números".to_string(),
                "simbolos" => "Especiais".to_string(),
                "gerar" => "Gerar Nova Senha".to_string(),
                "baixar" => "Baixar 10 Senhas".to_string(),
                _ => texto.to_string(),
            },
            Idioma::Ingles => match texto {
                "senha_gerada" => "Generated Password".to_string(),
                "copiada" => "Copied!".to_string(),
                "tamanho" => "Password length".to_string(),
                "maiusculas" => "Uppercase Letters".to_string(),
                "numeros" => "Numbers".to_string(),
                "simbolos" => "Special Characters".to_string(),
                "gerar" => "Generate New Password".to_string(),
                "baixar" => "Download 10 Passwords".to_string(),
                _ => texto.to_string(),
            },
        }
    }
}

#[wasm_bindgen]
pub struct Senhas {
    contem_numeros: bool,
    contem_simbolos: bool,
    contem_maiusculas: bool,
    idioma: Idioma,
}

#[wasm_bindgen]
impl Senhas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Senhas {
        Senhas {
            contem_numeros: true,
            contem_simbolos: true,
            contem_maiusculas: true,
            idioma: Idioma::Portugues,
        }
    }

    #[wasm_bindgen(js_name = "setIdioma")]
    pub fn set_idioma(&mut self, idioma: &str) {
        self.idioma = match idioma {
            "en" => Idioma::Ingles,
            _ => Idioma::Portugues,
        };
    }

    #[wasm_bindgen(js_name = "setNumeros")]
    pub fn set_numeros(&mut self, v: bool) {
        self.contem_numeros = v;
    }

    #[wasm_bindgen(js_name = "setSimbolos")]
    pub fn set_simbolos(&mut self, v: bool) {
        self.contem_simbolos = v;
    }

    #[wasm_bindgen(js_name = "setMaiusculas")]
    pub fn set_maiusculas(&mut self, v: bool) {
        self.contem_maiusculas = v;
    }

    #[wasm_bindgen]
    pub fn gerar(&self, tamanho: usize) -> String {
        let mut caracteres = String::from("abcdefghijklmnopqrstuvwxyz");

        if self.contem_maiusculas {
            caracteres += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        }
        if self.contem_numeros {
            caracteres += "0123456789";
        }
        if self.contem_simbolos {
            caracteres += "!@#$%^&*()_+-=[]{}|;:,.<>?";
        }

        let mut rng = SmallRng::from_entropy();
        let bytes = caracteres.as_bytes();

        (0..tamanho)
            .map(|_| {
                let idx = rng.gen_range(0..bytes.len());
                bytes[idx] as char
            })
            .collect()
    }

    #[wasm_bindgen(js_name = "gerarMultiplas")]
    pub fn gerar_multiplas(&self, tamanho: usize, quantidade: usize) -> String {
        let mut caracteres = String::from("abcdefghijklmnopqrstuvwxyz");

        if self.contem_maiusculas {
            caracteres += "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        }
        if self.contem_numeros {
            caracteres += "0123456789";
        }
        if self.contem_simbolos {
            caracteres += "!@#$%^&*()_+-=[]{}|;:,.<>?";
        }

        let mut rng = SmallRng::from_entropy();
        let bytes = caracteres.as_bytes();

        let mut senhas = Vec::new();
        for _ in 0..quantidade {
            let senha: String = (0..tamanho)
                .map(|_| {
                    let idx = rng.gen_range(0..bytes.len());
                    bytes[idx] as char
                })
                .collect();
            senhas.push(senha);
        }

        senhas.join("\n")
    }

    #[wasm_bindgen(js_name = "getLabels")]
    pub fn get_labels(&self) -> String {
        let labels = json!({
            "senha_gerada": self.idioma.traduzir("senha_gerada"),
            "copiada": self.idioma.traduzir("copiada"),
            "tamanho": self.idioma.traduzir("tamanho"),
            "maiusculas": self.idioma.traduzir("maiusculas"),
            "numeros": self.idioma.traduzir("numeros"),
            "simbolos": self.idioma.traduzir("simbolos"),
            "gerar": self.idioma.traduzir("gerar"),
            "baixar": self.idioma.traduzir("baixar"),
        });
        labels.to_string()
    }
}
