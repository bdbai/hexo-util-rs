#![deny(clippy::all)]
#[macro_use]
extern crate napi_derive;

#[cfg(all(
  not(all(target_os = "linux", target_env = "musl", target_arch = "aarch64")),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc_rust::GlobalMiMalloc = mimalloc_rust::GlobalMiMalloc;

mod is_external_link;
mod encode_url;
mod slugize;
mod strip_html;
