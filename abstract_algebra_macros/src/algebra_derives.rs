use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, TypePath, parse_macro_input, parse_quote};

fn impl_trait(trait_: TypePath, ast: DeriveInput, impl_block: Option<TokenStream>) -> TokenStream {
    let struct_name = &ast.ident;

    let fields: Vec<_> = match ast.data {
        Data::Struct(data) => match data.fields {
            Fields::Named(f) => f.named.into_iter().collect(),
            Fields::Unnamed(f) => f.unnamed.into_iter().collect(),
            Fields::Unit => vec![],
        },
        Data::Enum(data) => data
            .variants
            .into_iter()
            .flat_map(|x| x.fields.into_iter())
            .collect(),
        Data::Union(data) => data.fields.named.into_iter().collect(),
    };

    let mut generics = ast.generics.clone();
    let where_clause = generics.make_where_clause();

    for ty in fields.iter().map(|f| f.ty.clone()) {
        where_clause.predicates.push(parse_quote! {
            #ty: #trait_
        });
    }

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics #trait_ for #struct_name #type_generics #where_clause {
            #impl_block
        }
    }
}

pub fn derive_algebra(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    #[rustfmt::skip]
    let marker_traits: Vec<TypePath> = vec![
        parse_quote!(abstract_algebra::ops::Associativity<abstract_algebra::ops::Addition>),
        parse_quote!(abstract_algebra::ops::Associativity<abstract_algebra::ops::Multiplication>),
        parse_quote!(abstract_algebra::ops::Commutativity<abstract_algebra::ops::Addition>),
        parse_quote!(abstract_algebra::ops::Commutativity<abstract_algebra::ops::Multiplication>),
        parse_quote!(abstract_algebra::structures::Integrality),
    ];
    // parse_quote!(abstract_algebra::ops::Identity<abstract_algebra::ops::Multiplication>),
    // parse_quote!(abstract_algebra::ops::Inverse<abstract_algebra::ops::Addition>),
    // parse_quote!(abstract_algebra::ops::Inverse<abstract_algebra::ops::Multiplication>),
    // parse_quote!(abstract_algebra::ops::BinOp<abstract_algebra::ops::Addition>),
    // parse_quote!(abstract_algebra::ops::BinOp<abstract_algebra::ops::Multiplication>),

    marker_traits
        .into_iter()
        .flat_map(|trait_| impl_trait(trait_, ast.clone(), None))
        .collect::<TokenStream>()
        .into()
}
