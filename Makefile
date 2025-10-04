WASM_PACK := /mnt/c/Users/gabri/.cargo/bin/wasm-pack.exe
WEB_DIR := web
OUT_DIR := docs

build:
	@echo "🧹 Limpando o conteúdo da pasta $(OUT_DIR)..."
	@mkdir -p $(OUT_DIR)
	@rm -rf $(OUT_DIR)/*
	@echo "🔧 Compilando com wasm-pack..."
	@$(WASM_PACK) build --target web --out-dir $(WEB_DIR)/pkg
	@echo "📁 Copiando arquivos da pasta $(WEB_DIR) para $(OUT_DIR)..."
	@cp $(WEB_DIR)/index.html $(OUT_DIR)/
	@cp $(WEB_DIR)/styles.css $(OUT_DIR)/
	@cp $(WEB_DIR)/script.js $(OUT_DIR)/
	@mkdir -p $(OUT_DIR)/pkg
	@cp -r $(WEB_DIR)/pkg/* $(OUT_DIR)/pkg/
	@echo "✅ Build concluído! Verifique a pasta $(OUT_DIR)."

.PHONY: build
