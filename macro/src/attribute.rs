use cairo_lang_macro::{
    attribute_macro, quote, Diagnostic, Diagnostics, ProcMacroResult, TextSpan, Token, TokenStream, TokenTree
};
use cairo_lang_parser::utils::SimpleParserDatabase;
use cairo_lang_syntax::node::ast::Expr;
use cairo_lang_syntax::node::kind::SyntaxKind::ItemStruct;
use cairo_lang_syntax::node::{ast, db::SyntaxGroup};
use cairo_lang_syntax::node::{Terminal, TypedSyntaxNode};

/// Example of attribute macro.
/// Implement the `dojo::model` attribute which replaces all member types by `u128`
#[attribute_macro(parent = "dojo")]
pub fn model(_args: TokenStream, token_stream: TokenStream) -> ProcMacroResult {
    let db = SimpleParserDatabase::default();
    let (root_node, _diagnostics) = db.parse_token_stream(&token_stream);

    for n in root_node.descendants(&db) {
        if n.kind(&db) == ItemStruct {
            let struct_ast = ast::ItemStruct::from_syntax_node(&db, n);
            return process_model(&db, &struct_ast);
        }
    }

    ProcMacroResult::new(quote! {})
}

pub fn process_model(db: &dyn SyntaxGroup, struct_ast: &ast::ItemStruct) -> ProcMacroResult {
    let mut diagnostics = vec![];

    let member_nodes: Vec<_> = struct_ast
        .members(db)
        .elements(db)
        .iter()
        .map(|m| {
            match m.type_clause(db).ty(db) {
                Expr::Tuple(t) => {
                    diagnostics.push(Diagnostic::error(format!(
                        "The tuple '{}' is not supported",
                        t.as_syntax_node().get_text(db)
                    )));
                }
                _ => {}
            };

            let name = m.name(db).text(db);
            let name = TokenTree::Ident(Token::new(name, TextSpan::call_site()));
            quote! {
               #name : u128,
            }
        })
        .collect();

    let mut body = TokenStream::empty();
    for node in member_nodes {
        body.extend(node);
    }

    let model_name = struct_ast.name(db).text(db);
    let model_name = TokenTree::Ident(Token::new(model_name, TextSpan::call_site()));

    let tokens = quote! {
        struct #model_name {
            #body
        }
    };

    println!("[Attribute] Debug output:\n{}\n", tokens.to_string());

    ProcMacroResult::new(tokens).with_diagnostics(Diagnostics::new(diagnostics))
}
