use js_sys::Reflect;
use paste::paste;
use swc_core::ecma::{
    ast::{Module, Program},
    visit::{noop_visit_mut_type, VisitMut, VisitMutAstPath, VisitMutWith, VisitMutWithPath},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub struct BaseVisitor {
    visitor: Option<JsValue>,
    visitor_with_path: Option<JsValue>,
}

impl BaseVisitor {
    pub fn new(visitor: JsValue) -> Self {
        let visitor_value = if Reflect::has(&visitor, &JsValue::from_str("visit")).is_ok() {
            Reflect::get(&visitor, &JsValue::from_str("visit")).ok()
        } else {
            None
        };

        let visitor_with_path =
            if Reflect::has(&visitor, &JsValue::from_str("visitWithPath")).is_ok() {
                Reflect::get(&visitor, &JsValue::from_str("visitWithPath")).ok()
            } else {
                None
            };

        Self {
            visitor: visitor_value,
            visitor_with_path,
        }
    }

    fn call_visitor_reflected_fn(&self, property: &str, arg: &JsValue) {
        if let Some(visitor) = &self.visitor {
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
                    let _ = fn_value.call1(&JsValue::null(), &arg);
                }
            }
        }
    }
}

macro_rules! write_visit_mut {
    ($ty:ident) => {
        paste! {
            fn [<visit_mut_$ty:lower>](&mut self, n: &mut $ty) {
                let path_jsvalue = serde_wasm_bindgen::to_value(n).expect(format!("Should be able to serialize path {}", stringify!($ty)).as_str());

                self.call_visitor_reflected_fn(
                    &stringify!([<$ty:lower>]),
                    &path_jsvalue
                );

                n.visit_mut_children_with(self);
            }
        }
    };
}

impl VisitMut for BaseVisitor {
    noop_visit_mut_type!();

    write_visit_mut!(Program);
    write_visit_mut!(Module);
}

impl VisitMutAstPath for BaseVisitor {}

#[wasm_bindgen]
pub fn visit(p: JsValue, visitor: JsValue) {
    let mut p: Program = serde_wasm_bindgen::from_value(p).unwrap();

    let mut visitor = BaseVisitor::new(visitor);
    p.visit_mut_with(&mut visitor);
}
