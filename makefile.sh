#!/bin/bash

web_dir="web"
final_dir="docs"

echo "🛠️ Compilando o projeto..."
find "$final_dir" -type f -exec rm -f {} +

echo "🚧 Executando wasm-pack build..."
wasm-pack build --target web --out-dir "$web_dir/pkg"

echo "📁 Copiando arquivos de $web_dir para $final_dir..."
cp -r "$web_dir/"* "$final_dir/"

echo "✅ Compilação concluída!"
