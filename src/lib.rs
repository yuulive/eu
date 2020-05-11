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
            let added_unit = concat_ident(name, "Added");
            let empty_unit = concat_ident(name, "Empty");
            let argument_vector = argument_vector(&func.sig.inputs);
            println!("predicate: {}", predicate);
            let body: &Box<syn::Block> = &func.block;
            let func_struct = main_struct(&predicate, &argument_vector, &func.sig.output);
            let generator_func = generator_func(
                &predicate,
                name,
                &argument_vector,
                &func.sig.output,
                &empty_unit,
                body,
            );
            println!("struct: {}", func_struct);
            println!("generator: {}", generator_func);
            quote! {
                #[allow(non_camel_case_types,non_snake_case)]
                struct #added_unit;
                #[allow(non_camel_case_types,non_snake_case)]
                struct #empty_unit;
            }
        }
        _ => {
            func_item
                .span()
                .unstable()
                .error(
                    "Only functions can be partially applied, for structs use the builder pattern",
                )
                .emit();
            quote! { #func_item }
        }
    }
    .into()
}

fn generator_func<'a>(
    struct_name: &'a syn::Ident,
    name: &'a syn::Ident,
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
    empty_unit: &'a syn::Ident,
    body: &'a Box<syn::Block>,
) -> proc_macro::TokenStream {
    let arg_names = arg_names(&args);
    let arg_types = arg_types(&args);
    let marker_names = marker_names(&arg_names);

    let out = quote! {
        fn #name< #(#arg_names,)* >() -> #struct_name<#(#empty_unit,#arg_names,)*
        impl FnOnce(#(#arg_types,)*) #ret_type>
        where
            #(#arg_names: FnOnce() -> #arg_types,)*
        {
            #struct_name {
                #(#arg_names: None,)*
                #(#marker_names: ::std::marker::PhantomData,)*
                body: |#(#arg_names,)*| #body
            }
        }

    };
    out.into()
}

fn arg_names<'a>(args: &Vec<&syn::PatType>) -> Vec<syn::Ident> {
    args.iter()
        .map(|f| {
            let f_pat = &f.pat;
            syn::Ident::new(&format!("{}", quote!(#f_pat)), f.span())
        })
        .collect()
}

fn marker_names(names: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    names.iter().map(|f| concat_ident(f, "m")).collect()
}

fn concat_ident<'a>(ident: &'a syn::Ident, end: &str) -> syn::Ident {
    let name = format!("{}___{}", quote! {#ident}, end);
    syn::Ident::new(&name, ident.span())
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

fn argument_vector<'a>(
    args: &'a syn::punctuated::Punctuated<syn::FnArg, syn::token::Comma>,
) -> Vec<&syn::PatType> {
    args.iter()
        .map(|fn_arg| match fn_arg {
            syn::FnArg::Receiver(_) => panic!("should filter out reciever arguments"),
            syn::FnArg::Typed(t) => t,
        })
        .collect()
}

fn arg_types<'a>(args: &Vec<&syn::PatType>) -> Vec<syn::Ident> {
    args.iter()
        .map(|f| {
            let ty = &f.ty;
            syn::Ident::new(&format!("{}", quote!(#ty)), f.span())
        })
        .collect()
}

fn main_struct<'a>(
    name: &'a syn::Ident,
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
) -> proc_macro::TokenStream {
    let arg_types = arg_types(&args);

    let arg_names = arg_names(&args);

    let arg_augmented: Vec<_> = arg_names.iter().map(|f| concat_ident(f, "FN")).collect();

    let names_with_m = marker_names(&arg_names);

    TokenStream::from(quote!(
        #[allow(non_camel_case_types,non_snake_case)]
        struct #name <#(#arg_names, #arg_augmented,)*BODYFN>
        where
            #(#arg_augmented: FnOnce() -> #arg_types,)*
            BODYFN: FnOnce(#(#arg_types,)*) #ret_type,
        {
            #(#names_with_m: ::std::marker::PhantomData<#arg_names>,)*
            #(#arg_names: Option<#arg_augmented>,)*
            body: BODYFN,
        }
    ))
}
