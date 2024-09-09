<div align="center">

  <h1 style="line-height: 40px; vertical-align: bottom; display: inline-flex; align-items: center;"><img src="logo.svg" width="40"/><code>sm4-sdk</code></h1>

  <strong>sm4-sdk for keeping the key inside the wasm.</strong>

  <sub>Built with 🦀🕸 by <a href="https://rustwasm.github.io/">The Rust and WebAssembly Working Group</a></sub>
</div>

## 🚴 Usage

### 🐑 Get releases from this project and use in your project



### 🛠️ Clone this project and build with `wasm-pack build`

```
git clone https://github.com/honwhy/sm4-sdk.git
wasm-pack build --target web
```

for windows, set default host to `x86_64-pc-windows-msvc` which may save you some time.

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --firefox
# or
wasm-pack test --headless --chrome
```

### 🔥 Run demo html with wasm

```
python -m http.server 8080
```
open `http://localhost:8000/example.html` in your browser.

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.

## 🫛 Insider

this sm4-sdk use SM4 algorithm with PKCS5Padding padding mode.

## License

Licensed under [LICENSE-MIT]( http://opensource.org/licenses/MIT)

