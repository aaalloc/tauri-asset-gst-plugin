Build with

```
cargo build --release
```

and then get the absolute path of

```
./target/release
```

And use it when using your tauri project like

```
GST_PLUGIN_PATH=<folder-path-where-libgsttauriasset.so> npm run tauri dev
```
