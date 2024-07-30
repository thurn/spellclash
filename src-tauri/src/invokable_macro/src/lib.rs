// Copyright © spellclash 2024-present
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(Invokable)]
pub fn invokable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_invokable(&ast)
}

fn impl_invokable(ast: &syn::DeriveInput) -> TokenStream {
    let Data::Struct(data) = &ast.data else {
        panic!("Expected Invokable to be applied to a struct.");
    };
    let mut fields = vec![];
    for field in &data.fields {
        let name = field.ident.as_ref().expect("Expected named struct field");
        fields.push(quote! {
            self.#name.initialize(id);
        });
    }

    let name = &ast.ident;
    let gen = quote! {
        use invokable::InvokableType;
        use invokable::CardIdent;

        impl InvokableType for #name {
            fn initialize(&self, id: CardIdent) {
                #( #fields )*
            }
        }
    };
    gen.into()
}
