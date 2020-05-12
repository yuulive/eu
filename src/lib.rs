#![feature(proc_macro_diagnostic)]
extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn;
use syn::spanned::Spanned;

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

            let func_struct = main_struct(&predicate, &argument_vector, &func.sig.output);
            let generator_func = generator_func(
                &predicate,
                name,
                &argument_vector,
                &func.sig.output,
                &empty_unit,
                &func.block,
            );
            let unit_structs = quote! {
                #[allow(non_camel_case_types,non_snake_case)]
                struct #added_unit;
                #[allow(non_camel_case_types,non_snake_case)]
                struct #empty_unit;
            };

            let final_call =
                final_call(&predicate, &argument_vector, &func.sig.output, &added_unit);

            let argument_calls = argument_calls(
                &predicate,
                &argument_vector,
                &added_unit,
                &empty_unit,
                &func.sig.output,
            );

            // assemble output
            let mut out = proc_macro2::TokenStream::new();
            out.extend(unit_structs);
            out.extend(func_struct);
            out.extend(generator_func);
            out.extend(argument_calls);
            out.extend(final_call);
            TokenStream::from(out)
        }
        _ => {
            func_item
                .span()
                .unstable()
                .error(
                    "Only functions can be partially applied, for structs use the builder pattern",
                )
                .emit();
            let out: proc_macro::TokenStream = quote! { #func_item }.into();
            out
        }
    }
}

/// The portion of the signature necessary for each impl block
fn impl_signature<'a>(
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
) -> proc_macro2::TokenStream {
    let arg_names = arg_names(&args);
    let arg_types = arg_types(&args);
    let augmented_names = augmented_argument_names(&arg_names);

    quote! {
        #(#augmented_names: FnOnce() -> #arg_types,)*
        BODYFN: FnOnce(#(#arg_types,)*) #ret_type
    }
}

fn argument_calls<'a>(
    struct_name: &syn::Ident,
    args: &Vec<&syn::PatType>,
    unit_added: &syn::Ident,
    unit_empty: &syn::Ident,
    ret_type: &'a syn::ReturnType,
) -> proc_macro2::TokenStream {
    let impl_sig = impl_signature(args, ret_type);
    let arg_name_vec = arg_names(args);
    let aug_arg_names = augmented_argument_names(&arg_name_vec);
    arg_names(args)
        .into_iter()
        .zip(&aug_arg_names)
        .map(|(n, n_fn)| {
            // All variable names except the name of this function
            let free_vars: Vec<_> = arg_name_vec.iter().filter(|&x| x != &n).collect();
            let associated_vals_out: Vec<_> = arg_name_vec
                .iter()
                .map(|x| {
                    if &n == x {
                        unit_added.clone()
                    } else {
                        x.clone()
                    }
                })
                .collect();
            let associated_vals_in: Vec<_> = associated_vals_out
                .iter()
                .map(|x| if x == unit_added { unit_empty } else { x })
                .collect();
            quote! {
                #[allow(non_camel_case_types,non_snake_case)]
                impl< #impl_sig, #(#free_vars,)* >
                    #struct_name< #(#associated_vals_in, #aug_arg_names,)* BODYFN>
                {
                    fn #n (mut self, #n: #n_fn) ->
                        #struct_name< #(#associated_vals_out, #aug_arg_names,)* BODYFN>{
                        self.#n = Some(#n);
                        unsafe {
                            ::std::mem::transmute_copy::<
                                #struct_name<#(#associated_vals_in, #aug_arg_names,)* BODYFN>,
                            #struct_name<#(#associated_vals_out, #aug_arg_names,)* BODYFN>,
                            >(&self)
                        }
                    }
                }
            }
        })
        .collect()
}

fn final_call<'a>(
    struct_name: &syn::Ident,
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
    unit_added: &'a syn::Ident,
) -> proc_macro2::TokenStream {
    let impl_sig = impl_signature(args, ret_type);
    let arg_names = arg_names(args);
    let aug_args = augmented_argument_names(&arg_names);
    quote! {
        #[allow(non_camel_case_types,non_snake_case)]
        impl <#impl_sig>
            #struct_name<#(#unit_added, #aug_args,)* BODYFN>
        {
            fn call(self) #ret_type {
                (self.body)(#(self.#arg_names.unwrap()(),)*)
            }
        }
    }
}

/// The function called by the user to create an instance of a partially applicable function
fn generator_func<'a>(
    struct_name: &'a syn::Ident,
    name: &'a syn::Ident,
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
    empty_unit: &'a syn::Ident,
    body: &'a Box<syn::Block>,
) -> proc_macro2::TokenStream {
    let arg_names = arg_names(&args);
    let arg_types = arg_types(&args);
    let marker_names = marker_names(&arg_names);

    quote! {
        #[allow(non_camel_case_types,non_snake_case)]
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

    }
}

/// A vector of all argument names
fn arg_names<'a>(args: &Vec<&syn::PatType>) -> Vec<syn::Ident> {
    args.iter()
        .map(|f| {
            let f_pat = &f.pat;
            syn::Ident::new(&format!("{}", quote!(#f_pat)), f.span())
        })
        .collect()
}

/// returns the vector of names used to hold PhantomData
fn marker_names(names: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    names.iter().map(|f| concat_ident(f, "m")).collect()
}

/// Concatenates a identity with a string, returning a new identity with the same span.
fn concat_ident<'a>(ident: &'a syn::Ident, end: &str) -> syn::Ident {
    let name = format!("{}___{}", quote! {#ident}, end);
    syn::Ident::new(&name, ident.span())
}

/// Gets the name a of function.
fn get_name<'a>(func: &'a syn::ItemFn) -> &'a syn::Ident {
    // TODO: move this check somewhere else
    if let Some(r) = &func.sig.receiver() {
        r.span()
            .unstable()
            .error("Cannot make methods partially applicable yet")
            .emit();
    }
    &func.sig.ident
}

/// Filter an argument list to a pattern type
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

/// Retrieves the identities of an the argument list
fn arg_types<'a>(args: &Vec<&syn::PatType>) -> Vec<syn::Ident> {
    args.iter()
        .map(|f| {
            let ty = &f.ty;
            syn::Ident::new(&format!("{}", quote!(#ty)), f.span())
        })
        .collect()
}

fn augmented_argument_names<'a>(arg_names: &Vec<syn::Ident>) -> Vec<syn::Ident> {
    arg_names.iter().map(|f| concat_ident(f, "FN")).collect()
}

/// Generates the main struct for the partially applicable function.
/// All other functions are methods on this struct.
fn main_struct<'a>(
    name: &'a syn::Ident,
    args: &Vec<&syn::PatType>,
    ret_type: &'a syn::ReturnType,
) -> proc_macro2::TokenStream {
    let arg_types = arg_types(&args);

    let arg_names = arg_names(&args);

    let arg_augmented = augmented_argument_names(&arg_names);

    let names_with_m = marker_names(&arg_names);

    quote!(
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
    )
}
