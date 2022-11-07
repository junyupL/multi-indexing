extern crate proc_macro;
use proc_macro::Delimiter;
use proc_macro::TokenStream;
use proc_macro::TokenTree;

use syn::{*};

use syn::visit_mut::{self, VisitMut};
use syn::__private::{Span, ToTokens};

struct IndexVisitor;

impl VisitMut for IndexVisitor {
    // might do something with *expr operator since ptr[] replaces it
    /*fn visit_un_op_mut(&mut self, node: &mut UnOp) {

        println!("{:#?}", node);
        visit_mut::visit_un_op_mut(self, node);

    }*/
    fn visit_expr_mut(&mut self, node: &mut Expr) {
        match node {
            Expr::Index(index_expr) => {
                match &*index_expr.index {
                    Expr::Tuple(tuple) => {
                        if tuple.elems.len() == 0 {
                            let deref_paren = Expr::Paren(
                                ExprParen{
                                    attrs: Vec::new(),
                                    paren_token: token::Paren { span: index_expr.bracket_token.span }, //just any span - doesn't matter
                                    expr: Box::new(Expr::Unary(ExprUnary{
                                        attrs: Vec::new(),
                                        //again, any span works, and all future spans can also be any.
                                        op: UnOp::Deref(token::Star{spans: [index_expr.bracket_token.span]}),
                                        expr: index_expr.expr.clone()
                                    }
                                ))
                                }
                            );
                            *node = deref_paren;
                        }
                        else if tuple.elems.len() == 1 {
                            //This can't be the case because it is parsed as the expression just being incased by parenthesis
                            //but nothing needs to be done in this case anyways.
                        }
                        else {
                            let deref_paren = Expr::Paren(
                                ExprParen{
                                    attrs: Vec::new(),
                                    paren_token: token::Paren { span: index_expr.bracket_token.span },
                                    expr: Box::new(Expr::Unary(ExprUnary{
                                        attrs: Vec::new(),
                                        op: UnOp::Deref(token::Star{spans: [index_expr.bracket_token.span]}),
                                        expr: Box::new(Expr::MethodCall(syn::ExprMethodCall{
                                            args: tuple.elems.clone(),
                                            receiver: index_expr.expr.clone(),
                                            method: Ident::new(format!("index{}", tuple.elems.len()).as_str(), Span::call_site()),
                                            paren_token: token::Paren{span:  index_expr.bracket_token.span},
                                            turbofish: None,
                                            dot_token: token::Dot{spans: [index_expr.bracket_token.span]},
                                            attrs: Vec::new()

                                        }))  
                                    }
                                ))
                                }
                            );
                            *node = deref_paren;
                        }
                    },
                    _ => {}
                }
                    
                
                
            },
            _ => {}
        }
        visit_mut::visit_expr_mut(self, node);

    }
}

#[proc_macro]
pub fn file(body: TokenStream) -> TokenStream {
    let body = r_file(body);

    let mut file: syn::File = syn::parse(body.clone()).unwrap();
    IndexVisitor.visit_file_mut(&mut file);
    proc_macro::TokenStream::from(file.to_token_stream())
}

fn r_file(body: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();

    //just some random default value that isnt # or !
    let mut last_token = proc_macro::Punct::new('*', proc_macro::Spacing::Alone);
    for tt in body.into_iter() {
        
        match &tt {
            TokenTree::Group(x) => {
                
                if last_token.as_char() != '#' && last_token.as_char() != '!' && x.delimiter() == Delimiter::Bracket {
                    
                    let inside = r_file(x.stream());
                    let token_tree = TokenTree::Group(proc_macro::Group::new(Delimiter::Parenthesis, inside));
                    let token_tree = TokenTree::Group(proc_macro::Group::new(Delimiter::Bracket, TokenStream::from(token_tree)));
                    output.extend(TokenStream::from(token_tree).into_iter());
                } else {

                    let inside = r_file(x.stream());
                    let token_tree = TokenTree::Group(proc_macro::Group::new(x.delimiter(), inside));
                    output.extend(TokenStream::from(token_tree).into_iter());
                    
                }

                
            },
            TokenTree::Punct(x) => {
                last_token = x.clone();
                output.extend(TokenStream::from(tt.clone()).into_iter());
                
            },
            _ => {

                output.extend(TokenStream::from(tt.clone()).into_iter());
            }
        }
    }
    output
}