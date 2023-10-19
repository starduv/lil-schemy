use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};

use swc_ecma_ast::{Prop, PropName, Expr, Lit};
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax};

use std::{
    cell::RefCell,
    collections::BTreeMap,
    path::Path,
    sync::{Arc, RwLock}, ops::Deref,
};

use crate::{messaging::MessageBus, schemy::SchemyPath};

use super::{node::GetProperty, node_kind::NodeKind, symbol_tables::SymbolTables, Node};

pub struct Application {
    modules: BTreeMap<String, Arc<Node<'static>>>,
    source_map: Lrc<SourceMap>,
    symbol_tables: SymbolTables,
}

impl Application {
    pub(crate) fn new() -> Self {
        Application {
            modules: Default::default(),
            source_map: Default::default(),
            symbol_tables: Default::default(),
        }
    }

    pub(crate) fn get_module(&mut self, path: &str, bus: MessageBus) -> Arc<Node<'static>> {
        if !self.modules.contains_key(path) {
            let fm = self
                .source_map
                .load_file(Path::new(path))
                .expect(format!("Could not load file '{}'", path).as_str());

            let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(self.source_map.clone()));

            let lexer = Lexer::new(
                Syntax::Typescript(Default::default()),
                Default::default(),
                StringInput::from(&*fm),
                None,
            );

            let mut parser = Parser::new_from(lexer);

            for e in parser.take_errors() {
                e.into_diagnostic(&handler).emit();
            }

            let module = parser
                .parse_module()
                .map_err(|e| {
                    // Unrecoverable fatal error occurred
                    e.into_diagnostic(&handler).emit()
                })
                .expect(format!("Could not parse module '{}'", path).as_str());

            self.modules
                .insert(path.to_string(), Arc::new(Node::new(NodeKind::Module(module), None)));
        }

        self.modules.get(path).unwrap().clone()
    }

    pub(crate) fn run(&mut self, bus: MessageBus) -> () {
        let mut requested_modules = BTreeMap::<String, usize>::new();

        bus.gather_module_requests(|send_id, module_path| {
            let module_id = requested_modules.len();
            requested_modules.entry(module_path).or_insert_with(|| {
                bus.send_module_id(send_id, module_id);
                module_id
            });
        });

        for (module_path, module_id) in requested_modules {
            let node = self.get_module(&module_path, bus.clone());
            self.find_paths(node, &module_path, bus.clone());
        }

        bus.begin_serialize();
    }

    fn find_paths(&mut self, root: Arc<Node<'static>>, module_path: &str, bus: MessageBus) -> () {
        self.symbol_tables.store_declaration_maybe(root.clone(), module_path);

        for child in root.children(Arc::downgrade(&root)) {
            if child.is_path() {
                child.with_parent(|parent| {
                    parent.with_parent(|parent| {
                        self.symbol_tables.add_child_scope(module_path);
                        bus.schemy_created(self.create_schemy_path(parent, module_path));
                        self.symbol_tables.parent_scope(module_path);
                    })
                })
            } else {
                self.find_paths(child, module_path, bus)
            }
        }
    }

    fn create_schemy_path(&mut self, root: Arc<Node>, file_path: &str) -> SchemyPath {
        let mut schemy = SchemyPath::default();

        let args = root.call_arguments(Arc::downgrade(&root));
        let route_handler = args.first().unwrap();
        let route_options = args.last().unwrap();
        let options = set_options(&mut schemy, route_options);

        if let Some(path) = &options.path {
            if let Some(method) = &options.method {
                let operation = open_api.path(&path).add_operation(&method).clone();

                {
                    let mut borrow = (*operation).borrow_mut();
                    borrow.tags(options.tags.clone());
                }

                self.add_request_details(&operation, route_handler.clone(), file_path, &options);
            }
        }

        schemy
    }
}

fn set_options(schemy_path: &mut SchemyPath, options: &Arc<Node>) -> () {
    match options.kind {
        NodeKind::ObjectLit(raw_literal) => {
            for prop_or_spread in &raw_literal.props {
                match prop_or_spread.as_prop() {
                    Some(prop) => match prop.as_ref() {
                        Prop::KeyValue(key_value) => match &key_value.key {
                            PropName::Ident(i) if i.sym.eq("method") => {
                                schemy_path.method = match key_value.value.deref() {
                                    Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                                    _ => None,
                                }
                            }
                            PropName::Ident(i) if i.sym.eq("path") => {
                                schemy_path.path = match key_value.value.deref() {
                                    Expr::Lit(Lit::Str(s)) => Some(s.value.to_string()),
                                    _ => None,
                                }
                            }
                            PropName::Ident(i) if i.sym.eq("tags") => {
                                let mut tags = vec![];
                                match key_value.value.deref() {
                                    Expr::Array(literal) => {
                                        for element in &literal.elems {
                                            if let Some(element) = element {
                                                match element.expr.deref() {
                                                    Expr::Lit(Lit::Str(s)) => tags.push(s.value.to_string()),
                                                    _ => {}
                                                }
                                            }
                                        }
                                    }
                                    _ => {}
                                }

                                if tags.len() > 0 {
                                    schemy_path.tags = Some(tags);
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    },
                    None => {}
                }
            }
        },
        _ => {}
    }
}
