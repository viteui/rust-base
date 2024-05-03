extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{self, Data};
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(MyDefault)]
pub fn my_default(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let id = ast.ident;

    if let Data::Struct(s) = ast.data {
        // 声明一个新的ast，用于动态构建字段赋值的token
        let mut field_ast = quote! {};

        // 这里就是要动态添加token的地方了，需要动态完成Self的字段赋值
        for (idx, f) in s.fields.iter().enumerate() {
            let (field_id, field_ty) = (&f.ident, &f.ty);

            if let Some(field_id) = field_id {
                // 对于命名字段，都需要添加 `#field_name: #field_type::default(),` 这样的代码
                field_ast.extend(quote! { #field_id: <#field_ty>::default(), });
            } else {
                let field_idx = syn::Index::from(idx);
                let fty = &f.ty;
                //对于匿名字段，都需要添加 `#field_idx: <#field_type as std::default::Default>::default(),` 这样的代码
                field_ast.extend(quote! { #field_idx: <#fty>::default(), });
            }
        }

        let result = quote! {
            impl std::default::Default for #id {
                fn default() -> Self {
                    Self { #field_ast }
                }
            }
        }
        .into();

        result
    } else {
        panic!("MyDefault derive macro must use in struct");
    }
}