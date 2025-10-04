import init, { Senhas } from "./pkg/senhas.js";

let senhaInst;
let uppercase = true;
let numbers = true;
let special = true;
let idioma = "pt";
let escuro = false;

document.addEventListener("DOMContentLoaded", main);

async function main() {
    await init();
    senhaInst = new Senhas();
    senhaInst.setIdioma(idioma);

    updateOptionButtons();
    updateTheme();
    updateLangBtn();
    updateLabels();

    document.getElementById("theme-btn").onclick = () => {
        escuro = !escuro;
        updateTheme();
    };

    document.getElementById("uppercase-btn").onclick = () => {
        uppercase = !uppercase;
        senhaInst.setMaiusculas(uppercase);
        updateOptionButtons();
    };

    document.getElementById("numbers-btn").onclick = () => {
        numbers = !numbers;
        senhaInst.setNumeros(numbers);
        updateOptionButtons();
    };

    document.getElementById("special-btn").onclick = () => {
        special = !special;
        senhaInst.setSimbolos(special);
        updateOptionButtons();
    };

    const range = document.getElementById("length-range");
    const rangeValue = document.getElementById("length-value");
    range.oninput = () => {
        rangeValue.textContent = range.value;
    };

    document.getElementById("lang-btn").onclick = () => {
        idioma = idioma === "pt" ? "en" : "pt";
        senhaInst.setIdioma(idioma);
        updateLangBtn();
        updateLabels();
        updateOptionButtons();
        updateTheme();
    };

    document.getElementById("generate-btn").onclick = gerarSenha;
    document.getElementById("download-btn").onclick = baixarSenhas;

    gerarSenha();
}

function getParsedLabels() {
    return JSON.parse(senhaInst.getLabels());
}

function updateOptionButtons() {
    const labels = getParsedLabels();
    document.getElementById("uppercase-btn").textContent =
        uppercase ? "🔠 " + labels.maiusculas + ": ON" : "🔡 " + labels.maiusculas + ": OFF";
    document.getElementById("numbers-btn").textContent =
        numbers ? "🔢 " + labels.numeros + ": ON" : "🔢 " + labels.numeros + ": OFF";
    document.getElementById("special-btn").textContent =
        special ? "✨ " + labels.simbolos + ": ON" : "✨ " + labels.simbolos + ": OFF";
    document.getElementById("download-btn").textContent = "📥 " + labels.baixar;
}

function updateTheme() {
    document.body.style.backgroundColor = escuro ? "#1e1e1e" : "#f0f0f0";
    document.body.style.color = escuro ? "#fff" : "#000";
    document.querySelector(".sidebar").style.backgroundColor = escuro ? "#2c2c2c" : "#f9f9f9";

    const themeText = idioma === "pt"
        ? (escuro ? "🌙 Tema Escuro" : "☀️ Tema Claro")
        : (escuro ? "🌙 Dark Theme" : "☀️ Light Theme");

    document.getElementById("theme-btn").textContent = themeText;
}

function updateLangBtn() {
    document.getElementById("lang-btn").textContent =
        idioma === "pt" ? "change to english" : "mudar para português";
}

function updateLabels() {
    const labels = getParsedLabels();
    document.getElementById("options-title").textContent =
        idioma === "pt" ? "⚙️ Opções" : "⚙️ Options";
    document.getElementById("main-title").textContent =
        idioma === "pt" ? "🔐 Gerador de Senhas Fortes" : "🔐 Strong Password Generator";
    document.getElementById("generate-btn").textContent = "🔁 " + labels.gerar;
    document.getElementById("generated-label").textContent = labels.senha_gerada;
    document.getElementById("length-label").innerHTML =
        labels.tamanho + ': <span id="length-value">' + document.getElementById("length-range").value + "</span>";
}

function gerarSenha() {
    const tamanho = parseInt(document.getElementById("length-range").value, 10);
    const senha = senhaInst.gerar(tamanho);
    document.getElementById("senha-card").textContent = senha;
}

function baixarSenhas() {
    const tamanho = parseInt(document.getElementById("length-range").value, 10);
    const conteudo = senhaInst.gerarMultiplas(tamanho, 10);
    const blob = new Blob([conteudo], { type: "text/plain" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "senhas.txt";
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
}
