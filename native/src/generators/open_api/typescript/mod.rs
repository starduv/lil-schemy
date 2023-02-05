mod syntax_kind;

use neon::{prelude::*, result::Throw};

use self::syntax_kind::*;

pub struct TsNode<'cx> {
    ast: Handle<'cx, JsObject>,
}

impl<'cx> TsNode<'cx> {
    pub(crate) fn new(ast: Handle<'cx, JsObject>) -> Self {
        TsNode { ast }
    }

    pub(crate) fn get_api_paths(&mut self, cx: &mut FunctionContext) -> Result<Vec<TsNode<'cx>>, Throw> {
        let mut api_paths = Vec::new();

        self.find_paths(&mut api_paths, cx)?;

        Ok(api_paths)
    }

    fn find_paths(&mut self, api_paths: &mut Vec<TsNode<'cx>>, cx: &mut FunctionContext) -> Result<(), Throw> {
        let kind = self.ast.get::<JsNumber, FunctionContext, &str>(cx, "kind")?.value(cx);

        if kind == IMPORT_DECLARATION {}

        Ok(())
    }
}
