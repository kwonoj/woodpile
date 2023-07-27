use js_sys::Reflect;
use paste::paste;
use serde_wasm_bindgen::Serializer;
use swc_core::{
    common::{pass::AstKindPath, sync::Lrc, FileName, FilePathMapping, SourceMap},
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
use swc_estree_ast::flavor::Flavor;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

/// Calls a corresponding visitor callback in JavaScript with the given arguments.
/// if visitor is _with_path, passes path_arg.
fn call_visitor_reflected_fn(
    visitor: &JsValue,
    context: &JsValue,
    property: &str,
    arg: &JsValue,
    path_arg: Option<&JsValue>,
) -> Option<JsValue> {
    let fn_value = Reflect::has(visitor, &JsValue::from_str(property))
        .map(|has| {
            if has {
                Reflect::get(visitor, &JsValue::from_str(property)).ok()
            } else {
                None
            }
        })
        .unwrap_or_default();

    if let Some(fn_value) = fn_value {
        let fn_value = fn_value.dyn_into::<js_sys::Function>();
        if let Ok(fn_value) = fn_value {
            let result = if let Some(path_arg) = path_arg {
                fn_value.call3(context, &arg, path_arg, context)
            } else {
                fn_value.call2(context, &arg, context)
            };
            if let Ok(result) = result {
                if result.is_object() {
                    return Some(result);
                }
            }
        }
    }

    None
}

/// A visitor for visit callbacks.
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

/// Macro wraps each visit_mut_NODE
macro_rules! write_visit_mut {
    ($capital: ident, $ty: ident) => {
        paste! {
            fn [<visit_mut_$capital:lower _$ty:snake>](&mut self, n: &mut [<$capital:upper $ty>]) {
                let path_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize path {}", stringify!([<$capital:upper $ty>])).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}", stringify!([<$capital$ty:camel>])),
                    &path_jsvalue,
                    None
                );

                if let Some(ret) = ret {
                    let ret: [<$capital:upper $ty>] = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!([<$capital:upper $ty>])).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with(self);
            }
        }
    };
    ($ty:ident) => {
        paste! {
            // using paste! macro, combine visit_mut_ and $ty into snakecase
            fn [<visit_mut_$ty:snake>](&mut self, n: &mut $ty) {
                let path_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize path {}", stringify!($ty)).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}", stringify!([<$ty:camel>])),
                    &path_jsvalue,
                    None
                );

                if let Some(ret) = ret {
                    let ret: $ty = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!($ty)).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with(self);
            }
        }
    };
}

macro_rules! write_visit_mut_plural {
    ($ty:ident) => {
        paste! {
            fn [<visit_mut_$ty:snake s>](&mut self, n: &mut Vec<$ty>) {
                let path_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize path {}", stringify!([<$ty s>])).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}s", stringify!([<$ty:camel>])),
                    &path_jsvalue,
                    None
                );

                if let Some(ret) = ret {
                    let ret: Vec<$ty> = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!([<$ty s>])).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with(self);
            }
        }
    };
}

impl VisitMut for BaseVisitor {
    noop_visit_mut_type!();

    write_visit_mut!(Program);
    write_visit_mut!(Module);
    write_visit_mut!(Script);
    write_visit_mut!(ModuleItem);
    write_visit_mut_plural!(ModuleItem);
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

struct PathVisitor {
    visitor_context: JsValue,
    visitor: JsValue,
}

macro_rules! write_visit_mut_path {
    ($capital: ident, $ty: ident) => {
        paste! {
            fn [<visit_mut_$capital:lower _$ty:snake>](&mut self, n: &mut [<$capital:upper $ty>], p: &mut AstKindPath<AstParentKind>) {
                let node_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize path {}", stringify!([<$capital:upper $ty>])).as_str());
                let path: &Vec<AstParentKind> = &*p;
                let path_jsvalue = serde_wasm_bindgen::to_value(path).expect(format!("Should be able to serialize path {}", stringify!([<$capital:upper $ty>])).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}", stringify!([<$capital$ty:camel>])),
                    &node_jsvalue,
                    Some(&path_jsvalue)
                );

                if let Some(ret) = ret {
                    let ret: [<$capital:upper $ty>] = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!([<$capital:upper $ty>])).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with_path(self, p);
            }
        }
    };
    ($ty:ident) => {
        paste! {
            fn [<visit_mut_$ty:snake>](&mut self, n: &mut $ty, p: &mut AstKindPath<AstParentKind>) {
                let node_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize node {}", stringify!($ty)).as_str());
                let path: &Vec<AstParentKind> = &*p;
                let path_jsvalue = serde_wasm_bindgen::to_value(path).expect(format!("Should be able to serialize path {}", stringify!($ty)).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}", stringify!([<$ty:camel>])),
                    &node_jsvalue,
                    Some(&path_jsvalue)
                );

                if let Some(ret) = ret {
                    let ret: $ty = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!($ty)).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with_path(self, p);
            }
        }
    };
}

macro_rules! write_visit_mut_path_plural {
    ($ty:ident) => {
        paste! {
            fn [<visit_mut_$ty:snake s>](&mut self, n: &mut Vec<$ty>, p: &mut AstKindPath<AstParentKind>) {
                let node_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize node {}", stringify!([<$ty s>])).as_str());
                let path: &Vec<AstParentKind> = &*p;
                let path_jsvalue = serde_wasm_bindgen::to_value(path).expect(format!("Should be able to serialize path {}", stringify!([<$ty s>])).as_str());

                let ret = call_visitor_reflected_fn(
                    &self.visitor,
                    &self.visitor_context,
                    &format!("visit{}s", stringify!([<$ty:camel>])),
                    &node_jsvalue,
                    Some(&path_jsvalue)
                );

                if let Some(ret) = ret {
                    let ret: Vec<$ty> = serde_wasm_bindgen::from_value(ret).expect(format!("Should be able to deserialize {}", stringify!([<$ty s>])).as_str());
                    *n = ret;
                }

                n.visit_mut_children_with_path(self, p);
            }
        }
    };
}

impl PathVisitor {
    pub fn new(context: JsValue, visitor: JsValue) -> Self {
        Self {
            visitor_context: context,
            visitor,
        }
    }
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

#[wasm_bindgen]
pub fn visit(p: JsValue, visitor: JsValue) {
    let mut p: Program = serde_wasm_bindgen::from_value(p).unwrap();

    let visitor_value = if Reflect::has(&visitor, &JsValue::from_str("visit")).is_ok() {
        Reflect::get(&visitor, &JsValue::from_str("visit")).ok()
    } else {
        None
    };

    if let Some(visitor_value) = visitor_value {
        let mut base_visitor = BaseVisitor::new(visitor.clone(), visitor_value);
        p.visit_mut_with(&mut base_visitor);
    }

    let visitor_with_path = if Reflect::has(&visitor, &JsValue::from_str("visitWithPath")).is_ok() {
        Reflect::get(&visitor, &JsValue::from_str("visitWithPath")).ok()
    } else {
        None
    };

    if let Some(visitor_with_path) = visitor_with_path {
        let mut path_visitor = PathVisitor::new(visitor, visitor_with_path);
        p.visit_mut_with_path(&mut path_visitor, &mut Default::default());
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct CompatOptions {
    pub source: Option<String>,
    pub flavor: Option<String>,
}

#[wasm_bindgen()]
pub fn compat(p: JsValue, opts: Option<CompatOptions>) -> JsValue {
    let p: Program = serde_wasm_bindgen::from_value(p).unwrap();

    let src_input = opts
        .as_ref()
        .and_then(|opts| opts.source.as_ref())
        .map(|x| Lrc::new(x.to_string()));

    let cm = std::sync::Arc::new(SourceMap::new(FilePathMapping::empty()));
    let fm = cm.new_source_file_from(FileName::Anon, src_input.unwrap_or_default());

    let context = swc_estree_compat::babelify::Context {
        cm,
        fm,
        comments: Default::default(),
    };

    let p: swc_estree_ast::File =
        swc_estree_compat::babelify::Babelify::babelify(p, &context).into();

    let flavor = if let Some(opts) = opts {
        if let Some(flavor) = opts.flavor {
            match flavor.as_str() {
                "acorn" => Flavor::Acorn {
                    extra_comments: false,
                },
                _ => Flavor::Babel,
            }
        } else {
            Flavor::Babel
        }
    } else {
        Flavor::Babel
    };

    let serializer = Serializer::json_compatible()
        .serialize_missing_as_null(false)
        //https://github.com/serde-rs/serde/issues/1346
        .serialize_maps_as_objects(true);

    // [TODO]: Error handling
    let result = flavor
        .with(|| serde::Serialize::serialize(&p, &serializer))
        .unwrap();

    return result;
}
