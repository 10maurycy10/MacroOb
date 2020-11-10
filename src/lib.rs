use proc_macro::{Group, Ident, TokenStream, TokenTree};
use rot13::rot13;

fn rottt(t: TokenTree) -> TokenTree {
    match t {
        TokenTree::Group(g) => {
            let mut d = Group::new(g.delimiter(), rot_stream(g.stream()));
            d.set_span(g.span());
            TokenTree::Group(d)
	        }
        TokenTree::Ident(i) => {
            let gi = Ident::new(&rot13(&i.to_string()), i.span());
            TokenTree::Ident(gi)
        }
        _ => t,
    }
}

fn rot_stream(ts: TokenStream) -> TokenStream {
    ts.into_iter().map(rottt).collect()
}

#[proc_macro]
pub fn rot(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let output: TokenStream = rot_stream(input);
	output
}