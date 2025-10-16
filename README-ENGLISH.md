<<<<<<< HEAD
# 🔐 Password Generator
Generate secure passwords!

---

## ⚙️ How to Compile and Test

First, you need to install Nushell from:  
[https://github.com/nushell/nushell/releases](https://github.com/nushell/nushell/releases)

Then run the following commands:
```bash
.\build.nu
npm install -g serve
serve docs -l 8080

# extra
if you want to contact the creator of the project my :gmail is gabriel.leonardo.cruz2012@gmail.com
=======
# passwords generator
generate safe passwords!

## how to build

every time that you change the code, before using `git push` you need to make the build of the project

if you are on windows type: 
```.\makefile.ps1``` 

if you are on linux/macOS/BSD:

```.\makefile.sh```

What it does?

- delete the content on the `/docs`folder
- build the project
- throw the files of the folder `pkg`, `web`and `src`and after it paste on the `/docs`folder


if you dont want to type this command you can try these below:

```
cargo install wasm-pack
npm install -g serve
cd docs
serve -l 8080
``` 

# extra
if you want to contact the creator of the project my gmail is gabriel.leonardo.cruz2012@gmail.com
>>>>>>> c0c5963e23bea3d30fb2751ae83a2b975253e617
