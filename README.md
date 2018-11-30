# Tetris

This is an implementation of Tetris in Rust, compiled into WebAssembly so it
can be presented on a webpage.

## Setup

### Install tools

In order to build and run this project, you must have the following tools
installed:

* a Rust toolchain ([get it here](https://www.rust-lang.org/en-US/install.html))
* `wasm-pack` ([get it here](https://rustwasm.github.io/wasm-pack/installer/))
* `npm` ([get it here](https://www.npmjs.com/get-npm))

### Build and run

Clone this project. In the root directory, build the Rust crate:
```
wasm-pack build -d www/pkg
```

Now the Rust code has been compiled into webassembly and placed in `www/pkg`.

Build the webpage and serve it:

``` 
cd www
npm install
npm start
```

This will start the development server. You can navigate to the webpage at
the address that is printed (for example,
[http://localhost:8080/](http://localhost:8080/)).

