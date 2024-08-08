#clean build directory, build wasm, and copy to wasm directory
#default action
all: clean build copy run

#clean build directory
clean:
    rm -rf build

#build wasm
build:
    mkdir build
    wasm-pack build --target web --out-dir build

#copy wasm to wasm directory
copy:
    cp html/index.html build/index.html

run:
    python3 -m http.server --directory build
