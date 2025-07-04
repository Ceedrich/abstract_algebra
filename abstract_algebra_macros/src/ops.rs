use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(operations))]
struct Operations(#[deluxe(flatten)] Vec<String>);

pub fn derive_operations(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    let Operations(ops) = deluxe::extract_attributes(&mut ast)
        .map_err(|e| e.to_string())
        .unwrap();
    let ops = Operators::from(ops);

    let mut stream = TokenStream::new();

    if ops.add {
        stream.extend(impl_op(&ast, Operation::add()));
        stream.extend(impl_op_assign(&ast, Operation::add()));
    }
    if ops.mul {
        stream.extend(impl_op(&ast, Operation::mul()));
        stream.extend(impl_op_assign(&ast, Operation::mul()));
    }

    stream.into()
}

fn impl_op(ast: &DeriveInput, op: Operation) -> proc_macro2::TokenStream {
    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let Operation {
        name, trait_, func, ..
    } = op;

    quote! {
        impl #impl_generics #trait_<& #struct_name #type_generics > for & #struct_name #type_generics #where_clause {
            type Output = #struct_name #type_generics;
            fn #func(self, rhs: & #struct_name #type_generics ) -> Self::Output {
                <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(self, rhs)
            }
        }
        impl #impl_generics #trait_<#struct_name #type_generics > for & #struct_name #type_generics #where_clause {
            type Output = #struct_name #type_generics;
            fn #func(self, rhs: #struct_name #type_generics ) -> Self::Output {
                <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(self, &rhs)
            }
        }
        impl #impl_generics #trait_<#struct_name #type_generics > for #struct_name #type_generics #where_clause {
            type Output = #struct_name #type_generics;
            fn #func(self, rhs: #struct_name #type_generics ) -> Self::Output {
                <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(&self, &rhs)
            }
        }
        impl #impl_generics #trait_<& #struct_name #type_generics > for #struct_name #type_generics #where_clause {
            type Output = #struct_name #type_generics;
            fn #func(self, rhs: & #struct_name #type_generics ) -> Self::Output {
                <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(&self, rhs)
            }
        }
    }
}

fn impl_op_assign(ast: &DeriveInput, op: Operation) -> proc_macro2::TokenStream {
    let struct_name = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    let Operation {
        name,
        trait_assign,
        func_assign,
        ..
    } = op;
    quote! {
        impl #impl_generics #trait_assign <& #struct_name #type_generics > for #struct_name #type_generics #where_clause {
            fn #func_assign(&mut self, rhs: & #struct_name #type_generics ) {
                *self = <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(&self, rhs)
            }
        }
        impl #impl_generics #trait_assign <#struct_name #type_generics > for #struct_name #type_generics #where_clause {
            fn #func_assign(&mut self, rhs: #struct_name #type_generics ) {
                *self = <#struct_name #type_generics as ::abstract_algebra::ops::BinaryOperation<#name, _, _>>::op(&self, &rhs)
            }
        }
    }
}

#[derive(Default)]
struct Operators {
    add: bool,
    mul: bool,
}

impl<T> From<T> for Operators
where
    T: IntoIterator<Item: AsRef<str>>,
{
    fn from(value: T) -> Self {
        let mut x = Self {
            add: false,
            mul: false,
        };
        for v in value.into_iter() {
            match v.as_ref() {
                "Multiplication" => x.mul = true,
                "Addition" => x.add = true,
                s => panic!("Operation {s} not supported"),
            }
        }
        x
    }
}

struct Operation {
    name: TokenStream,
    trait_: TokenStream,
    trait_assign: TokenStream,
    func_assign: TokenStream,
    func: TokenStream,
}

impl Operation {
    fn add() -> Self {
        Self {
            name: quote! { ::abstract_algebra::ops::Addition },
            trait_: quote! { ::core::ops::Add },
            trait_assign: quote! { ::core::ops::AddAssign },
            func: quote! { add },
            func_assign: quote! { add_assign },
        }
    }
    fn mul() -> Self {
        Self {
            name: quote! { ::abstract_algebra::ops::Multiplication },
            trait_: quote! { ::core::ops::Mul },
            trait_assign: quote! { ::core::ops::MulAssign },
            func: quote! {mul},
            func_assign: quote! {mul_assign},
        }
    }
}
