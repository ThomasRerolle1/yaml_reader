use proc_macro::TokenStream;
use quote::quote;
use std::{io::Read, path::Path};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn double(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Expr);

    let expanded = quote! {
        #input * 2
    };
    TokenStream::from(expanded)
}

#[proc_macro]
pub fn yaml_reader(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let file_name = input.to_string().replace("\"", "").trim().to_string();

    let path_to_file = Path::new(&file_name);

    if !path_to_file.exists() {
        return TokenStream::from(quote!("File does not exist"));
    }

    let file_result: Result<std::fs::File, std::io::Error> = std::fs::File::open(file_name.clone());
    let mut buf = String::new();

    if let Ok(mut file) = file_result {
        let content_result: Result<usize, std::io::Error> = file.read_to_string(&mut buf);

        if content_result.is_err() {
            buf = format!("Failed to read '{}' file", file_name);
        }
    } else {
        buf = format!("Couldn't open '{}'", file_name);
    }

    let res = quote! {
        #buf
    };

    TokenStream::from(res)
    //format!("{{println!(\"{}\")}}", contents).parse().unwrap()
}
