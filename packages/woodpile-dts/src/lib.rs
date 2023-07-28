use convert_case::{Case, Casing};
use syn::Type;

extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate serde;
extern crate syn;

/// Generate a static typescript_custom_section augments to given interface.
#[proc_macro_attribute]
pub fn typed_visit(
    attr: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let syn::ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse(input.clone()).expect("failed to parse tokens as a function");
    let stmts = &block.stmts;

    if attr.to_string() != "Visitor" && attr.to_string() != "PathVisitor" {
        panic!("typed_visit only supports Visitor and PathVisitor");
    }

    let is_path_enabled = attr.to_string() == "PathVisitor";

    // Naively check if the argument to the node is Vec
    let is_visit_plural = if let syn::FnArg::Typed(typed) = &sig.inputs[1] {
        if let Type::Reference(path) = &*typed.ty {
            if let Type::Path(path) = &*path.elem {
                path.path.segments[0].ident.to_string() == "Vec"
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    };

    // Using input tokens, format a signature for the visitor fn for typescript.
    // This is somewhat limited since we don't bring strong types from @swc/core,
    // most of actual node types to be specified via generics.
    let var_name = format_ident!(
        "_{}",
        sig.ident.to_string().replace("_", "").to_case(Case::Upper)
    );
    let interface_name = format_ident!("{}", attr.to_string());
    let fn_name = format_ident!(
        "{}",
        sig.ident
            .to_string()
            .to_case(Case::Camel)
            .replace("Mut", "")
    );
    let node_args = if is_visit_plural { "Array<R>" } else { "R" };
    // We don't have strong types for path yet
    let definition_args = if is_path_enabled {
        format!(
            "node: {}, path: Array<Record<string, any>>, self: U",
            node_args
        )
    } else {
        format!("node: {}, self: U", node_args)
    };

    let definition = format!(
        "interface {}<U = Record<string, any>> {{
        {}?: <R = Record<string, any>>({}) => void | Record<string, any>;
    }}",
        interface_name, fn_name, definition_args
    );

    // Declare const for ts custom section _inside_ of visitor function,
    // to avoid declaration collision or not able to declare due to trait scope.
    quote! {
        #(#attrs)* #vis #sig {
            #[wasm_bindgen(typescript_custom_section)]
            const #var_name: &'static str = #definition;

            #(#stmts)*
        }
    }
    .into()
}
