use std::fmt::Debug;
use swc_ecma_ast::*;

#[derive(PartialEq)]
pub enum NodeKind<'m> {
    TsExprWithTypeArgs(&'m TsExprWithTypeArgs),
    BindingIdent(&'m BindingIdent),
    Constructor(&'m Constructor),
    TsTypeParam(&'m TsTypeParam),
    TsEntityName(&'m TsEntityName),
    TsTypeParamDecl(&'m TsTypeParamDecl),
    MemberProp(&'m MemberProp),
    TsModuleBlock(&'m TsModuleBlock),
    TsNamespaceDecl(&'m TsNamespaceDecl),
    TsEnumMember(&'m TsEnumMember),
    TsFnParam(&'m TsFnParam),
    ArrayPat(&'m ArrayPat),
    RestPat(&'m RestPat),
    ObjectPat(&'m ObjectPat),
    AssignPat(&'m AssignPat),
    ThisExpr(&'m ThisExpr),
    ArrayLit(&'m ArrayLit),
    FnExpr(&'m FnExpr),
    UnaryExpr(&'m UnaryExpr),
    UpdateExpr(&'m UpdateExpr),
    BinExpr(&'m BinExpr),
    AssignExpr(&'m AssignExpr),
    MemberExpr(&'m MemberExpr),
    SuperPropExpr(&'m SuperPropExpr),
    CondExpr(&'m CondExpr),
    NewExpr(&'m NewExpr),
    SeqExpr(&'m SeqExpr),
    Lit(&'m Lit),
    Tpl(&'m Tpl),
    TaggedTpl(&'m TaggedTpl),
    YieldExpr(&'m YieldExpr),
    MetaPropExpr(&'m MetaPropExpr),
    AwaitExpr(&'m AwaitExpr),
    ParenExpr(&'m ParenExpr),
    TsTypeAssertionExpr(&'m TsTypeAssertion),
    TsConstAssertionExpr(&'m TsConstAssertion),
    TsNonNullExpr(&'m TsNonNullExpr),
    TsAsExpr(&'m TsAsExpr),
    TsInstantiationExpr(&'m TsInstantiation),
    TsSatisfiesExpr(&'m TsSatisfiesExpr),
    PrivateNameExpr(&'m PrivateName),
    OptChainExpr(&'m OptChainExpr),
    InvalidExpr(&'m Invalid),
    ArrowExpr(&'m ArrowExpr),
    BlockStmt(&'m BlockStmt),
    BlockStmtOrExpr(&'m BlockStmtOrExpr),
    BreakStmt(&'m BreakStmt),
    Callee(&'m Callee),
    CallExpr(&'m CallExpr),
    Class(&'m Class),
    ClassDecl(&'m ClassDecl),
    ClassExpr(&'m ClassExpr),
    ClassMember(&'m ClassMember),
    ClassProp(&'m ClassProp),
    ContinueStmt(&'m ContinueStmt),
    DebuggerStmt(&'m DebuggerStmt),
    Decl(&'m Decl),
    DoWhileStmt(&'m DoWhileStmt),
    EmptyStmt(&'m EmptyStmt),
    ExportAll(&'m ExportAll),
    ExportDecl(&'m ExportDecl),
    ExportDefaultDecl(&'m ExportDefaultDecl),
    ExportDefaultExpr(&'m ExportDefaultExpr),
    ExportSpecifier(&'m ExportSpecifier),
    Expr(&'m Expr),
    ExprOrSpread(&'m ExprOrSpread),
    ExprStmt(&'m ExprStmt),
    FnDecl(&'m FnDecl),
    ForInStmt(&'m ForInStmt),
    ForOfStmt(&'m ForOfStmt),
    ForStmt(&'m ForStmt),
    Ident(&'m Ident),
    IfStmt(&'m IfStmt),
    ImportDecl(&'m ImportDecl),
    ImportDefaultSpecifier(&'m ImportDefaultSpecifier),
    ImportNamedSpecifier(&'m ImportNamedSpecifier),
    ImportSpecifier(&'m ImportSpecifier),
    LabeledStmt(&'m LabeledStmt),
    Module(&'m Module),
    ModuleItem(&'m ModuleItem),
    NamedExport(&'m NamedExport),
    ObjectLit(&'m ObjectLit),
    Pat(&'m Pat),
    ReturnStmt(&'m ReturnStmt),
    Source(&'m Module),
    SwitchStmt(&'m SwitchStmt),
    ThrowStmt(&'m ThrowStmt),
    TryStmt(&'m TryStmt),
    TsArrayType(&'m TsArrayType),
    TsConditionalType(&'m TsConditionalType),
    TsEnumDecl(&'m TsEnumDecl),
    TsExportAssignment(&'m TsExportAssignment),
    TsFnOrConstructorType(&'m TsFnOrConstructorType),
    TsImportEquals(&'m TsImportEqualsDecl),
    TsImportType(&'m TsImportType),
    TsIndexedAccessType(&'m TsIndexedAccessType),
    TsInferType(&'m TsInferType),
    TsInterfaceBody(&'m TsInterfaceBody),
    TsInterfaceDecl(&'m TsInterfaceDecl),
    TsKeywordType(&'m TsKeywordType),
    TsLitType(&'m TsLitType),
    TsMappedType(&'m TsMappedType),
    TsModuleDecl(&'m TsModuleDecl),
    TsNamespaceExport(&'m TsNamespaceExportDecl),
    TsOptionalType(&'m TsOptionalType),
    TsParenthesizedType(&'m TsParenthesizedType),
    TsPropertySignature(&'m TsPropertySignature),
    TsRestType(&'m TsRestType),
    TsThisType(&'m TsThisType),
    TsTupleType(&'m TsTupleType),
    TsType(&'m TsType),
    TsTypeAliasDecl(&'m TsTypeAliasDecl),
    TsTypeAnnotation(&'m TsTypeAnn),
    TsTypeElement(&'m TsTypeElement),
    TsTypeLit(&'m TsTypeLit),
    TsTypeOperator(&'m TsTypeOperator),
    TsTypeParamInstantiation(&'m TsTypeParamInstantiation),
    TsTypePredicate(&'m TsTypePredicate),
    TsTypeQuery(&'m TsTypeQuery),
    TsTypeRef(&'m TsTypeRef),
    TsUnionOrIntersectionType(&'m TsUnionOrIntersectionType),
    VarDecl(&'m VarDecl),
    VarDeclarator(&'m VarDeclarator),
    WhileStmt(&'m WhileStmt),
    WithStmt(&'m WithStmt),
    TsQualifiedName(&'m TsQualifiedName),
    Str(&'m Str),
    Bool(&'m Bool),
    Null(&'m Null),
    Num(&'m Number),
    BigInt(&'m BigInt),
    Regex(&'m Regex),
    DefaultDecl(&'m DefaultDecl),
}

impl<'n> Clone for NodeKind<'n> {
    fn clone(&self) -> NodeKind<'n> {
        match self {
            NodeKind::ArrowExpr(raw) => NodeKind::ArrowExpr(*raw),
            NodeKind::BlockStmt(raw) => NodeKind::BlockStmt(*raw),
            NodeKind::BlockStmtOrExpr(raw) => NodeKind::BlockStmtOrExpr(*raw),
            NodeKind::BreakStmt(raw) => NodeKind::BreakStmt(*raw),
            NodeKind::Callee(raw) => NodeKind::Callee(*raw),
            NodeKind::CallExpr(raw) => NodeKind::CallExpr(*raw),
            NodeKind::Class(raw) => NodeKind::Class(raw),
            NodeKind::ClassDecl(raw) => NodeKind::ClassDecl(*raw),
            NodeKind::ClassExpr(raw) => NodeKind::ClassExpr(*raw),
            NodeKind::ClassMember(raw) => NodeKind::ClassMember(raw),
            NodeKind::ClassProp(raw) => NodeKind::ClassProp(raw),
            NodeKind::ContinueStmt(raw) => NodeKind::ContinueStmt(*raw),
            NodeKind::DebuggerStmt(raw) => NodeKind::DebuggerStmt(*raw),
            NodeKind::Decl(raw) => NodeKind::Decl(*raw),
            NodeKind::DoWhileStmt(raw) => NodeKind::DoWhileStmt(*raw),
            NodeKind::EmptyStmt(raw) => NodeKind::EmptyStmt(*raw),
            NodeKind::ExportAll(raw) => NodeKind::ExportAll(*raw),
            NodeKind::ExportDecl(raw) => NodeKind::ExportDecl(*raw),
            NodeKind::ExportDefaultDecl(raw) => NodeKind::ExportDefaultDecl(*raw),
            NodeKind::ExportDefaultExpr(raw) => NodeKind::ExportDefaultExpr(*raw),
            NodeKind::ExportSpecifier(raw) => NodeKind::ExportSpecifier(raw),
            NodeKind::Expr(raw) => NodeKind::Expr(*raw),
            NodeKind::ExprOrSpread(raw) => NodeKind::ExprOrSpread(*raw),
            NodeKind::ExprStmt(raw) => NodeKind::ExprStmt(*raw),
            NodeKind::FnDecl(raw) => NodeKind::FnDecl(*raw),
            NodeKind::ForInStmt(raw) => NodeKind::ForInStmt(*raw),
            NodeKind::ForOfStmt(raw) => NodeKind::ForOfStmt(*raw),
            NodeKind::ForStmt(raw) => NodeKind::ForStmt(*raw),
            NodeKind::Ident(raw) => NodeKind::Ident(*raw),
            NodeKind::IfStmt(raw) => NodeKind::IfStmt(*raw),
            NodeKind::ImportDecl(raw) => NodeKind::ImportDecl(*raw),
            NodeKind::ImportDefaultSpecifier(raw) => NodeKind::ImportDefaultSpecifier(*raw),
            NodeKind::ImportNamedSpecifier(raw) => NodeKind::ImportNamedSpecifier(*raw),
            NodeKind::ImportSpecifier(raw) => NodeKind::ImportSpecifier(*raw),
            NodeKind::LabeledStmt(raw) => NodeKind::LabeledStmt(*raw),
            NodeKind::Module(raw) => NodeKind::Module(*raw),
            NodeKind::ModuleItem(raw) => NodeKind::ModuleItem(*raw),
            NodeKind::NamedExport(raw) => NodeKind::NamedExport(*raw),
            NodeKind::ObjectLit(raw) => NodeKind::ObjectLit(*raw),
            NodeKind::Pat(raw) => NodeKind::Pat(*raw),
            NodeKind::ReturnStmt(raw) => NodeKind::ReturnStmt(*raw),
            NodeKind::Source(raw) => NodeKind::Source(*raw),
            NodeKind::SwitchStmt(raw) => NodeKind::SwitchStmt(*raw),
            NodeKind::ThrowStmt(raw) => NodeKind::ThrowStmt(*raw),
            NodeKind::TryStmt(raw) => NodeKind::TryStmt(*raw),
            NodeKind::TsArrayType(raw) => NodeKind::TsArrayType(*raw),
            NodeKind::TsConditionalType(raw) => NodeKind::TsConditionalType(*raw),
            NodeKind::TsEnumDecl(raw) => NodeKind::TsEnumDecl(*raw),
            NodeKind::TsExportAssignment(raw) => NodeKind::TsExportAssignment(*raw),
            NodeKind::TsFnOrConstructorType(raw) => NodeKind::TsFnOrConstructorType(*raw),
            NodeKind::TsImportEquals(raw) => NodeKind::TsImportEquals(*raw),
            NodeKind::TsImportType(raw) => NodeKind::TsImportType(*raw),
            NodeKind::TsIndexedAccessType(raw) => NodeKind::TsIndexedAccessType(*raw),
            NodeKind::TsInferType(raw) => NodeKind::TsInferType(*raw),
            NodeKind::TsInterfaceBody(raw) => NodeKind::TsInterfaceBody(raw),
            NodeKind::TsInterfaceDecl(raw) => NodeKind::TsInterfaceDecl(*raw),
            NodeKind::TsKeywordType(raw) => NodeKind::TsKeywordType(*raw),
            NodeKind::TsLitType(raw) => NodeKind::TsLitType(*raw),
            NodeKind::TsMappedType(raw) => NodeKind::TsMappedType(*raw),
            NodeKind::TsModuleDecl(raw) => NodeKind::TsModuleDecl(*raw),
            NodeKind::TsNamespaceExport(raw) => NodeKind::TsNamespaceExport(*raw),
            NodeKind::TsOptionalType(raw) => NodeKind::TsOptionalType(*raw),
            NodeKind::TsParenthesizedType(raw) => NodeKind::TsParenthesizedType(*raw),
            NodeKind::TsPropertySignature(raw) => NodeKind::TsPropertySignature(*raw),
            NodeKind::TsRestType(raw) => NodeKind::TsRestType(*raw),
            NodeKind::TsThisType(raw) => NodeKind::TsThisType(*raw),
            NodeKind::TsTupleType(raw) => NodeKind::TsTupleType(*raw),
            NodeKind::TsType(raw) => NodeKind::TsType(*raw),
            NodeKind::TsTypeAliasDecl(raw) => NodeKind::TsTypeAliasDecl(*raw),
            NodeKind::TsTypeAnnotation(raw) => NodeKind::TsTypeAnnotation(*raw),
            NodeKind::TsTypeElement(raw) => NodeKind::TsTypeElement(raw),
            NodeKind::TsTypeLit(raw) => NodeKind::TsTypeLit(*raw),
            NodeKind::TsTypeOperator(raw) => NodeKind::TsTypeOperator(*raw),
            NodeKind::TsTypeParamInstantiation(raw) => NodeKind::TsTypeParamInstantiation(raw),
            NodeKind::TsTypePredicate(raw) => NodeKind::TsTypePredicate(*raw),
            NodeKind::TsTypeQuery(raw) => NodeKind::TsTypeQuery(*raw),
            NodeKind::TsTypeRef(raw) => NodeKind::TsTypeRef(*raw),
            NodeKind::TsUnionOrIntersectionType(raw) => NodeKind::TsUnionOrIntersectionType(*raw),
            NodeKind::VarDecl(raw) => NodeKind::VarDecl(*raw),
            NodeKind::VarDeclarator(raw) => NodeKind::VarDeclarator(*raw),
            NodeKind::WhileStmt(raw) => NodeKind::WhileStmt(*raw),
            NodeKind::WithStmt(raw) => NodeKind::WithStmt(*raw),
            NodeKind::ThisExpr(raw) => NodeKind::ThisExpr(raw),
            NodeKind::ArrayLit(raw) => NodeKind::ArrayLit(raw),
            NodeKind::FnExpr(raw) => NodeKind::FnExpr(raw),
            NodeKind::UnaryExpr(raw) => NodeKind::UnaryExpr(raw),
            NodeKind::UpdateExpr(raw) => NodeKind::UpdateExpr(raw),
            NodeKind::BinExpr(raw) => NodeKind::BinExpr(raw),
            NodeKind::AssignExpr(raw) => NodeKind::AssignExpr(raw),
            NodeKind::MemberExpr(raw) => NodeKind::MemberExpr(raw),
            NodeKind::SuperPropExpr(raw) => NodeKind::SuperPropExpr(raw),
            NodeKind::CondExpr(raw) => NodeKind::CondExpr(raw),
            NodeKind::NewExpr(raw) => NodeKind::NewExpr(raw),
            NodeKind::SeqExpr(raw) => NodeKind::SeqExpr(raw),
            NodeKind::Lit(raw) => NodeKind::Lit(raw),
            NodeKind::Tpl(raw) => NodeKind::Tpl(raw),
            NodeKind::TaggedTpl(raw) => NodeKind::TaggedTpl(raw),
            NodeKind::YieldExpr(raw) => NodeKind::YieldExpr(raw),
            NodeKind::MetaPropExpr(raw) => NodeKind::MetaPropExpr(raw),
            NodeKind::AwaitExpr(raw) => NodeKind::AwaitExpr(raw),
            NodeKind::ParenExpr(raw) => NodeKind::ParenExpr(raw),
            NodeKind::TsTypeAssertionExpr(raw) => NodeKind::TsTypeAssertionExpr(raw),
            NodeKind::TsConstAssertionExpr(raw) => NodeKind::TsConstAssertionExpr(raw),
            NodeKind::TsNonNullExpr(raw) => NodeKind::TsNonNullExpr(raw),
            NodeKind::TsAsExpr(raw) => NodeKind::TsAsExpr(raw),
            NodeKind::TsInstantiationExpr(raw) => NodeKind::TsInstantiationExpr(raw),
            NodeKind::TsSatisfiesExpr(raw) => NodeKind::TsSatisfiesExpr(raw),
            NodeKind::PrivateNameExpr(raw) => NodeKind::PrivateNameExpr(raw),
            NodeKind::OptChainExpr(raw) => NodeKind::OptChainExpr(raw),
            NodeKind::InvalidExpr(raw) => NodeKind::InvalidExpr(raw),
            NodeKind::ArrayPat(raw) => NodeKind::ArrayPat(raw),
            NodeKind::RestPat(raw) => NodeKind::RestPat(raw),
            NodeKind::ObjectPat(raw) => NodeKind::ObjectPat(raw),
            NodeKind::AssignPat(raw) => NodeKind::AssignPat(raw),
            NodeKind::TsFnParam(raw) => NodeKind::TsFnParam(raw),
            NodeKind::TsEnumMember(raw) => NodeKind::TsEnumMember(raw),
            NodeKind::TsModuleBlock(raw) => NodeKind::TsModuleBlock(raw),
            NodeKind::TsNamespaceDecl(raw) => NodeKind::TsNamespaceDecl(raw),
            NodeKind::MemberProp(raw) => NodeKind::MemberProp(raw),
            NodeKind::TsTypeParamDecl(raw) => NodeKind::TsTypeParamDecl(raw),
            NodeKind::TsEntityName(raw) => NodeKind::TsEntityName(raw),
            NodeKind::TsQualifiedName(raw) => NodeKind::TsQualifiedName(raw),
            NodeKind::Str(raw) => NodeKind::Str(raw),
            NodeKind::Bool(raw) => NodeKind::Bool(raw),
            NodeKind::Null(raw) => NodeKind::Null(raw),
            NodeKind::Num(raw) => NodeKind::Num(raw),
            NodeKind::BigInt(raw) => NodeKind::BigInt(raw),
            NodeKind::Regex(raw) => NodeKind::Regex(raw),
            NodeKind::TsTypeParam(raw) => NodeKind::TsTypeParam(raw),
            NodeKind::DefaultDecl(raw) => NodeKind::DefaultDecl(raw),
            NodeKind::Constructor(raw) => NodeKind::Constructor(raw),
            NodeKind::BindingIdent(raw) => NodeKind::BindingIdent(raw),
            NodeKind::TsExprWithTypeArgs(raw) => NodeKind::TsExprWithTypeArgs(raw),
        }
    }
}

impl<'m> Debug for NodeKind<'m> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeKind::ArrowExpr(_) => f.debug_tuple("ArrowExpr").finish(),
            NodeKind::BlockStmt(_) => f.debug_tuple("BlockStmt").finish(),
            NodeKind::BlockStmtOrExpr(_) => f.debug_tuple("BlockStmtOrExpr").finish(),
            NodeKind::BreakStmt(_) => f.debug_tuple("BreakStmt").finish(),
            NodeKind::Callee(_) => f.debug_tuple("Callee").finish(),
            NodeKind::CallExpr(_) => f.debug_tuple("CallExpr").finish(),
            NodeKind::Class(_) => f.debug_tuple("Class").finish(),
            NodeKind::ClassDecl(_) => f.debug_tuple("ClassDecl").finish(),
            NodeKind::ClassExpr(_) => f.debug_tuple("ClassExpr").finish(),
            NodeKind::ClassMember(_) => f.debug_tuple("ClassMember").finish(),
            NodeKind::ClassProp(_) => f.debug_tuple("ClassProp").finish(),
            NodeKind::ContinueStmt(_) => f.debug_tuple("ContinueStmt").finish(),
            NodeKind::DebuggerStmt(_) => f.debug_tuple("DebuggerStmt").finish(),
            NodeKind::Decl(_) => f.debug_tuple("Decl").finish(),
            NodeKind::DoWhileStmt(_) => f.debug_tuple("DoWhileStmt").finish(),
            NodeKind::EmptyStmt(_) => f.debug_tuple("EmptyStmt").finish(),
            NodeKind::ExportAll(_) => f.debug_tuple("ExportAll").finish(),
            NodeKind::ExportDecl(_) => f.debug_tuple("ExportDecl").finish(),
            NodeKind::ExportDefaultDecl(_) => f.debug_tuple("ExportDefaultDecl").finish(),
            NodeKind::ExportDefaultExpr(_) => f.debug_tuple("ExportDefaultExpr").finish(),
            NodeKind::ExportSpecifier(_) => f.debug_tuple("ExportSpecifier").finish(),
            NodeKind::Expr(_) => f.debug_tuple("Expr").finish(),
            NodeKind::ExprOrSpread(_) => f.debug_tuple("ExprOrSpread").finish(),
            NodeKind::ExprStmt(_) => f.debug_tuple("ExprStmt").finish(),
            NodeKind::FnDecl(_) => f.debug_tuple("FnDecl").finish(),
            NodeKind::ForInStmt(_) => f.debug_tuple("ForInStmt").finish(),
            NodeKind::ForOfStmt(_) => f.debug_tuple("ForOfStmt").finish(),
            NodeKind::ForStmt(_) => f.debug_tuple("ForStmt").finish(),
            NodeKind::Ident(_) => f.debug_tuple("Ident").finish(),
            NodeKind::IfStmt(_) => f.debug_tuple("IfStmt").finish(),
            NodeKind::ImportDecl(_) => f.debug_tuple("ImportDecl").finish(),
            NodeKind::ImportDefaultSpecifier(_) => f.debug_tuple("ImportDefaultSpecifier").finish(),
            NodeKind::ImportNamedSpecifier(_) => f.debug_tuple("ImportNamedSpecifier").finish(),
            NodeKind::ImportSpecifier(_) => f.debug_tuple("ImportSpecifier").finish(),
            NodeKind::LabeledStmt(_) => f.debug_tuple("LabeledStmt").finish(),
            NodeKind::Module(_) => f.debug_tuple("Module").finish(),
            NodeKind::ModuleItem(_) => f.debug_tuple("ModuleItem").finish(),
            NodeKind::NamedExport(_) => f.debug_tuple("NamedExport").finish(),
            NodeKind::ObjectLit(_) => f.debug_tuple("ObjectLit").finish(),
            NodeKind::Pat(_) => f.debug_tuple("Pat").finish(),
            NodeKind::ReturnStmt(_) => f.debug_tuple("ReturnStmt").finish(),
            NodeKind::Source(_) => f.debug_tuple("Source").finish(),
            NodeKind::SwitchStmt(_) => f.debug_tuple("SwitchStmt").finish(),
            NodeKind::ThrowStmt(_) => f.debug_tuple("ThrowStmt").finish(),
            NodeKind::TryStmt(_) => f.debug_tuple("TryStmt").finish(),
            NodeKind::TsArrayType(_) => f.debug_tuple("TsArrayType").finish(),
            NodeKind::TsConditionalType(_) => f.debug_tuple("TsConditionalType").finish(),
            NodeKind::TsEnumDecl(_) => f.debug_tuple("TsEnumDecl").finish(),
            NodeKind::TsExportAssignment(_) => f.debug_tuple("TsExportAssignment").finish(),
            NodeKind::TsFnOrConstructorType(_) => f.debug_tuple("TsFnOrConstructorType").finish(),
            NodeKind::TsImportEquals(_) => f.debug_tuple("TsImportEquals").finish(),
            NodeKind::TsImportType(_) => f.debug_tuple("TsImportType").finish(),
            NodeKind::TsIndexedAccessType(_) => f.debug_tuple("TsIndexedAccessType").finish(),
            NodeKind::TsInferType(_) => f.debug_tuple("TsInferType").finish(),
            NodeKind::TsInterfaceBody(_) => f.debug_tuple("TsInterfaceBody").finish(),
            NodeKind::TsInterfaceDecl(_) => f.debug_tuple("TsInterfaceDecl").finish(),
            NodeKind::TsKeywordType(_) => f.debug_tuple("TsKeywordType").finish(),
            NodeKind::TsLitType(_) => f.debug_tuple("TsLitType").finish(),
            NodeKind::TsMappedType(_) => f.debug_tuple("TsMappedType").finish(),
            NodeKind::TsModuleDecl(_) => f.debug_tuple("TsModuleDecl").finish(),
            NodeKind::TsNamespaceExport(_) => f.debug_tuple("TsNamespaceExport").finish(),
            NodeKind::TsOptionalType(_) => f.debug_tuple("TsOptionalType").finish(),
            NodeKind::TsParenthesizedType(_) => f.debug_tuple("TsParenthesizedType").finish(),
            NodeKind::TsPropertySignature(_) => f.debug_tuple("TsPropertySignature").finish(),
            NodeKind::TsRestType(_) => f.debug_tuple("TsRestType").finish(),
            NodeKind::TsThisType(_) => f.debug_tuple("TsThisType").finish(),
            NodeKind::TsTupleType(_) => f.debug_tuple("TsTupleType").finish(),
            NodeKind::TsType(_) => f.debug_tuple("TsType").finish(),
            NodeKind::TsTypeAliasDecl(_) => f.debug_tuple("TsTypeAliasDecl").finish(),
            NodeKind::TsTypeAnnotation(_) => f.debug_tuple("TsTypeAnnotation").finish(),
            NodeKind::TsTypeElement(_) => f.debug_tuple("TsTypeElement").finish(),
            NodeKind::TsTypeLit(_) => f.debug_tuple("TsTypeLit").finish(),
            NodeKind::TsTypeOperator(_) => f.debug_tuple("TsTypeOperator").finish(),
            NodeKind::TsTypeParamInstantiation(_) => f.debug_tuple("TsTypeParamInstantiation").finish(),
            NodeKind::TsTypePredicate(_) => f.debug_tuple("TsTypePredicate").finish(),
            NodeKind::TsTypeQuery(_) => f.debug_tuple("TsTypeQuery").finish(),
            NodeKind::TsTypeRef(_) => f.debug_tuple("TsTypeRef").finish(),
            NodeKind::TsUnionOrIntersectionType(_) => f.debug_tuple("TsUnionOrIntersectionType").finish(),
            NodeKind::VarDecl(_) => f.debug_tuple("VarDecl").finish(),
            NodeKind::VarDeclarator(_) => f.debug_tuple("VarDeclarator").finish(),
            NodeKind::WhileStmt(_) => f.debug_tuple("WhileStmt").finish(),
            NodeKind::WithStmt(_) => f.debug_tuple("WithStmt").finish(),
            NodeKind::ThisExpr(_) => f.debug_tuple("ThisExpr").finish(),
            NodeKind::ArrayLit(_) => f.debug_tuple("ArrayLit").finish(),
            NodeKind::FnExpr(_) => f.debug_tuple("FnExpr").finish(),
            NodeKind::UnaryExpr(_) => f.debug_tuple("UnaryExpr").finish(),
            NodeKind::UpdateExpr(_) => f.debug_tuple("UpdateExpr").finish(),
            NodeKind::BinExpr(_) => f.debug_tuple("BinExpr").finish(),
            NodeKind::AssignExpr(_) => f.debug_tuple("AssignExpr").finish(),
            NodeKind::MemberExpr(_) => f.debug_tuple("MemberExpr").finish(),
            NodeKind::SuperPropExpr(_) => f.debug_tuple("SuperPropExpr").finish(),
            NodeKind::CondExpr(_) => f.debug_tuple("CondExpr").finish(),
            NodeKind::NewExpr(_) => f.debug_tuple("NewExpr").finish(),
            NodeKind::SeqExpr(_) => f.debug_tuple("SeqExpr").finish(),
            NodeKind::Lit(_) => f.debug_tuple("Lit").finish(),
            NodeKind::Tpl(_) => f.debug_tuple("Tpl").finish(),
            NodeKind::TaggedTpl(_) => f.debug_tuple("TaggedTpl").finish(),
            NodeKind::YieldExpr(_) => f.debug_tuple("YieldExpr").finish(),
            NodeKind::MetaPropExpr(_) => f.debug_tuple("MetaPropExpr").finish(),
            NodeKind::AwaitExpr(_) => f.debug_tuple("AwaitExpr").finish(),
            NodeKind::ParenExpr(_) => f.debug_tuple("ParenExpr").finish(),
            NodeKind::TsTypeAssertionExpr(_) => f.debug_tuple("TsTypeAssertionExpr").finish(),
            NodeKind::TsConstAssertionExpr(_) => f.debug_tuple("TsConstAssertionExpr").finish(),
            NodeKind::TsNonNullExpr(_) => f.debug_tuple("TsNonNullExpr").finish(),
            NodeKind::TsAsExpr(_) => f.debug_tuple("TsAsExpr").finish(),
            NodeKind::TsInstantiationExpr(_) => f.debug_tuple("TsInstantiationExpr").finish(),
            NodeKind::TsSatisfiesExpr(_) => f.debug_tuple("TsSatisfiesExpr").finish(),
            NodeKind::PrivateNameExpr(_) => f.debug_tuple("PrivateNameExpr").finish(),
            NodeKind::OptChainExpr(_) => f.debug_tuple("OptChainExpr").finish(),
            NodeKind::InvalidExpr(_) => f.debug_tuple("InvalidExpr").finish(),
            NodeKind::ArrayPat(_) => f.debug_tuple("ArrayPat").finish(),
            NodeKind::RestPat(_) => f.debug_tuple("RestPat").finish(),
            NodeKind::ObjectPat(_) => f.debug_tuple("ObjectPat").finish(),
            NodeKind::AssignPat(_) => f.debug_tuple("AssignPat").finish(),
            NodeKind::TsFnParam(_) => f.debug_tuple("TsFnParam").finish(),
            NodeKind::TsEnumMember(_) => f.debug_tuple("TsEnumMember").finish(),
            NodeKind::TsModuleBlock(_) => f.debug_tuple("TsModuleBlock").finish(),
            NodeKind::TsNamespaceDecl(_) => f.debug_tuple("TsNamespaceDecl").finish(),
            NodeKind::MemberProp(_) => f.debug_tuple("MemberProp").finish(),
            NodeKind::TsTypeParamDecl(_) => f.debug_tuple("TsTypeParamDecl").finish(),
            NodeKind::TsEntityName(_) => f.debug_tuple("TsEntityName").finish(),
            NodeKind::TsQualifiedName(_) => f.debug_tuple("TsQualifiedName").finish(),
            NodeKind::Str(_) => f.debug_tuple("Str").finish(),
            NodeKind::Bool(_) => f.debug_tuple("Bool").finish(),
            NodeKind::Null(_) => f.debug_tuple("Null").finish(),
            NodeKind::Num(_) => f.debug_tuple("Num").finish(),
            NodeKind::BigInt(_) => f.debug_tuple("BigInt").finish(),
            NodeKind::Regex(_) => f.debug_tuple("Regex").finish(),
            NodeKind::TsTypeParam(_) => f.debug_tuple("TsTypeParam").finish(),
            NodeKind::DefaultDecl(_) => f.debug_tuple("DefaultDecl").finish(),
            NodeKind::Constructor(_) => f.debug_tuple("Constructor").finish(),
            NodeKind::BindingIdent(_) => f.debug_tuple("BindingIdent").finish(),
            NodeKind::TsExprWithTypeArgs(_) => f.debug_tuple("TsExprWithTypeArgs").finish()
        }
    }
}
