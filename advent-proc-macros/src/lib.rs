use proc_macro::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token, Ident, LitInt, LitStr, Result, Token,
};

struct Year {
    year: LitInt,
    bracket_token: token::Bracket,
    days: Punctuated<LitInt, Token![,]>,
}

impl Parse for Year {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        Ok(Year {
            year: input.parse()?,
            bracket_token: bracketed!(content in input),
            days: content.parse_terminated(LitInt::parse, Token![,])?,
        })
    }
}

#[proc_macro]
pub fn year(tokens: TokenStream) -> TokenStream {
    let Year {
        year,
        bracket_token: _bracket_token,
        days,
    } = parse_macro_input!(tokens as Year);
    let mut day_modules = quote! {};
    let mut year_day_struct_fields = quote! {};
    let mut fn_match_patterns = quote! {};

    let year_day_struct_ident = Ident::new(&format!("Advent{}Day", year), year.span());

    for day in days {
        let lowercase_day_ident = Ident::new(&format!("dy{}", day), day.span());

        day_modules = quote! {
            #day_modules
            mod #lowercase_day_ident;
        };

        let day_litstr = LitStr::new(&day.to_string(), day.span());
        let uppercase_day_ident = Ident::new(&format!("Dy{}", day), day.span());

        year_day_struct_fields = quote! {
            #year_day_struct_fields
            #[value(name = #day_litstr)]
            #uppercase_day_ident,
        };

        fn_match_patterns = quote! {
            #fn_match_patterns
            #year_day_struct_ident::#uppercase_day_ident => match part {
                AdventPuzzlePart::Pt1 => #lowercase_day_ident::pt1(input),
                AdventPuzzlePart::Pt2 => #lowercase_day_ident::pt2(input),
            },
        };
    }

    let fn_ident = Ident::new(&format!("yr{}", year), year.span());

    TokenStream::from(quote! {
        #day_modules

        use clap::ValueEnum;

        use super::AdventPuzzlePart;

        #[derive(ValueEnum, Clone)]
        pub(crate) enum #year_day_struct_ident {
            #year_day_struct_fields
        }

        pub(crate) fn #fn_ident(day: #year_day_struct_ident, part: AdventPuzzlePart, input: String) {
            match day {
                #fn_match_patterns
            }
        }
    })
}
