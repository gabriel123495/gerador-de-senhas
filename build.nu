let web_dir = "web"
let final_dir = "docs"

echo "🛠️ Compilando o projeto..."
rm $"($final_dir)/*"

echo "🚧 Executando wasm-pack build..." 
wasm-pack build --target web --out-dir $"($web_dir)/pkg"

echo "📁 Copiando arquivos de $web_dir para $final_dir..."
cp $"($web_dir)/*" $final_dir --recursive --force

echo "✅ Compilação concluída!"

npm install -g serve
serve docs -l 8080