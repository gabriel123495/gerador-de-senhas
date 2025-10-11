# passwords generator
modfy this how do you want, because with no licenses you can do whatever do you want 

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

# important warning!!!
edit the makefile because it only works on my machine

# how to compile and test
type:

```make```

if you dont want to type this command you can try these below:

```
cargo install wasm-pack
npm install -g serve
cd docs
serve -l 8080
``` 