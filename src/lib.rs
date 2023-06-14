extern crate proc_macro;
extern crate syn;
extern crate quote;


use quote::{quote, ToTokens};
use syn::{Expr, FnArg, ItemTrait, parse_macro_input, TraitItemFn};



///
/// # Description
/// A simple decorator to define a HTTP GET api.
/// # Arguments
///
/// * `args`: path
/// * `input`: The api function.
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// #[derive(Debug, Deserialize, Serialize)]
/// struct Detail{
///     name: String,
///     year: i32,
///     path: String,
///     data: Option<Value>
/// }
///
/// #[Api]
/// trait Api{
///     #[Get("/{id}/detail")]
///     fn detail(&self, id: i32) -> serde_json::Result<Detail>;
///
///     #[Post("/detail")]
///     fn detail_post(&self, id: i32) -> serde_json::Result<Detail>;
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Get( args:proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream{
    let item_member_fn = parse_macro_input!(input as TraitItemFn);
    let path = parse_macro_input!(args as Expr);
    let fn_sig = item_member_fn.sig;
    (quote!{
        #fn_sig{
            serde_json::from_value::<_>( self.get(format!(#path) ) )
        }
    }).into()
}

///
/// # Description
/// A simple decorator to define a HTTP POST api.
/// # Arguments
///
/// * `args`: path
/// * `input`: The api function.
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// #[derive(Debug, Deserialize, Serialize)]
/// struct Detail{
///     name: String,
///     year: i32,
///     path: String,
///     data: Option<Value>
/// }
///
/// #[Api]
/// trait Api{
///     #[Get("/{id}/detail")]
///     fn detail(&self, id: i32) -> serde_json::Result<Detail>;
///
///     #[Post("/detail")]
///     fn detail_post(&self, id: i32) -> serde_json::Result<Detail>;
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Post( args:proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream{
    let item_member_fn = parse_macro_input!(input as TraitItemFn);
    let path = parse_macro_input!(args as Expr);
    let fn_sig = item_member_fn.sig;
    let inputs = &fn_sig.inputs;
    let mut args = Vec::new();
    for i in inputs{
        if let FnArg::Typed(typed) = i{
            args.push(format!("\"{0}\":{{{0}}}", typed.pat.to_token_stream()) );
        }
    }
    let args = format!("{{{{{}}}}}", args.join(","));
    (quote!{
        #fn_sig{
            use serde_json;
            serde_json::from_value( self.post( format!(#path) , serde_json::from_str( format!(#args).as_str() )? ) )
        }
    }).into()
}


///
/// # Description
/// A simple decorator to define a HTTP REST api trait.
/// # Arguments
///
/// * `input`: The trait itself.
///
/// returns: TokenStream
///
/// # Examples
///
/// ```
/// #[derive(Debug, Deserialize, Serialize)]
/// struct Detail{
///     name: String,
///     year: i32,
///     path: String,
///     data: Option<Value>
/// }
///
/// #[Api]
/// trait Api{
///     #[Get("/{id}/detail")]
///     fn detail(&self, id: i32) -> serde_json::Result<Detail>;
///
///     #[Post("/detail")]
///     fn detail_post(&self, id: i32) -> serde_json::Result<Detail>;
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Api( _: proc_macro::TokenStream, input : proc_macro::TokenStream) -> proc_macro::TokenStream{
    let item_trait: ItemTrait = parse_macro_input!(input as ItemTrait);
    let trait_name = item_trait.ident;
    let vis = item_trait.vis;
    let items = item_trait.items.iter();

    (quote! {
        #vis trait #trait_name {
            #( #items )*
            fn get(&self, path: String) -> serde_json::Value;
            fn post(&self, path: String, data: serde_json::Value)  -> serde_json::Value;
        }

    }).into()
}

