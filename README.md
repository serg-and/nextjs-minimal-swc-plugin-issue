Minimal reproduction of issue with next SWC plugin runner.

Next's pluggin runner no longer provides the known file path to the metadata for a SWC plugin, only providing the file name.

This is conflict with SWC, saying: [swc docs](https://rustdoc.swc.rs/swc_common/plugin/metadata/struct.TransformPluginMetadataContext.html#structfield.filename)

```rust
/// Host side metadata context plugin may need to access.
/// This is a global context - any plugin in single transform will have same
/// values.
pub struct TransformPluginMetadataContext {
    /// The path of the file being processed. This includes all of the path as
    /// much as possible.
    pub filename: Option<String>,

    ...
}
```

This breaks several packages such as [next-superjson-plugin](https://github.com/blitz-js/next-superjson-plugin) which rely on the path to determine whether some files are page or app router based.

I believe the issue originates from here https://github.com/vercel/next.js/blob/v15.0.1/turbopack/crates/turbopack-ecmascript-plugins/src/transform/swc_ecma_transform_plugins.rs#L189

`Some(ctx.file_name_str.to_string())` could simply be changed to `Some(ctx.file_path_str.to_string())`

## Example

Simple SWC plugin that logs the metadata filename of the transformed file, shows that only the filename is provided.

## Run

```sh
npm i
cd swc_plugin
npm run prepublish
cd ..
npm run dev
```
