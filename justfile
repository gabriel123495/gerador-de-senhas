web_dir := "web"
final_dir := "docs"

build:
    echo "Compilando o projeto..."
    echo "Executando wasm-pack build..."
    wasm-pack build --target web --out-dir "{{web_dir}}/pkg"
    echo "Copiando arquivos de {{web_dir}} para {{final_dir}}..."
    powershell -Command "Copy-Item '{{web_dir}}\\*' -Destination '{{final_dir}}' -Recurse -Force"
    echo "Compilação concluída com sucesso!"

serve:
    npm install -g serve
    serve docs -l 8080
