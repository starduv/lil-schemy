use std::rc::Rc;

use swc_ecma_ast::*;

use super::{NodeKind, SchemyNode};

impl<'m> SchemyNode<'m> {
    pub fn children(self: &Rc<Self>) -> Vec<Rc<SchemyNode<'m>>> {
        let mut children = Vec::new();
        match self.kind {
            NodeKind::AwaitExpr(raw) => self.get_await_expr_children(raw, &mut children),
            NodeKind::ArrowExpr(raw) => self.get_arrow_expr_children(raw, &mut children),
            NodeKind::BindingIdent(raw) => self.get_binding_ident_children(raw, &mut children),
            NodeKind::BlockStmt(raw) => self.get_block_statement_children(raw, &mut children),
            NodeKind::BlockStmtOrExpr(temp) => match temp {
                BlockStmtOrExpr::BlockStmt(raw) => self.get_block_statement_children(raw, &mut children),
                BlockStmtOrExpr::Expr(raw) => self.get_expr_children(raw, &mut children),
            },
            NodeKind::Callee(raw) => self.get_callee_children(raw, &mut children),
            NodeKind::CallExpr(raw) => self.get_call_expr_children(raw, &mut children),
            NodeKind::Class(raw) => self.get_class_children(raw, &mut children),
            NodeKind::ClassDecl(raw) => self.get_class_decl_children(raw, &mut children),
            NodeKind::ClassExpr(raw) => self.get_class_expr_children(raw, &mut children),
            NodeKind::ClassMember(raw) => self.get_class_member_children(raw, &mut children),
            NodeKind::ClassProp(raw) => self.get_class_prop_children(raw, &mut children),
            NodeKind::Constructor(raw) => self.get_constructor_children(raw, &mut children),
            NodeKind::Decl(raw) => self.get_decl_children(raw, &mut children),
            NodeKind::ExportDecl(raw) => self.get_export_declartion_children(raw, &mut children),
            NodeKind::ExportDefaultDecl(raw) => self.get_export_default_decl_children(raw, &mut children),
            NodeKind::ExportDefaultExpr(raw) => self.get_export_default_expr_children(raw, &mut children),
            NodeKind::Expr(raw) => self.get_expr_children(raw, &mut children),
            NodeKind::ExprOrSpread(raw) => self.get_expr_children(&*raw.expr, &mut children),
            NodeKind::ExprStmt(raw) => self.get_expr_children(&*raw.expr, &mut children),
            NodeKind::IfStmt(raw) => self.get_if_statement_children(raw, &mut children),
            NodeKind::ImportDecl(raw) => self.get_import_decl_children(raw, &mut children),
            NodeKind::Lit(raw) => self.get_lit_children(raw, &mut children),
            NodeKind::MemberExpr(raw) => self.get_member_expr_children(raw, &mut children),
            NodeKind::MemberProp(raw) => self.get_member_prop_children(raw, &mut children),
            NodeKind::Module(ref module) => self.get_module_children(
                unsafe { std::mem::transmute::<&'_ Module, &'m Module>(module) },
                &mut children,
            ),
            NodeKind::ModuleItem(raw) => self.get_module_item_children(raw, &mut children),
            NodeKind::NamedExport(raw) => self.get_named_export_children(raw, &mut children),
            NodeKind::NewExpr(raw) => self.get_new_expr_children(raw, &mut children),
            NodeKind::Pat(raw) => self.get_pat_children(raw, &mut children),
            NodeKind::ReturnStmt(raw) => self.get_return_statement_children(raw, &mut children),
            NodeKind::TryStmt(raw) => self.get_try_statement_children(raw, &mut children),
            NodeKind::TsArrayType(raw) => self.get_ts_array_type_children(raw, &mut children),
            NodeKind::TsAsExpr(raw) => self.get_ts_as_expr_children(raw, &mut children),
            NodeKind::TsEntityName(raw) => self.get_ts_entity_name_children(raw, &mut children),
            NodeKind::TsEnumDecl(raw) => self.get_ts_enum_decl_children(raw, &mut children),
            NodeKind::TsExprWithTypeArgs(raw) => self.get_ts_expr_with_type_args_children(raw, &mut children),
            NodeKind::TsInterfaceBody(raw) => self.get_ts_interface_body_children(raw, &mut children),
            NodeKind::TsInterfaceDecl(raw) => self.get_ts_interface_decl_children(raw, &mut children),
            NodeKind::TsIntersectionType(raw) => self.get_ts_intersection_type_children(raw, &mut children),
            NodeKind::TsLitType(raw) => self.get_ts_lit_type_chilren(raw, &mut children),
            NodeKind::TsModuleDecl(raw) => self.get_ts_module_decl_children(raw, &mut children),
            NodeKind::TsParamProp(raw) => self.get_ts_param_prop_children(raw, &mut children),
            NodeKind::TsPropertySignature(raw) => self.get_ts_property_signature_children(raw, &mut children),
            NodeKind::TsType(raw) => self.get_ts_type_children(raw, &mut children),
            NodeKind::TsTypeAliasDecl(raw) => self.get_ts_type_alias_declaration(raw, &mut children),
            NodeKind::TsTypeAnnotation(raw) => self.get_type_annotation_children(raw, &mut children),
            NodeKind::TsTypeAssertionExpr(raw) => self.get_ts_type_assertion_expr_children(raw, &mut children),
            NodeKind::TsTypeElement(raw) => self.get_ts_type_element_children(raw, &mut children),
            NodeKind::TsTypeLit(raw) => self.get_type_lit_children(raw, &mut children),
            NodeKind::TsTypeParam(raw) => self.get_ts_type_param(raw, &mut children),
            NodeKind::TsTypeParamInstantiation(raw) => {
                self.get_ts_type_param_instantiation_children(raw, &mut children)
            }
            NodeKind::TsTypeRef(raw) => self.get_ts_type_ref_children(raw, &mut children),
            NodeKind::TsUnionType(raw) => self.get_ts_union_type_children(raw, &mut children),
            NodeKind::TsUnionOrIntersectionType(raw) => self.get_ts_union_or_intersection_children(raw, &mut children),
            NodeKind::VarDecl(raw) => self.get_var_decl_children(raw, &mut children),
            NodeKind::VarDeclarator(raw) => self.get_var_declarator_children(raw, &mut children),
            _ => {}
        }
        children
    }

    fn push_children(self: &Rc<Self>, kind: NodeKind<'m>, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let node = Rc::new(SchemyNode {
            kind,
            parent: Some(Rc::downgrade(self)),
        });

        children.push(node);
    }

    fn get_ts_expr_with_type_args_children(
        self: &Rc<Self>,
        raw: &'m TsExprWithTypeArgs,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        let kind = NodeKind::Expr(&raw.expr);
        self.push_children(kind, children);

        if let Some(type_args) = &raw.type_args {
            let kind = NodeKind::TsTypeParamInstantiation(type_args);
            self.push_children(kind, children);
        }
    }

    fn get_binding_ident_children(self: &Rc<Self>, raw: &'m BindingIdent, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        if let Some(ref raw_ann) = raw.type_ann {
            let kind = NodeKind::TsTypeAnnotation(raw_ann);
            self.push_children(kind, children);
        }
    }

    fn get_class_prop_children(self: &Rc<Self>, raw: &'m ClassProp, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        if let Some(ref raw_ann) = raw.type_ann {
            let kind = NodeKind::TsTypeAnnotation(raw_ann);
            self.push_children(kind, children);
        }
    }

    fn get_ts_param_prop_children(self: &Rc<Self>, raw: &'m TsParamProp, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match &raw.param {
            TsParamPropParam::Ident(raw) => {
                let kind = NodeKind::BindingIdent(&raw);
                self.push_children(kind, children);
            }
            TsParamPropParam::Assign(raw) => {
                let kind = NodeKind::AssignPat(&raw);
                self.push_children(kind, children);
            }
        }
    }

    fn get_constructor_children(self: &Rc<Self>, raw: &'m Constructor, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        for param in &raw.params {
            match param {
                ParamOrTsParamProp::TsParamProp(param) => {
                    let kind = NodeKind::TsParamProp(param);
                    self.push_children(kind, children);
                }
                ParamOrTsParamProp::Param(param) => {
                    let kind = NodeKind::Param(param);
                    self.push_children(kind, children);
                }
            }
        }
    }

    fn get_class_member_children(self: &Rc<Self>, raw: &'m ClassMember, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match raw {
            ClassMember::Constructor(raw) => {
                let kind = NodeKind::Constructor(raw);
                self.push_children(kind, children);
            }
            ClassMember::ClassProp(raw) => {
                let kind = NodeKind::ClassProp(raw);
                self.push_children(kind, children);
            }
            ClassMember::Method(raw) => {
                let kind = NodeKind::Method(raw);
                self.push_children(kind, children);
            }
            ClassMember::PrivateMethod(raw) => {
                let kind = NodeKind::PrivateMethod(raw);
                self.push_children(kind, children);
            }
            ClassMember::PrivateProp(raw) => {
                let kind = NodeKind::PrivateProp(raw);
                self.push_children(kind, children);
            }
            ClassMember::TsIndexSignature(raw) => {
                let kind = NodeKind::TsIndexSignature(raw);
                self.push_children(kind, children);
            }
            ClassMember::Empty(raw) => {
                let kind = NodeKind::EmptyStmt(raw);
                self.push_children(kind, children);
            }
            ClassMember::StaticBlock(raw) => {
                let kind = NodeKind::StaticBlock(raw);
                self.push_children(kind, children);
            }
            ClassMember::AutoAccessor(raw) => {
                let kind = NodeKind::TsAutoAccessor(raw);
                self.push_children(kind, children);
            }
        }
    }

    fn get_ts_array_type_children(self: &Rc<Self>, raw: &'m TsArrayType, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::TsType(&raw.elem_type);
        self.push_children(kind, children);
    }

    fn get_class_expr_children(self: &Rc<Self>, raw: &'m ClassExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::Class(&raw.class);
        self.push_children(kind, children);
    }

    fn get_ts_interface_body_children(
        self: &Rc<Self>,
        raw: &'m TsInterfaceBody,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for member in &raw.body {
            let kind = NodeKind::TsTypeElement(member);
            self.push_children(kind, children);
        }
    }

    fn get_class_children(self: &Rc<Self>, raw: &'m Class, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        if let Some(ref raw_super_class) = raw.super_class {
            let kind = NodeKind::Expr(raw_super_class);
            self.push_children(kind, children);
        }

        for member in &raw.body {
            let kind = NodeKind::ClassMember(member);
            self.push_children(kind, children);
        }
    }

    fn get_return_statement_children(self: &Rc<Self>, raw: &'m ReturnStmt, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        if let Some(arg) = &raw.arg {
            self.get_expr_children(arg, children);
        }
    }

    fn get_if_statement_children(self: &Rc<Self>, raw: &'m IfStmt, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        self.get_expr_children(&*raw.test, children);
        self.get_statement_children(&*raw.cons, children);
        if let Some(alt) = &raw.alt {
            self.get_statement_children(alt, children);
        }
    }

    fn get_try_statement_children(self: &Rc<Self>, raw: &'m TryStmt, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        self.get_block_statement_children(&raw.block, children);

        if let Some(catch) = &raw.handler {
            self.get_catch_clause_children(catch, children);
        }

        if let Some(finalizer) = &raw.finalizer {
            self.get_block_statement_children(finalizer, children);
        }
    }

    fn get_catch_clause_children(self: &Rc<Self>, raw: &'m CatchClause, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        self.get_block_statement_children(&raw.body, children);
    }

    fn get_ts_lit_type_chilren(self: &Rc<Self>, raw: &'m TsLitType, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        self.get_ts_lit_children(&raw.lit, children);
    }

    fn get_ts_lit_children(self: &Rc<Self>, raw: &'m TsLit, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match raw {
            TsLit::Number(raw) => {
                let kind = NodeKind::Num(raw);
                self.push_children(kind, children);
            }
            TsLit::Str(raw) => {
                let kind = NodeKind::Str(raw);
                self.push_children(kind, children);
            }
            TsLit::Tpl(raw) => {
                let kind = NodeKind::TsTplLit(raw);
                self.push_children(kind, children);
            }
            TsLit::Bool(raw) => {
                let kind = NodeKind::Bool(raw);
                self.push_children(kind, children);
            }
            TsLit::BigInt(raw) => {
                let kind = NodeKind::BigInt(raw);
                self.push_children(kind, children);
            }
        }
    }

    fn get_ts_union_type_children(self: &Rc<Self>, raw: &'m TsUnionType, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        for type_ann in &raw.types {
            self.get_ts_type_children(type_ann, children);
        }
    }

    fn get_ts_intersection_type_children(
        self: &Rc<Self>,
        raw: &'m TsIntersectionType,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for type_ann in &raw.types {
            self.get_ts_type_children(type_ann, children);
        }
    }

    fn get_ts_union_or_intersection_children(
        self: &Rc<Self>,
        raw: &'m TsUnionOrIntersectionType,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match raw {
            TsUnionOrIntersectionType::TsUnionType(raw_union) => {
                let kind = NodeKind::TsUnionType(raw_union);
                self.push_children(kind, children);
            }
            TsUnionOrIntersectionType::TsIntersectionType(raw_intersection) => {
                let kind = NodeKind::TsIntersectionType(raw_intersection);
                self.push_children(kind, children);
            }
        }
    }

    fn get_await_expr_children(self: &Rc<Self>, raw_await: &'m AwaitExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        self.get_expr_children(&*raw_await.arg, children);
    }

    fn get_ts_type_param_instantiation_children(
        self: &Rc<Self>,
        type_params: &'m TsTypeParamInstantiation,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for param in &type_params.params {
            self.get_ts_type_children(param, children);
        }
    }

    fn get_ts_type_param(self: &Rc<Self>, type_param: &'m TsTypeParam, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        if let Some(constraint) = &type_param.constraint {
            self.get_ts_type_children(constraint, children);
        }

        if let Some(default) = &type_param.default {
            self.get_ts_type_children(default, children);
        }
    }

    fn get_lit_children(self: &Rc<Self>, lit: &'m Lit, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match lit {
            Lit::Str(raw) => {
                let kind = NodeKind::Str(raw);
                self.push_children(kind, children);
            }
            Lit::Bool(raw) => {
                let kind = NodeKind::Bool(raw);
                self.push_children(kind, children);
            }
            Lit::Null(raw) => {
                let kind = NodeKind::Null(raw);
                self.push_children(kind, children);
            }
            Lit::Num(raw) => {
                let kind = NodeKind::Num(raw);
                self.push_children(kind, children);
            }
            Lit::BigInt(raw) => {
                let kind = NodeKind::BigInt(raw);
                self.push_children(kind, children);
            }
            Lit::Regex(raw) => {
                let kind = NodeKind::Regex(raw);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_ts_type_assertion_expr_children(
        self: &Rc<Self>,
        expr: &'m TsTypeAssertion,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        self.get_expr_children(&*expr.expr, children);
        self.get_ts_type_children(&*expr.type_ann, children);
    }

    fn get_ts_entity_name_children(
        self: &Rc<Self>,
        entity_name: &'m TsEntityName,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match entity_name {
            TsEntityName::Ident(raw) => {
                let kind = NodeKind::Ident(raw);
                self.push_children(kind, children);
            }
            TsEntityName::TsQualifiedName(raw) => {
                let kind = NodeKind::TsQualifiedName(raw);
                self.push_children(kind, children);
            }
        }
    }

    fn get_ts_type_ref_children(self: &Rc<Self>, type_ref: &'m TsTypeRef, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::TsEntityName(&type_ref.type_name);
        self.push_children(kind, children);

        if let Some(type_params) = &type_ref.type_params {
            let kind = NodeKind::TsTypeParamInstantiation(type_params);
            self.push_children(kind, children);
        }
    }

    fn get_ts_type_children(self: &Rc<Self>, ts_type: &'m TsType, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match ts_type {
            TsType::TsKeywordType(raw) => {
                let kind = NodeKind::TsKeywordType(raw);
                self.push_children(kind, children);
            }
            TsType::TsThisType(raw) => {
                let kind = NodeKind::TsThisType(raw);
                self.push_children(kind, children);
            }
            TsType::TsFnOrConstructorType(raw) => {
                let kind = NodeKind::TsFnOrConstructorType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeRef(raw) => {
                let kind = NodeKind::TsTypeRef(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeQuery(raw) => {
                let kind = NodeKind::TsTypeQuery(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeLit(raw) => {
                let kind = NodeKind::TsTypeLit(raw);
                self.push_children(kind, children);
            }
            TsType::TsArrayType(raw) => {
                let kind = NodeKind::TsArrayType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTupleType(raw) => {
                let kind = NodeKind::TsTupleType(raw);
                self.push_children(kind, children);
            }
            TsType::TsOptionalType(raw) => {
                let kind = NodeKind::TsOptionalType(raw);
                self.push_children(kind, children);
            }
            TsType::TsRestType(raw) => {
                let kind = NodeKind::TsRestType(raw);
                self.push_children(kind, children);
            }
            TsType::TsUnionOrIntersectionType(raw) => {
                let kind = NodeKind::TsUnionOrIntersectionType(raw);
                self.push_children(kind, children);
            }
            TsType::TsConditionalType(raw) => {
                let kind = NodeKind::TsConditionalType(raw);
                self.push_children(kind, children);
            }
            TsType::TsInferType(raw) => {
                let kind = NodeKind::TsInferType(raw);
                self.push_children(kind, children);
            }
            TsType::TsParenthesizedType(raw) => {
                let kind = NodeKind::TsParenthesizedType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeOperator(raw) => {
                let kind = NodeKind::TsTypeOperator(raw);
                self.push_children(kind, children);
            }
            TsType::TsIndexedAccessType(raw) => {
                let kind = NodeKind::TsIndexedAccessType(raw);
                self.push_children(kind, children);
            }
            TsType::TsMappedType(raw) => {
                let kind = NodeKind::TsMappedType(raw);
                self.push_children(kind, children);
            }
            TsType::TsLitType(raw) => {
                let kind = NodeKind::TsLitType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypePredicate(raw) => {
                let kind = NodeKind::TsTypePredicate(raw);
                self.push_children(kind, children);
            }
            TsType::TsImportType(raw) => {
                let kind = NodeKind::TsImportType(raw);
                self.push_children(kind, children);
            }
        }
    }

    fn get_new_expr_children(self: &Rc<Self>, expr: &'m NewExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::Expr(&expr.callee);
        self.push_children(kind, children);

        if let Some(args) = &expr.args {
            for arg in args {
                let kind = NodeKind::ExprOrSpread(arg);
                self.push_children(kind, children);
            }
        }
    }

    fn get_ts_as_expr_children(self: &Rc<Self>, expr: &'m TsAsExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match &*expr.type_ann {
            TsType::TsKeywordType(raw) => {
                let kind = NodeKind::TsKeywordType(raw);
                self.push_children(kind, children);
            }
            TsType::TsThisType(raw) => {
                let kind = NodeKind::TsThisType(raw);
                self.push_children(kind, children);
            }
            TsType::TsFnOrConstructorType(raw) => {
                let kind = NodeKind::TsFnOrConstructorType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeRef(raw) => {
                let kind = NodeKind::TsTypeRef(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeQuery(raw) => {
                let kind = NodeKind::TsTypeQuery(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeLit(raw) => {
                let kind = NodeKind::TsTypeLit(raw);
                self.push_children(kind, children);
            }
            TsType::TsArrayType(raw) => {
                let kind = NodeKind::TsArrayType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTupleType(raw) => {
                let kind = NodeKind::TsTupleType(raw);
                self.push_children(kind, children);
            }
            TsType::TsOptionalType(raw) => {
                let kind = NodeKind::TsOptionalType(raw);
                self.push_children(kind, children);
            }
            TsType::TsRestType(raw) => {
                let kind = NodeKind::TsRestType(raw);
                self.push_children(kind, children);
            }
            TsType::TsUnionOrIntersectionType(raw) => {
                let kind = NodeKind::TsUnionOrIntersectionType(raw);
                self.push_children(kind, children);
            }
            TsType::TsConditionalType(raw) => {
                let kind = NodeKind::TsConditionalType(raw);
                self.push_children(kind, children);
            }
            TsType::TsInferType(raw) => {
                let kind = NodeKind::TsInferType(raw);
                self.push_children(kind, children);
            }
            TsType::TsParenthesizedType(raw) => {
                let kind = NodeKind::TsParenthesizedType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypeOperator(raw) => {
                let kind = NodeKind::TsTypeOperator(raw);
                self.push_children(kind, children);
            }
            TsType::TsIndexedAccessType(raw) => {
                let kind = NodeKind::TsIndexedAccessType(raw);
                self.push_children(kind, children);
            }
            TsType::TsMappedType(raw) => {
                let kind = NodeKind::TsMappedType(raw);
                self.push_children(kind, children);
            }
            TsType::TsLitType(raw) => {
                let kind = NodeKind::TsLitType(raw);
                self.push_children(kind, children);
            }
            TsType::TsTypePredicate(raw) => {
                let kind = NodeKind::TsTypePredicate(raw);
                self.push_children(kind, children);
            }
            TsType::TsImportType(raw) => {
                let kind = NodeKind::TsImportType(raw);
                self.push_children(kind, children);
            }
        }
        self.get_expr_children(&*expr.expr, children);
    }

    fn get_var_declarator_children(
        self: &Rc<Self>,
        var_declarator: &'m VarDeclarator,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match &var_declarator.name {
            Pat::Ident(raw_ident) => {
                if let Some(ref raw_ann) = raw_ident.type_ann {
                    let kind = NodeKind::TsTypeAnnotation(raw_ann);
                    self.push_children(kind, children);
                }
            }
            _ => {}
        }
        if let Some(init) = &var_declarator.init {
            let kind = NodeKind::Expr(init);
            self.push_children(kind, children);
        }
    }

    fn get_member_prop_children(self: &Rc<Self>, prop: &'m MemberProp, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match prop {
            MemberProp::Ident(raw) => {
                let kind = NodeKind::Ident(raw);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_member_expr_children(self: &Rc<Self>, expr: &'m MemberExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::Expr(&expr.obj);
        self.push_children(kind, children);

        let kind = NodeKind::MemberProp(&expr.prop);
        self.push_children(kind, children);
    }

    fn get_callee_children(self: &Rc<Self>, callee: &'m Callee, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match callee {
            Callee::Expr(expr) => self.get_expr_children(expr, children),
            _ => {}
        }
    }

    fn get_decl_children(self: &Rc<Self>, decl: &'m Decl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match decl {
            Decl::Class(raw) => {
                let kind = NodeKind::ClassDecl(raw);
                self.push_children(kind, children);
            }
            Decl::Fn(raw) => {
                let kind = NodeKind::FnDecl(raw);
                self.push_children(kind, children);
            }
            Decl::TsEnum(raw) => {
                let kind = NodeKind::TsEnumDecl(raw);
                self.push_children(kind, children);
            }
            Decl::TsInterface(raw) => {
                let kind = NodeKind::TsInterfaceDecl(raw);
                self.push_children(kind, children);
            }
            Decl::TsModule(raw) => {
                let kind = NodeKind::TsModuleDecl(raw);
                self.push_children(kind, children);
            }
            Decl::Var(raw) => {
                let kind = NodeKind::VarDecl(raw);
                self.push_children(kind, children);
            }
            Decl::TsTypeAlias(raw) => {
                let kind = NodeKind::TsTypeAliasDecl(raw);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_ts_module_decl_children(self: &Rc<Self>, decl: &'m TsModuleDecl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match &decl.body {
            Some(TsNamespaceBody::TsModuleBlock(raw)) => self.get_ts_module_block_children(raw, children),
            Some(TsNamespaceBody::TsNamespaceDecl(raw)) => self.get_ts_namespace_decl_children(raw, children),
            _ => {}
        }
    }

    fn get_ts_namespace_decl_children(
        self: &Rc<Self>,
        decl: &'m TsNamespaceDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match &*decl.body {
            TsNamespaceBody::TsModuleBlock(raw) => self.get_ts_module_block_children(raw, children),
            TsNamespaceBody::TsNamespaceDecl(raw) => self.get_ts_namespace_decl_children(raw, children),
        }
    }

    fn get_ts_module_block_children(self: &Rc<Self>, block: &'m TsModuleBlock, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        block.body.iter().for_each(|item| {
            let kind = NodeKind::ModuleItem(item);
            self.push_children(kind, children);
        });
    }

    fn get_class_decl_children(self: &Rc<Self>, decl: &'m ClassDecl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::Class(&decl.class);
        self.push_children(kind, children);
        // decl.class.body.iter().for_each(|member| {
        //     let kind = NodeKind::ClassMember(member);
        //     self.push_children(kind, children);
        // });
    }
    fn get_fn_decl_children(self: &Rc<Self>, decl: &'m FnDecl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        decl.function.body.iter().for_each(|member| {
            let kind = NodeKind::BlockStmt(member);
            self.push_children(kind, children);
        });
    }
    fn get_ts_enum_decl_children(self: &Rc<Self>, decl: &'m TsEnumDecl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        decl.members.iter().for_each(|member| {
            let kind = NodeKind::TsEnumMember(member);
            self.push_children(kind, children);
        });
    }
    fn get_ts_interface_decl_children(
        self: &Rc<Self>,
        decl: &'m TsInterfaceDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for extend in &decl.extends {
            let kind = NodeKind::TsExprWithTypeArgs(&extend);
            self.push_children(kind, children);
        }

        if let Some(type_params) = &decl.type_params {
            for param in &type_params.params {
                let kind = NodeKind::TsTypeParam(&param);
                self.push_children(kind, children);
            }
        }

        let kind = NodeKind::TsInterfaceBody(&decl.body);
        self.push_children(kind, children);
    }

    fn get_ts_type_alias_declaration(
        self: &Rc<Self>,
        decl: &'m TsTypeAliasDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        let kind = NodeKind::TsType(&decl.type_ann);
        self.push_children(kind, children);
    }

    fn get_module_decl_children(self: &Rc<Self>, decl: &'m ModuleDecl, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match decl {
            ModuleDecl::Import(raw) => self.get_import_decl_children(raw, children),
            ModuleDecl::ExportDecl(raw) => self.get_export_declartion_children(raw, children),
            ModuleDecl::ExportNamed(raw) => self.get_named_export_children(raw, children),
            ModuleDecl::ExportDefaultDecl(raw) => self.get_export_default_decl_children(raw, children),
            ModuleDecl::ExportDefaultExpr(raw) => self.get_export_default_expr_children(raw, children),
            _ => {}
        }
    }

    fn get_named_export_children(
        self: &Rc<Self>,
        named_export: &'m NamedExport,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for specifier in &named_export.specifiers {
            let kind = NodeKind::ExportSpecifier(specifier);
            self.push_children(kind, children);
        }
    }

    fn get_ts_type_element_children(
        self: &Rc<Self>,
        type_element: &'m TsTypeElement,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match type_element {
            TsTypeElement::TsCallSignatureDecl(raw) => self.get_ts_call_signature_decl_children(raw, children),
            TsTypeElement::TsConstructSignatureDecl(raw) => {
                self.get_ts_construct_signature_decl_children(raw, children)
            }
            TsTypeElement::TsIndexSignature(raw) => self.get_ts_index_signature_children(raw, children),
            TsTypeElement::TsMethodSignature(raw) => self.get_ts_method_signature_children(raw, children),
            TsTypeElement::TsPropertySignature(raw) => self.get_ts_property_signature_children(raw, children),
            TsTypeElement::TsGetterSignature(raw) => self.get_ts_getter_signature_children(raw, children),
            _ => {}
        }
    }

    fn get_ts_call_signature_decl_children(
        self: &Rc<Self>,
        thing: &'m TsCallSignatureDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        {
            for param in &thing.params {
                let kind = NodeKind::TsFnParam(param);
                self.push_children(kind, children);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_construct_signature_decl_children(
        self: &Rc<Self>,
        thing: &'m TsConstructSignatureDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        {
            for param in &thing.params {
                let kind = NodeKind::TsFnParam(param);
                self.push_children(kind, children);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_index_signature_children(
        self: &Rc<Self>,
        thing: &'m TsIndexSignature,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        {
            for param in &thing.params {
                let kind = NodeKind::TsFnParam(param);
                self.push_children(kind, children);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_method_signature_children(
        self: &Rc<Self>,
        thing: &'m TsMethodSignature,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        {
            for param in &thing.params {
                let kind = NodeKind::TsFnParam(param);
                self.push_children(kind, children);
            }
        }

        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }
    fn get_ts_property_signature_children(
        self: &Rc<Self>,
        signature: &'m TsPropertySignature,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        {
            for param in &signature.params {
                let kind = NodeKind::TsFnParam(param);
                self.push_children(kind, children);
            }
        }

        if let Some(type_ann) = &signature.type_ann {
            let kind = NodeKind::TsTypeAnnotation(type_ann);
            self.push_children(kind, children);
        }
    }
    fn get_ts_getter_signature_children(
        self: &Rc<Self>,
        thing: &'m TsGetterSignature,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        if let Some(type_ann) = &thing.type_ann {
            self.get_type_annotation_children(type_ann, children);
        }
    }

    fn get_type_lit_children(self: &Rc<Self>, type_lit: &'m TsTypeLit, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        for member in &type_lit.members {
            let kind = NodeKind::TsTypeElement(member);
            self.push_children(kind, children);
        }
    }

    fn get_call_expr_children(self: &Rc<Self>, expr: &'m CallExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::Callee(&expr.callee);
        self.push_children(kind, children);

        expr.args.iter().for_each(|arg| {
            let kind = NodeKind::ExprOrSpread(arg);
            self.push_children(kind, children);
        });
    }

    fn get_expr_children(self: &Rc<Self>, expr: &'m Expr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match expr {
            Expr::This(raw) => {
                let kind = NodeKind::ThisExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Array(raw) => {
                let kind = NodeKind::ArrayLit(raw);
                self.push_children(kind, children);
            }
            Expr::Object(raw) => {
                let kind = NodeKind::ObjectLit(raw);
                self.push_children(kind, children);
            }
            Expr::Fn(raw) => {
                let kind = NodeKind::FnExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Unary(raw) => {
                let kind = NodeKind::UnaryExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Update(raw) => {
                let kind = NodeKind::UpdateExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Bin(raw) => {
                let kind = NodeKind::BinExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Assign(raw) => {
                let kind = NodeKind::AssignExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Member(raw) => {
                let kind = NodeKind::MemberExpr(raw);
                self.push_children(kind, children);
            }
            Expr::SuperProp(raw) => {
                let kind = NodeKind::SuperPropExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Cond(raw) => {
                let kind = NodeKind::CondExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Call(raw) => {
                let kind = NodeKind::CallExpr(raw);
                self.push_children(kind, children);
            }
            Expr::New(raw) => {
                let kind = NodeKind::NewExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Seq(raw) => {
                let kind = NodeKind::SeqExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Ident(raw) => {
                let kind = NodeKind::Ident(raw);
                self.push_children(kind, children);
            }
            Expr::Lit(raw) => {
                let kind = NodeKind::Lit(raw);
                self.push_children(kind, children);
            }
            Expr::Tpl(raw) => {
                let kind = NodeKind::TemplateLiteral(raw);
                self.push_children(kind, children);
            }
            Expr::TaggedTpl(raw) => {
                let kind = NodeKind::TaggedTpl(raw);
                self.push_children(kind, children);
            }
            Expr::Arrow(raw) => {
                let kind = NodeKind::ArrowExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Class(raw) => {
                let kind = NodeKind::ClassExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Yield(raw) => {
                let kind = NodeKind::YieldExpr(raw);
                self.push_children(kind, children);
            }
            Expr::MetaProp(raw) => {
                let kind = NodeKind::MetaPropExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Await(raw) => {
                let kind = NodeKind::AwaitExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Paren(raw) => {
                let kind = NodeKind::ParenExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsTypeAssertion(raw) => {
                let kind = NodeKind::TsTypeAssertionExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsConstAssertion(raw) => {
                let kind = NodeKind::TsConstAssertionExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsNonNull(raw) => {
                let kind = NodeKind::TsNonNullExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsAs(raw) => {
                let kind = NodeKind::TsAsExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsInstantiation(raw) => {
                let kind = NodeKind::TsInstantiationExpr(raw);
                self.push_children(kind, children);
            }
            Expr::TsSatisfies(raw) => {
                let kind = NodeKind::TsSatisfiesExpr(raw);
                self.push_children(kind, children);
            }
            Expr::PrivateName(raw) => {
                let kind = NodeKind::PrivateNameExpr(raw);
                self.push_children(kind, children);
            }
            Expr::OptChain(raw) => {
                let kind = NodeKind::OptChainExpr(raw);
                self.push_children(kind, children);
            }
            Expr::Invalid(raw) => {
                let kind = NodeKind::InvalidExpr(raw);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_arrow_expr_children(self: &Rc<Self>, expr: &'m ArrowExpr, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        let kind = NodeKind::BlockStmtOrExpr(&*expr.body);
        self.push_children(kind, children);

        expr.params.iter().for_each(|param| {
            let kind = NodeKind::Pat(param);
            self.push_children(kind, children);
        });
    }

    fn get_module_children(self: &Rc<Self>, module: &'m Module, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        for item in &module.body {
            let kind = NodeKind::ModuleItem(item);
            self.push_children(kind, children);
        }
    }

    fn get_export_declartion_children(
        self: &Rc<Self>,
        export_decl: &'m ExportDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        let kind = NodeKind::Decl(&export_decl.decl);
        self.push_children(kind, children);
    }

    fn get_export_default_expr_children(
        self: &Rc<Self>,
        expression: &'m ExportDefaultExpr,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        self.get_expr_children(&expression.expr, children)
    }

    fn get_export_default_decl_children(
        self: &Rc<Self>,
        export_declaration: &'m ExportDefaultDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match &export_declaration.decl {
            DefaultDecl::Class(declaration) => {
                let kind = NodeKind::ClassExpr(&declaration);

                self.push_children(kind, children);
            }
            DefaultDecl::TsInterfaceDecl(declaration) => {
                let kind = NodeKind::TsInterfaceDecl(&declaration);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_import_decl_children(
        self: &Rc<Self>,
        import_declaration: &'m ImportDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        for specifier in &import_declaration.specifiers {
            let kind = NodeKind::ImportSpecifier(&specifier);
            self.push_children(kind, children);
        }
    }

    fn get_module_item_children(self: &Rc<Self>, module_item: &'m ModuleItem, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match module_item {
            ModuleItem::ModuleDecl(declaration) => match declaration {
                ModuleDecl::Import(declaration) => {
                    let kind = NodeKind::ImportDecl(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::ExportDecl(declaration) => {
                    let kind = NodeKind::ExportDecl(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::ExportNamed(declaration) => {
                    let kind = NodeKind::NamedExport(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::ExportDefaultDecl(declaration) => {
                    let kind = NodeKind::ExportDefaultDecl(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::ExportDefaultExpr(declaration) => {
                    let kind = NodeKind::ExportDefaultExpr(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::ExportAll(declaration) => {
                    let kind = NodeKind::ExportAll(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::TsImportEquals(declaration) => {
                    let kind = NodeKind::TsImportEquals(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::TsExportAssignment(declaration) => {
                    let kind = NodeKind::TsExportAssignment(&declaration);
                    self.push_children(kind, children);
                }
                ModuleDecl::TsNamespaceExport(declaration) => {
                    let kind = NodeKind::TsNamespaceExport(&declaration);
                    self.push_children(kind, children);
                }
            },
            ModuleItem::Stmt(statement) => self.get_statement_children(statement, children),
        }
    }

    fn get_pat_children(self: &Rc<Self>, pat: &'m Pat, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match pat {
            Pat::Ident(ident) => {
                let kind = NodeKind::TsTypeAnnotation(ident.type_ann.as_ref().unwrap());
                self.push_children(kind, children);
            }
            Pat::Array(raw) => {
                let kind = NodeKind::ArrayPat(raw);
                self.push_children(kind, children);
            }
            Pat::Rest(raw) => {
                let kind = NodeKind::RestPat(raw);
                self.push_children(kind, children);
            }
            Pat::Object(raw) => {
                let kind = NodeKind::ObjectPat(raw);
                self.push_children(kind, children);
            }
            Pat::Assign(raw) => {
                let kind = NodeKind::AssignPat(raw);
                self.push_children(kind, children);
            }
            Pat::Expr(raw) => {
                let kind = NodeKind::Expr(raw);
                self.push_children(kind, children);
            }
            _ => {}
        }
    }

    fn get_statement_children(self: &Rc<Self>, statement: &'m Stmt, children: &mut Vec<Rc<SchemyNode<'m>>>) {
        match statement {
            Stmt::Block(block_stmt) => {
                let kind = NodeKind::BlockStmt(block_stmt);
                self.push_children(kind, children);
            }
            Stmt::Empty(empty_stmt) => {
                let kind = NodeKind::EmptyStmt(empty_stmt);
                self.push_children(kind, children);
            }
            Stmt::Debugger(debugger_stmt) => {
                let kind = NodeKind::DebuggerStmt(debugger_stmt);
                self.push_children(kind, children);
            }
            Stmt::With(with_stmt) => {
                let kind = NodeKind::WithStmt(with_stmt);
                self.push_children(kind, children);
            }
            Stmt::Return(return_stmt) => {
                let kind = NodeKind::ReturnStmt(return_stmt);
                self.push_children(kind, children);
            }
            Stmt::Labeled(labeled_stmt) => {
                let kind = NodeKind::LabeledStmt(labeled_stmt);
                self.push_children(kind, children);
            }
            Stmt::Break(break_stmt) => {
                let kind = NodeKind::BreakStmt(break_stmt);
                self.push_children(kind, children);
            }
            Stmt::Continue(continue_stmt) => {
                let kind = NodeKind::ContinueStmt(continue_stmt);
                self.push_children(kind, children);
            }
            Stmt::If(if_stmt) => {
                let kind = NodeKind::IfStmt(if_stmt);
                self.push_children(kind, children);
            }
            Stmt::Switch(switch_stmt) => {
                let kind = NodeKind::SwitchStmt(switch_stmt);
                self.push_children(kind, children);
            }
            Stmt::Throw(throw_stmt) => {
                let kind = NodeKind::ThrowStmt(throw_stmt);
                self.push_children(kind, children);
            }
            Stmt::Try(try_stmt) => {
                let kind = NodeKind::TryStmt(try_stmt);
                self.push_children(kind, children);
            }
            Stmt::While(while_stmt) => {
                let kind = NodeKind::WhileStmt(while_stmt);
                self.push_children(kind, children);
            }
            Stmt::DoWhile(do_while_stmt) => {
                let kind = NodeKind::DoWhileStmt(do_while_stmt);
                self.push_children(kind, children);
            }
            Stmt::For(for_stmt) => {
                let kind = NodeKind::ForStmt(for_stmt);
                self.push_children(kind, children);
            }
            Stmt::ForIn(for_in_stmt) => {
                let kind = NodeKind::ForInStmt(for_in_stmt);
                self.push_children(kind, children);
            }
            Stmt::ForOf(for_of_stmt) => {
                let kind = NodeKind::ForOfStmt(for_of_stmt);
                self.push_children(kind, children);
            }
            Stmt::Decl(decl_stmt) => {
                let kind = NodeKind::Decl(decl_stmt);
                self.push_children(kind, children);
            }
            Stmt::Expr(expr_stmt) => {
                let kind = NodeKind::ExprStmt(expr_stmt);
                self.push_children(kind, children);
            }
        }
    }

    fn get_block_statement_children(
        self: &Rc<Self>,
        block_stmnt: &'m BlockStmt,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        block_stmnt
            .stmts
            .iter()
            .for_each(|statement| self.get_statement_children(statement, children))
    }

    fn get_type_annotation_children(
        self: &Rc<Self>,
        type_annotation: &'m TsTypeAnn,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        match &*type_annotation.type_ann {
            TsType::TsKeywordType(ts_keyword_type) => {
                let kind = NodeKind::TsKeywordType(&ts_keyword_type);
                self.push_children(kind, children);
            }
            TsType::TsThisType(ts_this_type) => {
                let kind = NodeKind::TsThisType(&ts_this_type);
                self.push_children(kind, children);
            }
            TsType::TsFnOrConstructorType(ts_fn_or_constructor_type) => {
                let kind = NodeKind::TsFnOrConstructorType(&ts_fn_or_constructor_type);
                self.push_children(kind, children);
            }
            TsType::TsTypeRef(ts_type_ref) => {
                let kind = NodeKind::TsTypeRef(&ts_type_ref);
                self.push_children(kind, children);
            }
            TsType::TsTypeQuery(ts_type_query) => {
                let kind = NodeKind::TsTypeQuery(&ts_type_query);
                self.push_children(kind, children);
            }
            TsType::TsTypeLit(ts_type_lit) => {
                let kind = NodeKind::TsTypeLit(&ts_type_lit);
                self.push_children(kind, children);
            }
            TsType::TsArrayType(ts_array_type) => {
                let kind = NodeKind::TsArrayType(&ts_array_type);
                self.push_children(kind, children);
            }
            TsType::TsTupleType(ts_tuple_type) => {
                let kind = NodeKind::TsTupleType(&ts_tuple_type);
                self.push_children(kind, children);
            }
            TsType::TsOptionalType(ts_optional_type) => {
                let kind = NodeKind::TsOptionalType(&ts_optional_type);
                self.push_children(kind, children);
            }
            TsType::TsRestType(ts_rest_type) => {
                let kind = NodeKind::TsRestType(&ts_rest_type);
                self.push_children(kind, children);
            }
            TsType::TsUnionOrIntersectionType(ts_union_or_intersection_type) => {
                let kind = NodeKind::TsUnionOrIntersectionType(&ts_union_or_intersection_type);
                self.push_children(kind, children);
            }
            TsType::TsConditionalType(ts_conditional_type) => {
                let kind = NodeKind::TsConditionalType(&ts_conditional_type);
                self.push_children(kind, children);
            }
            TsType::TsInferType(ts_infer_type) => {
                let kind = NodeKind::TsInferType(&ts_infer_type);
                self.push_children(kind, children);
            }
            TsType::TsParenthesizedType(ts_parenthesized_type) => {
                let kind = NodeKind::TsParenthesizedType(&ts_parenthesized_type);
                self.push_children(kind, children);
            }
            TsType::TsTypeOperator(ts_type_operator) => {
                let kind = NodeKind::TsTypeOperator(&ts_type_operator);
                self.push_children(kind, children);
            }
            TsType::TsIndexedAccessType(ts_indexed_access_type) => {
                let kind = NodeKind::TsIndexedAccessType(&ts_indexed_access_type);
                self.push_children(kind, children);
            }
            TsType::TsMappedType(ts_mapped_type) => {
                let kind = NodeKind::TsMappedType(&ts_mapped_type);
                self.push_children(kind, children);
            }
            TsType::TsLitType(ts_lit_type) => {
                let kind = NodeKind::TsLitType(&ts_lit_type);
                self.push_children(kind, children);
            }
            TsType::TsTypePredicate(ts_type_predicate) => {
                let kind = NodeKind::TsTypePredicate(&ts_type_predicate);
                self.push_children(kind, children);
            }
            TsType::TsImportType(ts_import_type) => {
                let kind = NodeKind::TsImportType(&ts_import_type);
                self.push_children(kind, children);
            }
        }
    }

    fn get_var_decl_children(
        self: &Rc<Self>,
        variable_declaration: &'m VarDecl,
        children: &mut Vec<Rc<SchemyNode<'m>>>,
    ) {
        variable_declaration.decls.iter().for_each(|decl| {
            let kind = NodeKind::VarDeclarator(decl);
            self.push_children(kind, children);
        })
    }
}
