use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
use swc_core::{ecma::ast::Program, plugin::metadata::TransformPluginMetadataContextKind};

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    match _metadata.get_context(&TransformPluginMetadataContextKind::Filename) {
        Some(filename) => println!("got filename: {filename}"),
        _ => {}
    };

    program
}
