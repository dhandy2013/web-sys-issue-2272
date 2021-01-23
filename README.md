web_sys issue 2272
==================

https://github.com/rustwasm/wasm-bindgen/issues/2272

To reproduce:
```
wasm-pack build --target=no-modules
python3 -m http.server
# Open browser to the printed URL
# Look at console log messages in browser developer tools
```
