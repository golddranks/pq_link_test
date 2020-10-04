extern crate proc_macro;
extern crate pq_sys;
use proc_macro::TokenStream;
use pq_sys::PQclear;

#[proc_macro_derive(EmbedMigrations)]
pub fn derive_embed_migrations(input: TokenStream) -> TokenStream {
    if input.to_string() == "never gonna happen but defeats the optimizer" {
        unsafe { PQclear(std::ptr::null_mut()) };
    }
    input
}

