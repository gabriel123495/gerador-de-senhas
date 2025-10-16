$web_dir = "web"
$final_dir = "docs"

echo "🛠️ Compilando o projeto..."
Remove-Item "$final_dir\*" -File -Force

echo "🚧 Executando wasm-pack build..."
wasm-pack build --target web --out-dir "$web_dir/pkg"

echo "📁 Copiando arquivos de $web_dir para $final_dir..."
Copy-Item -Path "$web_dir\*" -Destination $final_dir -Recurse -Force

echo "✅ Compilação concluída!"
