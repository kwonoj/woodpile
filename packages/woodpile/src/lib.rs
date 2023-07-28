use js_sys::Reflect;
use serde_wasm_bindgen::Serializer;
use swc_core::{
    common::{sync::Lrc, FileName, FilePathMapping, SourceMap},
    ecma::{
        ast::Program,
        visit::{VisitMutWith, VisitMutWithPath},
    },
};
use swc_estree_ast::flavor::Flavor;
use visitors::{BaseVisitor, PathVisitor};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod visit_mut_macros;
mod visit_mut_macros_helper;
mod visitors;

#[wasm_bindgen(typescript_custom_section)]
const VISITOR_INTERFACE: &'static str = r#"
export interface Visitor<U = Record<string, any>> {

}

export interface PathVisitor<U = Record<string, any>> {

}

export interface BaseVisitorOption<C = Record<string, any>> {
    // Arbitrary context attached in the visitor.
    // This can be accessed visitor callback's last argument.
    [key: string]: any;
    // The actual object contains visitor callbacks, called with (node, context)
    visit?: Visitor<C>;
}

export interface PathVisitorOption<C = Record<string, any>> {
    // Arbitrary context attached in the visitor.
    // This can be accessed visitor callback's last argument.
    [key: string]: any;
    // The actual object contains visitor callbacks, called with (node, path, context)
    // path is an array of parent nodes.
    visitWithPath?: PathVisitor<C>;
}

export type VisitorOptions = BaseVisitorOption & PathVisitorOption;

/// Traverse a given AST with specified visitor.
export function visit<T = Record<string, any>>(ast: T, visitor: VisitorOptions): void;
"#;

#[wasm_bindgen(skip_typescript)]
pub fn visit(p: JsValue, visitor: JsValue) {
    console_error_panic_hook::set_once();
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

#[wasm_bindgen(getter_with_clone, skip_typescript)]
pub struct CompatOptions {
    pub source: Option<String>,
    pub flavor: Option<String>,
}

#[wasm_bindgen(typescript_custom_section)]
const COMPAT_INTERFACE: &'static str = r#"
export interface CompatOptions {
    source?: string;
    flavor?: "acorn" | "babel";
}

// Returns an estree-compatbile AST from SWC's AST.
export function compat<T = Record<string, any>, U = Record<string, any>>(ast: T, options?: CompatOptions): U;
"#;

#[wasm_bindgen(skip_typescript)]
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
