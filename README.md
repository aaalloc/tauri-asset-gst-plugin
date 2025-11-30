Build with

```
cargo build --release
```

and then get the absolute path of

```
./target/release/libgsttauriasset.so
```

And use it when using your tauri project like

```
GST_PLUGIN_PATH=<abs-path-of-libgsttauriasset.so> npm run tauri dev
```
