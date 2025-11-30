use proc_macro::TokenStream;
use syn::{parse_macro_input, Expr, Token};
use syn::punctuated::Punctuated;
use syn::parse::{Parse, ParseStream};
use quote::quote;

struct KeyValue {
    key: Expr,
    _arrow: Token![=>],
    value: Expr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(KeyValue {
            key: input.parse()?,
            _arrow: input.parse()?,
            value: input.parse()?,
        })
    }
}

struct BTreeMapInput {
    pairs: Punctuated<KeyValue, Token![,]>,
}

impl Parse for BTreeMapInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(BTreeMapInput {
            pairs: Punctuated::parse_terminated(input)?,
        })
    }
}

#[proc_macro]
pub fn btreemap_proc(input: TokenStream) -> TokenStream {
    let BTreeMapInput { pairs } = parse_macro_input!(input as BTreeMapInput);

    let inserts = pairs.iter().map(|kv| {
        let k = &kv.key;
        let v = &kv.value;
        quote! { map.insert(#k, #v); }
    });

    let expanded = quote! {
        {
            let mut map = std::collections::BTreeMap::new();
            #(#inserts)*
            map
        }
    };

    expanded.into()
}
