# gerador de senhas
modifique isto como quiser, pois sem licensa de uso tu pode fazer oque quiser.

## Como fazer o build

Sempre que mudar o código, antes de dar o `git push`, você deve fazer o build do projeto.

Pra fazer o build, instale a ferramenta `make` e rode o comando:

```
make build
```

O que o comando faz?

- limpa os conteúdos da pasta `docs/`
- faz o build do projeto
- joga os arquivos da pasta `pkg`, `web` e `src` e depois cola pra dentro da pasta `docs/`

# aviso importante!!! 
edite o makefile pois ele só funciona na minha maquina

# como compilar e testar
digite:

```make```

caso não queira usar este comando pode usaar estes abaixo
```
cargo install wasm-pack
npm install -g serve
cd docs
serve -l 8080
```
