use js_sys::Reflect;
use serde::de::DeserializeOwned;
use swc_core::{common::pass::AstKindPath, ecma::visit::AstParentKind};
use wasm_bindgen::{JsCast, JsValue};

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
                fn_value
                    .call3(context, &arg, path_arg, context)
                    .expect("Visitor function failed")
            } else {
                fn_value
                    .call2(context, &arg, context)
                    .expect("Visitor function failed")
            };

            if result.is_object() {
                return Some(result);
            }
        }
    }

    None
}

/// Generalized fn to be called in each visit_mut macro expansion,
/// does serialization / deserization to jsvalues then calls
/// corresponding JS callbacks for the specific visitor node.
pub fn call_visit_mut_inner<T: serde::Serialize + DeserializeOwned>(
    visitor: &JsValue,
    context: &JsValue,
    // Actual node, i.e `n` in `visit_mut_expr(&mut self, n: &mut Expr)`
    node: &mut T,
    // Name of the property on the javascript visitor fn to call, i.e `visitExpr`
    js_node_property: &str,
    // stringified node types for error messages, i.e ModuleItems, Expr, etc.
    node_display: &str,
    node_paths: Option<&mut AstKindPath<AstParentKind>>,
) {
    let node_jsvalue = serde_wasm_bindgen::to_value(node)
        .expect(format!("Should be able to serialize node {}", node_display).as_str());

    let path_jsvalue = if let Some(node_paths) = node_paths {
        let path: &Vec<AstParentKind> = &*node_paths;
        let value = serde_wasm_bindgen::to_value(path)
            .expect(format!("Should be able to serialize path {}", node_display).as_str());
        Some(value)
    } else {
        None
    };

    let ret = call_visitor_reflected_fn(
        &visitor,
        &context,
        js_node_property,
        &node_jsvalue,
        path_jsvalue.as_ref(),
    );

    if let Some(ret) = ret {
        let ret: T = serde_wasm_bindgen::from_value(ret)
            .expect(format!("Should be able to deserialize {}", node_display).as_str());
        *node = ret;
    }
}
