use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Ident, Lit, Meta, Token, Type, TypePath,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

mod ops;

#[proc_macro_derive(Operations, attributes(operations))]
pub fn derive_operations(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ops::derive_operations(input)
}

#[proc_macro_derive(Blub, attributes(blub))]
pub fn accessor(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let Some(attr) = ast.attrs.iter().find(|attr| attr.path().is_ident("blub")) else {
        panic!("blub is required")
    };

    let Attributes {
        accessor: Element { accessor, type_ },
        bin_op,
        id,
        inv,
        ring,
    } = Attributes::from_attr(attr).unwrap();

    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();

    let ident = ast.ident;

    let ring_impl = ring.map(|r| {
        quote! {
            impl #impl_generics #r for #ident #type_generics #where_clause {}
        }
    });

    quote! {
        #(
            impl #impl_generics #bin_op for #ident #type_generics #where_clause {
                fn op(&self, rhs: &Self) -> Self {
                    let res = <#type_ as #bin_op>::op(&self #accessor, &rhs #accessor);
                    Self::new(res)
                }
            }
        )*

        #(
            impl #impl_generics #id for #ident #type_generics #where_clause {
                fn id() -> Self {
                    let res = <#type_ as #id>::id();
                    Self::new(res)
                }
            }
        )*

        #(
            impl #impl_generics #inv for #ident #type_generics #where_clause {
                fn inv(&self) -> Self {
                    let res = <#type_ as #inv>::inv(&self #accessor);
                    Self::new(res)
                }
            }

        )*

        #ring_impl
    }
    .into()
}

struct Attributes {
    accessor: Element,
    bin_op: Vec<MathTrait>,
    inv: Vec<MathTrait>,
    id: Vec<MathTrait>,
    ring: Option<MathTrait>,
}

impl Attributes {
    fn from_attr(attr: &Attribute) -> syn::Result<Self> {
        let mut accessor = None;
        let mut bin_op = Vec::new();
        let mut inv = Vec::new();
        let mut id = Vec::new();
        let mut ring = None;
        for meta in attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)? {
            match meta {
                Meta::List(meta) if meta.path.is_ident("accessor") => {
                    accessor = Some(meta.parse_args()?);
                }
                Meta::List(meta) if meta.path.is_ident("bin_op") => {
                    let x =
                        meta.parse_args_with(Punctuated::<MathTrait, Token![,]>::parse_terminated)?;
                    bin_op = x.into_iter().collect();
                }
                Meta::List(meta) if meta.path.is_ident("inv") => {
                    let x =
                        meta.parse_args_with(Punctuated::<MathTrait, Token![,]>::parse_terminated)?;
                    inv = x.into_iter().collect();
                }
                Meta::List(meta) if meta.path.is_ident("id") => {
                    let x =
                        meta.parse_args_with(Punctuated::<MathTrait, Token![,]>::parse_terminated)?;
                    id = x.into_iter().collect();
                }
                Meta::List(meta) if meta.path.is_ident("ring") => {
                    ring = Some(meta.parse_args()?);
                }
                m => {
                    return Err(syn::Error::new(
                        m.span(),
                        format!("{} is not a valid field", m.path().to_token_stream()),
                    ));
                }
            }
        }

        Ok(Attributes {
            accessor: accessor.expect("Element is required"),
            bin_op,
            inv,
            id,
            ring,
        })
    }
}

impl Parse for Element {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let accessor = input.parse()?;
        let _: Token![:] = input.parse()?;
        let type_ = input.parse()?;
        Ok(Element { accessor, type_ })
    }
}

impl Parse for Accessor {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![.] = input.parse()?;
        Ok(if input.peek(Ident) {
            Self::Ident(input.parse()?)
        } else if input.peek(Lit) {
            Self::Literal(input.parse()?)
        } else {
            return Err(syn::Error::new(input.span(), "unexpected token"));
        })
    }
}

impl ToTokens for Accessor {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let x = match self {
            Self::Ident(x) => quote! { .#x },
            Self::Literal(x) => quote! { .#x},
        };
        tokens.extend(x);
    }
}

enum Accessor {
    Ident(Ident),
    Literal(Lit),
}

struct Element {
    accessor: Accessor,
    type_: Type,
}

struct MathTrait(TypePath);

impl Parse for MathTrait {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse().map(Self)
    }
}

impl ToTokens for MathTrait {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens);
    }
}
