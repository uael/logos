use proc_macro2::TokenStream;
use quote::quote;

use crate::generator::{Context, Generator};
use crate::leaf::{Callback, Leaf};
use crate::util::MaybeVoid;

impl<'a> Generator<'a> {
    pub fn generate_leaf(&mut self, leaf: &Leaf, mut ctx: Context) -> TokenStream {
        let bump = ctx.bump();

        let ident = &leaf.ident;
        let name = self.name;
        let this = self.this;
        let ty = &leaf.field;

        let constructor = match leaf.field {
            MaybeVoid::Some(_) => quote!(#name::#ident),
            MaybeVoid::Void => quote!(|()| #name::#ident),
        };

        let sublexer = match &leaf.sublexer {
            None => quote!(),
            Some(ty) => quote! {
                let mut sublexer = #ty::lexer(lex.source());
                sublexer.bump_unchecked(lex.span().end);
                sublexer.trivia();

                while let Some(token) = sublexer.next() {
                    if token == #ty::ERROR {
                        lex.goto(sublexer.span());
                        lex.error(&[]);
                        return;
                    }
                }

                lex.bump_unchecked(sublexer.span().end - lex.span().end);
            }
        };

        match &leaf.callback {
            Some(Callback::Label(callback)) => quote! {
                #bump
                #sublexer
                #callback(lex).construct(#constructor, lex);
            },
            Some(Callback::Inline(inline)) => {
                let arg = &inline.arg;
                let body = &inline.body;

                quote! {
                    #bump
                    #sublexer

                    #[inline]
                    fn callback<'s>(#arg: &mut Lexer<'s>) -> impl CallbackResult<'s, #ty, #this> {
                        #body
                    }

                    callback(lex).construct(#constructor, lex);
                }
            }
            None if matches!(leaf.field, MaybeVoid::Void) => quote! {
                #bump
                #sublexer
                lex.set(#name::#ident);
            },
            None => quote! {
                #bump
                #sublexer
                let token = #name::#ident(lex.slice());
                lex.set(token);
            },
        }
    }
}
