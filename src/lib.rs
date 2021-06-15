use syn::*;
use quote::*;
use proc_macro::TokenStream;

fn is_field_type_of(f: &Field, target: &str) -> bool {
    if let Type::Path(ref ty) = f.ty {
        if let Some(v) = ty.path.segments.first() {
            return v.ident.to_string().contains(target);
        }
    }
    false
}

fn is_type_of(ty: &Type, target: &str) -> bool {
    if let Type::Path(ref ty) = ty {
        if let Some(v) = ty.path.segments.first() {
            return v.ident.to_string().contains(target);
        }
    }
    false
}

fn get_type_under_option(f: &Field) -> Option<&Type> {
    if is_field_type_of(f, "Option") {
        if let Type::Path(ref tp) = f.ty {
            if let PathArguments::AngleBracketed(ref ag) = tp.path.segments.first().unwrap().arguments {
                if let GenericArgument::Type(ref ty) = ag.args.first().unwrap() {
                    return Some(ty);
                }
            }
        }
    }
    None
}

#[proc_macro_derive(Builder)]
pub fn test(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    let in_ident = &input.ident;
    let b_name = format_ident!("{}Builder", input.ident.to_string());

    let nf: FieldsNamed;
    if let Data::Struct(ref v) = input.data {
        if let Fields::Named(ref v) = v.fields {
            nf = v.clone();
        } else {
            panic!("_");
        }
    } else {
        panic!("You can only apply builder pattern to struct");
    }

    let fields = nf.named.iter().map(
        |f| {
            let ident = f.ident.as_ref().unwrap();
            let ty = &f.ty;
            if is_field_type_of(f, "Option") {
                quote! { #ident: #ty, }
            } else {
                quote! { #ident: std::option::Option<#ty>, }
            }
        }
    );
    let methods = nf.named.iter().cloned().map(
        |f| {
            if is_field_type_of(&f, "bool") || {
                if let Some(ty) = get_type_under_option(&f) {
                    is_type_of(ty, "bool")
                } else {
                    false
                }
            } {
                f.ident.unwrap()
            } else {
                format_ident!("with_{}", f.ident.unwrap().to_string())
            }
        }
    ).zip(nf.named.iter().cloned()).map(|e| {
        let func_name = e.0;
        let field_name = e.1.ident.as_ref().unwrap();
        let field_type = if let Some(ty) = get_type_under_option(&e.1) { ty } else { &e.1.ty };
        quote! {
            #[inline]
            pub fn #func_name(mut self, mut #field_name: #field_type) -> Self {
                self.#field_name = std::option::Option::Some(#field_name);
                self
            }
        }
    });

    let generics = input.generics.params;
    let where_clause = input.generics.where_clause;

    let assign = nf.named.iter().map(
        |f| {
            let name = f.ident.as_ref().unwrap();
            let expect_message = format!("{} cannot be None.", name.to_string());
            let value = if is_field_type_of(f, "Option") {
                quote! { self.#name }
            } else {
                quote! { self.#name.expect(#expect_message) }
            };
            quote! {
                #name: #value,
            }
        }
    );

    let build = quote! {
        #[inline]
        pub fn build(self) -> #in_ident<#generics> #where_clause {
            #in_ident::<#generics> {
                #(#assign)*
            }
        }
    };

    let out = quote! {
        #[derive(Debug, Clone, Default)]
        pub struct #b_name<#generics> #where_clause {
            #(#fields)*
        }

        impl<#generics> #in_ident<#generics> #where_clause {
            #[inline]
            pub fn builder() -> #b_name<#generics> #where_clause {
                <#b_name<#generics>>::default()
            }
        }

        impl<#generics> #b_name<#generics> #where_clause {
            #(#methods)*
            #build
        }
    };
    out.into()
}
