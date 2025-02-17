use cairo_lang_macro::{
    inline_macro, quote, ProcMacroResult, TextSpan, TokenStream, TokenTree, Token
};
use cairo_lang_parser::utils::SimpleParserDatabase;
use cainome::cairo_serde::{ByteArray, CairoSerde};
use smol_str::ToSmolStr;
use starknet_crypto::poseidon_hash_many;

/// Example of inline macro.
/// implements the `bytearray_hash` macro which replaces the input string value
/// by its hash value.
#[inline_macro]
pub fn bytearray_hash(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (root_node, _diagnostics) = db.parse_token_stream(&token_stream);

    let input = root_node.get_text(&db);
    let input = input.strip_prefix("(\"").unwrap().strip_suffix("\")").unwrap();

    let ba = ByteArray::from_string(input).unwrap_or_else(|_| panic!("Invalid ByteArray: {}", input));
    let hash = poseidon_hash_many(&ByteArray::cairo_serialize(&ba));

    let token = TokenTree::Ident(Token::new(hash.to_smolstr(), TextSpan::call_site()));
    let tokens = quote! { #token };

    println!("[Inline] Debug output:\n{}\n", tokens.to_string());

    ProcMacroResult::new(tokens)
}
