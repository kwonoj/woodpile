use swc_core::{
    common::pass::AstKindPath,
    ecma::{
        ast::{
            ArrayLit, ArrayPat, ArrowExpr, AssignPat, AssignPatProp, AssignProp, AwaitExpr,
            BinExpr, BindingIdent, BlockStmt, BreakStmt, CallExpr, Callee, CatchClause, Class,
            ClassDecl, ClassExpr, ClassMember, ClassMethod, ClassProp, ComputedPropName, CondExpr,
            Constructor, ContinueStmt, DebuggerStmt, Decl, Decorator, DefaultDecl, DoWhileStmt,
            EmptyStmt, ExportAll, ExportDecl, ExportDefaultDecl, ExportDefaultExpr,
            ExportDefaultSpecifier, ExportNamedSpecifier, ExportNamespaceSpecifier,
            ExportSpecifier, Expr, ExprOrSpread, ExprStmt, FnDecl, FnExpr, ForInStmt, ForOfStmt,
            ForStmt, Function, GetterProp, Ident, IfStmt, ImportDecl, ImportSpecifier, JSXAttr,
            JSXAttrName, JSXAttrOrSpread, JSXAttrValue, JSXClosingElement, JSXClosingFragment,
            JSXElement, JSXElementChild, JSXElementName, JSXEmptyExpr, JSXExprContainer,
            JSXFragment, JSXMemberExpr, JSXNamespacedName, JSXObject, JSXOpeningElement,
            JSXOpeningFragment, JSXSpreadChild, JSXText, KeyValuePatProp, KeyValueProp,
            LabeledStmt, Lit, MemberExpr, MetaPropExpr, MethodProp, Module, ModuleDecl,
            ModuleExportName, ModuleItem, NamedExport, NewExpr, ObjectLit, ObjectPat,
            ObjectPatProp, OptChainExpr, Param, ParenExpr, Pat, PatOrExpr, PrivateMethod,
            PrivateName, PrivateProp, Program, Prop, PropName, RestPat, ReturnStmt, Script,
            SeqExpr, SetterProp, SpreadElement, StaticBlock, Stmt, Str, SuperPropExpr, SwitchCase,
            SwitchStmt, TaggedTpl, ThisExpr, ThrowStmt, Tpl, TryStmt, UnaryExpr, UpdateExpr,
            VarDecl, VarDeclarator, WhileStmt, WithStmt, YieldExpr,
        },
        visit::{
            noop_visit_mut_type, AstParentKind, VisitMut, VisitMutAstPath, VisitMutWith,
            VisitMutWithPath,
        },
    },
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use woodpile_dts::typed_visit;

use crate::{
    write_visit_mut, write_visit_mut_path, write_visit_mut_path_plural, write_visit_mut_plural,
};

/// A visitor for visit callbacks without path support.
pub struct BaseVisitor {
    visitor_context: JsValue,
    visitor: JsValue,
}

impl BaseVisitor {
    pub fn new(context: JsValue, visitor: JsValue) -> Self {
        Self {
            visitor_context: context,
            visitor,
        }
    }
}

/// A visitor for visit callbacks with path support.
pub struct PathVisitor {
    visitor_context: JsValue,
    visitor: JsValue,
}

impl PathVisitor {
    pub fn new(context: JsValue, visitor: JsValue) -> Self {
        Self {
            visitor_context: context,
            visitor,
        }
    }
}

impl VisitMut for BaseVisitor {
    noop_visit_mut_type!();

    write_visit_mut!(Program);
    write_visit_mut_plural!(ModuleItem);
    write_visit_mut!(Module);
    write_visit_mut!(Script);
    write_visit_mut!(ModuleItem);
    write_visit_mut!(ModuleDecl);
    write_visit_mut!(ExportAll);
    write_visit_mut!(ExportDefaultDecl);
    write_visit_mut!(ExportDefaultExpr);
    write_visit_mut!(ExportSpecifier);
    write_visit_mut_plural!(ExportSpecifier);
    write_visit_mut!(ExportNamedSpecifier);
    write_visit_mut!(NamedExport);
    write_visit_mut!(ModuleExportName);
    write_visit_mut!(ExportNamespaceSpecifier);
    write_visit_mut!(ExportDefaultSpecifier);
    write_visit_mut!(Str);
    write_visit_mut!(DefaultDecl);
    write_visit_mut!(FnExpr);
    write_visit_mut!(ExportDecl);
    write_visit_mut!(ArrayLit);
    write_visit_mut!(ExprOrSpread);
    write_visit_mut!(SpreadElement);
    write_visit_mut!(Expr);
    write_visit_mut!(ArrowExpr);
    write_visit_mut!(BlockStmt);
    write_visit_mut!(Stmt);
    write_visit_mut_plural!(Stmt);
    write_visit_mut!(SwitchStmt);
    write_visit_mut!(SwitchCase);
    write_visit_mut_plural!(SwitchCase);
    write_visit_mut!(IfStmt);
    write_visit_mut!(ObjectPat);
    write_visit_mut!(ObjectPatProp);
    write_visit_mut_plural!(ObjectPatProp);
    write_visit_mut!(ArrayPat);
    write_visit_mut!(Pat);
    write_visit_mut!(ImportDecl);
    write_visit_mut!(ImportSpecifier);
    write_visit_mut!(BreakStmt);
    write_visit_mut!(WhileStmt);
    write_visit_mut!(TryStmt);
    write_visit_mut!(CatchClause);
    write_visit_mut!(ThrowStmt);
    write_visit_mut!(ReturnStmt);
    write_visit_mut!(LabeledStmt);
    write_visit_mut!(ForStmt);
    write_visit_mut!(ForOfStmt);
    write_visit_mut!(ForInStmt);
    write_visit_mut!(EmptyStmt);
    write_visit_mut!(DoWhileStmt);
    write_visit_mut!(DebuggerStmt);
    write_visit_mut!(WithStmt);
    write_visit_mut!(Decl);
    write_visit_mut!(VarDecl);
    write_visit_mut!(VarDeclarator);
    write_visit_mut_plural!(VarDeclarator);
    write_visit_mut!(FnDecl);
    write_visit_mut!(Class);
    write_visit_mut!(ClassDecl);
    write_visit_mut!(ClassExpr);
    write_visit_mut!(ClassProp);
    write_visit_mut!(ClassMethod);
    write_visit_mut!(ClassMember);
    write_visit_mut_plural!(ClassMember);
    write_visit_mut!(PrivateProp);
    write_visit_mut!(PrivateMethod);
    write_visit_mut!(PrivateName);
    write_visit_mut!(Constructor);
    write_visit_mut!(StaticBlock);
    write_visit_mut!(PropName);
    write_visit_mut!(ComputedPropName);
    write_visit_mut!(Function);
    write_visit_mut!(Decorator);
    write_visit_mut_plural!(Decorator);
    write_visit_mut!(ExprStmt);
    write_visit_mut!(ContinueStmt);
    write_visit_mut!(OptChainExpr);
    write_visit_mut!(PatOrExpr);
    write_visit_mut!(YieldExpr);
    write_visit_mut!(UpdateExpr);
    write_visit_mut!(UnaryExpr);
    write_visit_mut!(ThisExpr);
    write_visit_mut!(Tpl);
    write_visit_mut!(TaggedTpl);
    write_visit_mut!(Param);
    write_visit_mut_plural!(Param);
    write_visit_mut!(SeqExpr);
    write_visit_mut!(Lit);
    write_visit_mut!(ParenExpr);
    write_visit_mut!(ObjectLit);
    write_visit_mut!(Prop);
    write_visit_mut!(SetterProp);
    write_visit_mut!(MethodProp);
    write_visit_mut!(KeyValueProp);
    write_visit_mut!(GetterProp);
    write_visit_mut!(AssignProp);
    write_visit_mut!(NewExpr);
    write_visit_mut!(MetaPropExpr);
    write_visit_mut!(MemberExpr);
    write_visit_mut!(SuperPropExpr);
    write_visit_mut!(Callee);
    write_visit_mut!(JSX, Text);
    write_visit_mut!(JSX, NamespacedName);
    write_visit_mut!(JSX, MemberExpr);
    write_visit_mut!(JSX, Object);
    write_visit_mut!(JSX, Fragment);
    write_visit_mut!(JSX, ClosingFragment);
    write_visit_mut!(JSX, ElementChild);
    write_visit_mut!(JSX, ExprContainer);
    write_visit_mut!(JSX, SpreadChild);
    write_visit_mut!(JSX, OpeningFragment);
    write_visit_mut!(JSX, EmptyExpr);
    write_visit_mut!(JSX, Element);
    write_visit_mut!(JSX, ClosingElement);
    write_visit_mut!(JSX, ElementName);
    write_visit_mut!(JSX, OpeningElement);
    write_visit_mut!(JSX, Attr);
    write_visit_mut!(JSX, AttrOrSpread);
    write_visit_mut!(JSX, AttrValue);
    write_visit_mut!(JSX, AttrName);
    write_visit_mut!(CondExpr);
    write_visit_mut!(CallExpr);
    write_visit_mut!(BinExpr);
    write_visit_mut!(AwaitExpr);
    write_visit_mut!(BindingIdent);
    write_visit_mut!(Ident);
    write_visit_mut!(RestPat);
    write_visit_mut!(AssignPatProp);
    write_visit_mut!(AssignPat);
    write_visit_mut!(KeyValuePatProp);
}

impl VisitMutAstPath for PathVisitor {
    write_visit_mut_path!(Program);
    write_visit_mut_path!(Module);
    write_visit_mut_path!(Script);
    write_visit_mut_path!(ModuleItem);
    write_visit_mut_path_plural!(ModuleItem);
    write_visit_mut_path!(ModuleDecl);
    write_visit_mut_path!(ExportAll);
    write_visit_mut_path!(ExportDefaultDecl);
    write_visit_mut_path!(ExportDefaultExpr);
    write_visit_mut_path!(ExportSpecifier);
    write_visit_mut_path_plural!(ExportSpecifier);
    write_visit_mut_path!(ExportNamedSpecifier);
    write_visit_mut_path!(NamedExport);
    write_visit_mut_path!(ModuleExportName);
    write_visit_mut_path!(ExportNamespaceSpecifier);
    write_visit_mut_path!(ExportDefaultSpecifier);
    write_visit_mut_path!(Str);
    write_visit_mut_path!(DefaultDecl);
    write_visit_mut_path!(FnExpr);
    write_visit_mut_path!(ExportDecl);
    write_visit_mut_path!(ArrayLit);
    write_visit_mut_path!(ExprOrSpread);
    write_visit_mut_path!(SpreadElement);
    write_visit_mut_path!(Expr);
    write_visit_mut_path!(ArrowExpr);
    write_visit_mut_path!(BlockStmt);
    write_visit_mut_path!(Stmt);
    write_visit_mut_path_plural!(Stmt);
    write_visit_mut_path!(SwitchStmt);
    write_visit_mut_path!(SwitchCase);
    write_visit_mut_path_plural!(SwitchCase);
    write_visit_mut_path!(IfStmt);
    write_visit_mut_path!(ObjectPat);
    write_visit_mut_path!(ObjectPatProp);
    write_visit_mut_path_plural!(ObjectPatProp);
    write_visit_mut_path!(ArrayPat);
    write_visit_mut_path!(Pat);
    write_visit_mut_path!(ImportDecl);
    write_visit_mut_path!(ImportSpecifier);
    write_visit_mut_path!(BreakStmt);
    write_visit_mut_path!(WhileStmt);
    write_visit_mut_path!(TryStmt);
    write_visit_mut_path!(CatchClause);
    write_visit_mut_path!(ThrowStmt);
    write_visit_mut_path!(ReturnStmt);
    write_visit_mut_path!(LabeledStmt);
    write_visit_mut_path!(ForStmt);
    write_visit_mut_path!(ForOfStmt);
    write_visit_mut_path!(ForInStmt);
    write_visit_mut_path!(EmptyStmt);
    write_visit_mut_path!(DoWhileStmt);
    write_visit_mut_path!(DebuggerStmt);
    write_visit_mut_path!(WithStmt);
    write_visit_mut_path!(Decl);
    write_visit_mut_path!(VarDecl);
    write_visit_mut_path!(VarDeclarator);
    write_visit_mut_path_plural!(VarDeclarator);
    write_visit_mut_path!(FnDecl);
    write_visit_mut_path!(Class);
    write_visit_mut_path!(ClassDecl);
    write_visit_mut_path!(ClassExpr);
    write_visit_mut_path!(ClassProp);
    write_visit_mut_path!(ClassMethod);
    write_visit_mut_path!(ClassMember);
    write_visit_mut_path_plural!(ClassMember);
    write_visit_mut_path!(PrivateProp);
    write_visit_mut_path!(PrivateMethod);
    write_visit_mut_path!(PrivateName);
    write_visit_mut_path!(Constructor);
    write_visit_mut_path!(StaticBlock);
    write_visit_mut_path!(PropName);
    write_visit_mut_path!(ComputedPropName);
    write_visit_mut_path!(Function);
    write_visit_mut_path!(Decorator);
    write_visit_mut_path_plural!(Decorator);
    write_visit_mut_path!(ExprStmt);
    write_visit_mut_path!(ContinueStmt);
    write_visit_mut_path!(OptChainExpr);
    write_visit_mut_path!(PatOrExpr);
    write_visit_mut_path!(YieldExpr);
    write_visit_mut_path!(UpdateExpr);
    write_visit_mut_path!(UnaryExpr);
    write_visit_mut_path!(ThisExpr);
    write_visit_mut_path!(Tpl);
    write_visit_mut_path!(TaggedTpl);
    write_visit_mut_path!(Param);
    write_visit_mut_path_plural!(Param);
    write_visit_mut_path!(SeqExpr);
    write_visit_mut_path!(Lit);
    write_visit_mut_path!(ParenExpr);
    write_visit_mut_path!(ObjectLit);
    write_visit_mut_path!(Prop);
    write_visit_mut_path!(SetterProp);
    write_visit_mut_path!(MethodProp);
    write_visit_mut_path!(KeyValueProp);
    write_visit_mut_path!(GetterProp);
    write_visit_mut_path!(AssignProp);
    write_visit_mut_path!(NewExpr);
    write_visit_mut_path!(MetaPropExpr);
    write_visit_mut_path!(MemberExpr);
    write_visit_mut_path!(SuperPropExpr);
    write_visit_mut_path!(Callee);
    write_visit_mut_path!(JSX, Text);
    write_visit_mut_path!(JSX, NamespacedName);
    write_visit_mut_path!(JSX, MemberExpr);
    write_visit_mut_path!(JSX, Object);
    write_visit_mut_path!(JSX, Fragment);
    write_visit_mut_path!(JSX, ClosingFragment);
    write_visit_mut_path!(JSX, ElementChild);
    write_visit_mut_path!(JSX, ExprContainer);
    write_visit_mut_path!(JSX, SpreadChild);
    write_visit_mut_path!(JSX, OpeningFragment);
    write_visit_mut_path!(JSX, EmptyExpr);
    write_visit_mut_path!(JSX, Element);
    write_visit_mut_path!(JSX, ClosingElement);
    write_visit_mut_path!(JSX, ElementName);
    write_visit_mut_path!(JSX, OpeningElement);
    write_visit_mut_path!(JSX, Attr);
    write_visit_mut_path!(JSX, AttrOrSpread);
    write_visit_mut_path!(JSX, AttrValue);
    write_visit_mut_path!(JSX, AttrName);
    write_visit_mut_path!(CondExpr);
    write_visit_mut_path!(CallExpr);
    write_visit_mut_path!(BinExpr);
    write_visit_mut_path!(AwaitExpr);
    write_visit_mut_path!(BindingIdent);
    write_visit_mut_path!(Ident);
    write_visit_mut_path!(RestPat);
    write_visit_mut_path!(AssignPatProp);
    write_visit_mut_path!(AssignPat);
    write_visit_mut_path!(KeyValuePatProp);
}
