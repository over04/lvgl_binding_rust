use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{Data, Field, Fields};

#[proc_macro_attribute]
pub fn lvgl_obj(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_struct = syn::parse_macro_input!(item as syn::DeriveInput);
    let struct_name = &input_struct.ident;
    let field_token = quote! { pub _lv_obj_ptr: *mut lvgl_sys::lv_obj_t };
    match &mut input_struct.data {
        Data::Struct(data) => match &mut data.fields {
            Fields::Named(fields) => {
                fields
                    .named
                    .push(Field::parse_named.parse2(field_token).unwrap());
            }
            _ => {
                return syn::Error::new_spanned(&input_struct, "can only be used on named structs")
                    .to_compile_error()
                    .into();
            }
        },
        _ => {
            return syn::Error::new_spanned(&input_struct, "can only be used on structs")
                .to_compile_error()
                .into();
        }
    }
    let expanded = quote! {
        impl lvgl_base::obj::LvObjPtr for #struct_name {
            fn as_ptr(&self) -> *mut lvgl_sys::lv_obj_t {
                self._lv_obj_ptr
            }
            fn as_mut(&mut self) -> *mut lvgl_sys::lv_obj_t {
                self._lv_obj_ptr
            }
        }

        impl lvgl_base::obj::LvObj for #struct_name {
            fn create(parent: &dyn lvgl_base::obj::LvObjPtr) -> Self {
                unsafe {
                    Self {
                        _lv_obj_ptr: lvgl_sys::lv_obj_create(parent.as_ptr()),
                    }
                }
            }

            fn from_raw(raw: *mut lvgl_sys::lv_obj_t) -> Self {
                Self { _lv_obj_ptr: raw }
            }
        }

        impl lvgl_base::obj::LvObjEvent for #struct_name {}
        impl lvgl_base::obj::LvObjEventData for #struct_name {}
    };
    quote! {
        #input_struct
        #expanded
    }
    .into()
}
