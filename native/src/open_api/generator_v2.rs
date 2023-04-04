use deno_ast::{ParseParams, ParsedSource, SourceTextInfo};
use dprint_swc_ext::view::{with_ast_view_for_module, ModuleInfo, NodeTrait};
use url::Url;

use super::{add_paths::find_paths, open_api::OpenApi};

pub(crate) fn from_source_file(path: String) -> OpenApi {
    let mut open_api = OpenApi::new();
    let result = get_syntax_tree(path);
    with_ast_view_for_module(
        ModuleInfo {
            module: result.module(),
            comments: None,
            text_info: Some(result.text_info()),
            tokens: None,
        },
        |module| find_paths(&mut open_api, &module.as_node()),
    );

    open_api
}

fn get_syntax_tree(path: String) -> ParsedSource {
    let specifier = Url::from_file_path(path).unwrap();
    let source_text = std::fs::read_to_string(specifier.path()).unwrap();
    let parsed_source = deno_ast::parse_module(ParseParams {
        capture_tokens: true,
        maybe_syntax: None,
        media_type: deno_ast::MediaType::TypeScript,
        scope_analysis: false,
        specifier: specifier.to_string(),
        text_info: SourceTextInfo::new(source_text.into()),
    })
    .unwrap();

    parsed_source
}
