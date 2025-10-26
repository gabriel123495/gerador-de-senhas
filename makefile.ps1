$web_dir = "web"
$final_dir = "docs"

Write-Host "Compilando o projeto..."

Write-Host "Executando wasm-pack build..."
wasm-pack build --target web --out-dir "$web_dir/pkg"

Write-Host "Copiando arquivos de $web_dir para $final_dir..."
Copy-Item "$web_dir\*" -Destination $final_dir -Recurse -Force

Write-Host "Compilação concluída com sucesso!"

npm install -g serve
serve docs -l 8080
