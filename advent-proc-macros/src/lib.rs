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
    let mut fn_source_code_match_patterns = quote! {};

    let year_day_struct_ident = Ident::new(&format!("Advent{}Day", year), year.span());
    let lowercase_year_ident = Ident::new(&format!("yr{}", year), year.span());

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

        let day_file_path = LitStr::new(
            &format!("{}/{}.rs", lowercase_year_ident, lowercase_day_ident),
            day.span(),
        );

        fn_source_code_match_patterns = quote! {
            #fn_source_code_match_patterns
            #year_day_struct_ident::#uppercase_day_ident => include_bytes!(#day_file_path),
        };
    }

    TokenStream::from(quote! {
        #day_modules

        use clap::{Command, ValueEnum};
        use tree_sitter_highlight::{
            HighlightConfiguration,
            Highlighter,
            HighlightEvent
        };

        use super::AdventPuzzlePart;

        #[derive(ValueEnum, Clone)]
        pub(crate) enum #year_day_struct_ident {
            #year_day_struct_fields
        }

        pub(crate) fn #lowercase_year_ident(cmd: &mut Command, day: #year_day_struct_ident, part: Option<AdventPuzzlePart>, input: Option<String>) {
            if let Some(part) = part {
                if let Some(input) = input {
                    match day {
                        #fn_match_patterns
                    }
                } else {
                    cmd.error(
                        clap::error::ErrorKind::MissingRequiredArgument,
                        "the following arguments were not provided and are required because [PART] was specified:\n  \x1b[32m[INPUT]\x1b[39m",
                    ).exit();
                }
            } else {
                let highlight_names = [
                    // Red
                    "property",
                    // Green
                    "string",
                    "string.special",
                    // Yellow
                    "constant",
                    // Blue
                    "function.builtin",
                    "function",
                    // Magenta
                    "keyword",
                    // Cyan
                    "type",
                    "type.builtin",
                ];
                let mut highlighter = Highlighter::new();
                let rust_language = tree_sitter_rust::language();
                let mut rust_config = HighlightConfiguration::new(
                    rust_language,
                    tree_sitter_rust::HIGHLIGHT_QUERY,
                    tree_sitter_rust::INJECTIONS_QUERY,
                    "",
                ).unwrap();
                rust_config.configure(&highlight_names);
                let source: &[u8] = match day {
                    #fn_source_code_match_patterns
                };
                let highlights = highlighter.highlight(
                    &rust_config,
                    source,
                    None,
                    |_| None,
                ).unwrap();
                let source_str = std::str::from_utf8(source).unwrap();

                for event in highlights {
                    match event.unwrap() {
                        HighlightEvent::Source { start, end } => {
                            print!("{}", &source_str[start..end]);
                        },
                        HighlightEvent::HighlightStart(s) => {
                            match s.0 {
                                0 => print!("\x1b[31m"),
                                1 | 2 => print!("\x1b[32m"),
                                3 => print!("\x1b[33m"),
                                4 | 5 => print!("\x1b[94m"),
                                6 => print!("\x1b[35m"),
                                7 | 8 => print!("\x1b[36m"),
                                _ => (),
                            }
                        },
                        HighlightEvent::HighlightEnd => {
                            print!("\x1b[39m")
                        },
                    }
                }
            }
        }
    })
}
