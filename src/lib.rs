use proc_macro::TokenStream;
use quote::*;
use syn::__private::TokenStream2;
use syn::*;

fn fix_raw_ident(ident: Ident) -> Ident {
    let s = ident.to_string();
    if s.contains('#') {
        Ident::new(s.split_once('#').unwrap().1, ident.span())
    } else {
        ident
    }
}

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
            if let PathArguments::AngleBracketed(ref ag) =
                tp.path.segments.first().unwrap().arguments
            {
                if let GenericArgument::Type(ref ty) = ag.args.first().unwrap() {
                    return Some(ty);
                }
            }
        }
    }
    None
}

fn get_fields(nf: &FieldsNamed) -> impl Iterator<Item = TokenStream2> + '_ {
    nf.named.iter().map(|f| {
        let ident = f.ident.as_ref().unwrap();
        let ty = &f.ty;
        if is_field_type_of(f, "Option") {
            quote! { #ident: #ty, }
        } else {
            quote! { #ident: ::core::option::Option<#ty>, }
        }
    })
}

fn get_methods(nf: &FieldsNamed) -> impl Iterator<Item = TokenStream2> + '_ {
    nf.named
        .iter()
        .cloned()
        .map(|f| {
            if is_field_type_of(&f, "bool") || {
                if let Some(ty) = get_type_under_option(&f) {
                    is_type_of(ty, "bool")
                } else {
                    false
                }
            } {
                f.ident.unwrap()
            } else {
                format_ident!("with_{}", fix_raw_ident(f.ident.unwrap()).to_string())
            }
        })
        .zip(nf.named.iter().cloned())
        .map(|e| {
            if to_skip(&e.1) {
                quote! {}
            } else {
                let func_name = e.0;
                let field_name = e.1.ident.as_ref().unwrap();
                let field_type = if let Some(ty) = get_type_under_option(&e.1) {
                    ty
                } else {
                    &e.1.ty
                };
                quote! {
                    #[inline]
                    pub fn #func_name(mut self, mut #field_name: #field_type) -> Self {
                        self.#field_name = ::core::option::Option::Some(#field_name);
                        self
                    }
                }
            }
        })
}

fn to_skip(f: &Field) -> bool {
    f.attrs.iter().any(|a| {
        a.path
            .segments
            .first()
            .unwrap()
            .ident
            .to_string()
            .contains("skip")
    })
}

fn get_default_value(f: &Field) -> TokenStream2 {
    if let Some(a) = f.attrs.iter().find(|a| {
        a.path
            .segments
            .first()
            .unwrap()
            .ident
            .to_string()
            .contains("default")
    }) {
        let tokens: TokenStream = a.clone().tokens.into();
        let value: Expr = parse(tokens).unwrap();
        return quote! { ::core::option::Option::Some#value };
    }
    return quote! { ::core::default::Default::default() };
}

#[proc_macro_derive(Builder, attributes(skip, default))]
pub fn test(ts: TokenStream) -> TokenStream {
    let input = parse_macro_input!(ts as DeriveInput);
    //eprintln!("{:#?}", input);
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

    let generics = input.generics.params;
    let where_clause = input.generics.where_clause;

    let assign = nf.named.iter().map(|f| {
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
    });

    let assignment = nf.named.iter().map(|f| {
        let id = f.ident.as_ref().unwrap();
        let val = get_default_value(f);
        quote! {
            #id: #val,
        }
    });

    let builder = quote! {
        #[inline]
        pub fn builder() -> #b_name<#generics> #where_clause {
            #b_name::<#generics> {
                #(#assignment)*
            }
        }
    };

    let methods = get_methods(&nf);
    let fields = get_fields(&nf);
    let out = quote! {
        #[derive(Debug, Clone, Default)]
        pub struct #b_name<#generics> #where_clause {
            #(#fields)*
        }

        impl<#generics> #in_ident<#generics> #where_clause {
            #builder
        }

        impl<#generics> #b_name<#generics> #where_clause {
            #(#methods)*

            #[inline]
            pub fn build(self) -> #in_ident<#generics> #where_clause {
                #in_ident::<#generics> {
                    #(#assign)*
                }
            }
        }
    };
    out.into()
}
