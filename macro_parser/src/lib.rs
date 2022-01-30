extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// a proc macro takes tokens as argument, and generates tokens
#[proc_macro]
pub fn parse(input: TokenStream) -> TokenStream {
    // we expect a string literal here, we let syn extract it
    let s: syn::LitStr = syn::parse(input).unwrap();
    let string = s.value();

    // we can then parse that string. We unwrap here because
    // panicking will display a compilation error
    let (_, name) = parser::parser(&string).unwrap();

    // we can then generate code using what we parsed. That
    // code will replace the macro call, so
    // `let val = parse!("Hello Alice!");` will be replaced
    // by `let val = Hello { name: "Alice" };`
    let gen = quote! {
        Hello {
            name: #name,
        }
    };
    gen.into()
}

