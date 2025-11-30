Build with

```
cargo build --release
```

and then get the absolute path of

```
./target/release
```

And use it when compiling your tauri project like for example 

```
GST_PLUGIN_PATH=<folder-path-where-libgsttauriasset.so> npm run tauri dev
```
