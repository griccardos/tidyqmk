#clean build directory, build wasm, and copy to wasm directory
#default action
default: all

#clean build directory
#only for macos
[macos]
all:
    rm -rf build
    mkdir build
    wasm-pack build --target web --out-dir build
    cp html/index.html build/index.html
    python3 -m http.server --directory build

[windows]
all:
    #!powershell.exe
    rd build
    md build
    wasm-pack build --target web --out-dir build
    copy html\index.html build\index.html
    python -m http.server --directory build
