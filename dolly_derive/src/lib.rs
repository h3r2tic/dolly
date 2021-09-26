extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(DollyDriver)]
pub fn dolly_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_dolly_macro(&ast)
}

fn impl_dolly_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl RigDriver for #name {
            fn update(&mut self, params: RigUpdateParams) -> dolly::transform::Transform {
                let t = self.rig.update(params.delta_time_seconds);
                dolly::transform::Transform {
                    position: t.position,
                    rotation: t.rotation,
                }
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    };
    gen.into()
}
