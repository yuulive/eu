#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn;
use syn::spanned::Spanned;

#[cfg(test)]
mod example;
#[cfg(test)]
mod tests {
    use crate::example::add1;
    #[test]
    fn it_works() {
        let a = add1();
        let with_three = a.x(|| 3);
        let and_two = with_three.y(|| 2);
        assert_eq!(and_two.call(), 5);
    }
}

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    item
}

#[proc_macro_attribute]
pub fn part_app(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func_item: syn::Item = syn::parse(item).expect("failed to parse input");
    if !attr.is_empty() {
        func_item
            .span()
            .unstable()
            .error("No attributes accepted")
            .emit()
    }

    match func_item {
        syn::Item::Fn(ref func) => {
            let name = get_name(func);
            let predicate =
                syn::Ident::new(&format!("__PartialApplication__{}_", name), name.span());
            // TODO: maybe these should be public if the original function is
            // itself public
            println!("predicate: {}", predicate);
            let body = &func.block;
            let func_struct = main_struct(&predicate, &func.sig.inputs, &func.sig.output);
            println!("struct: {}", func_struct);
        }
        _ => func_item
            .span()
            .unstable()
            .error("Only functions can be partially applied, for structs use the builder pattern")
            .emit(),
    }
    let out = quote! { #func_item };
    out.into()
}

fn get_name<'a>(func: &'a syn::ItemFn) -> &'a syn::Ident {
    if let Some(r) = &func.sig.receiver() {
        r.span()
            .unstable()
            .error("Cannot make methods partially applicable yet")
            .emit();
    }
    &func.sig.ident
}

fn main_struct<'a>(
    name: &'a syn::Ident,
    args: &'a syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
    ret_type: &'a syn::ReturnType,
) -> proc_macro::TokenStream {
    let arg_vec: Vec<_> = args
        .iter()
        .map(|fn_arg| match fn_arg {
            syn::FnArg::Receiver(_) => panic!("should filter out reciever arguments"),
            syn::FnArg::Typed(t) => t,
        })
        .collect();

    let arg_types: Vec<_> = arg_vec
        .iter()
        .map(|f| {
            let ty = &f.ty;
            syn::Ident::new(&format!("{}", quote!(#ty)), f.span())
        })
        .collect();

    let arg_names: Vec<_> = arg_vec
        .iter()
        .map(|f| {
            let f_pat = &f.pat;
            syn::Ident::new(&format!("{}", quote!(#f_pat)), f.span())
        })
        .collect();

    let arg_augmented_only: Vec<_> = arg_names
        .iter()
        .map(|f| syn::Ident::new(&format!("{}___FN", quote!(#f)), f.span()))
        .collect();

    // iterators are consumed in one use
    let iter1 = arg_names.iter();
    let iter2 = arg_augmented_only.iter();
    let iter3 = arg_types.iter();
    let iter4 = arg_types.iter();
    let iter5 = arg_augmented_only.iter();
    TokenStream::from(quote!(
        struct #name <#(#iter1, #iter2,)*BODYFN>
        where
            #(#iter5: FnOnce() -> #iter3,)*
            BODYFN: FnOnce(#(#iter4,)*) #ret_type,
        {
            stuff
        }
    ))
}
