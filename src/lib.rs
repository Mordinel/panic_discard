
use proc_macro::TokenStream;

use syn::{parse_macro_input, ItemFn};
use quote::quote;

#[proc_macro_attribute]
pub fn panic_discard(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as ItemFn);

    let original_block = input_fn.block.clone();

    input_fn.block.stmts = vec![
        syn::parse2(quote! {
            let _ = std::panic::catch_unwind(|| {
                #original_block
            });
        })
        .unwrap(),
    ];

    TokenStream::from(quote! {
        #input_fn
    })
}

