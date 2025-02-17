use cairo_lang_macro::{
    derive_macro, quote, ProcMacroResult, TextSpan, TokenStream, TokenTree, Token
};
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::{ast, kind::SyntaxKind::ItemStruct, Terminal, TypedSyntaxNode};
use smol_str::ToSmolStr;

/// Example of derive macro.
/// implements the `Introspect` macro which generates the implementation of the NameTrait
/// for the underlying struct.
#[derive_macro]
pub fn introspect(token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (root_node, _diagnostics) = db.parse_token_stream(&token_stream);

    for n in root_node.descendants(&db) {
        if n.kind(&db) == ItemStruct {
            let struct_ast = ast::ItemStruct::from_syntax_node(&db, n);
            let tokens = process(&db, &struct_ast);

            println!("[Derive] Debug output:\n{}\n", tokens.to_string());

            return ProcMacroResult::new(tokens);
        }
    }

    ProcMacroResult::new(quote! {})
}

pub fn process(db: &SimpleParserDatabase, struct_ast: &ast::ItemStruct) -> TokenStream {

    let name_string = struct_ast.name(db).text(db);
    let name_token = TokenTree::Ident(Token::new(name_string.clone(), TextSpan::call_site()));

    let impl_string = format!("{}NameImpl", name_string.clone()).to_smolstr();
    let impl_token = TokenTree::Ident(Token::new(impl_string, TextSpan::call_site()));

    let res_string = format!("\"{}\"", name_string);
    let res_token = TokenTree::Ident(Token::new(res_string, TextSpan::call_site()));

    quote! {
        impl #impl_token of NameTrait<#name_token> {
            fn name(self: @#name_token) -> ByteArray {
                #res_token
            } 
        }
    }
}
