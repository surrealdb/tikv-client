// Copyright 2020 TiKV Project Authors. Licensed under Apache-2.0.

#![allow(clippy::large_enum_variant)]
#![allow(clippy::enum_variant_names)]

pub use protos::*;

// Proto messages cover types the client code never references directly
// (e.g. `Participant`, `StoreRecoverState`), so the regenerated bindings
// trigger `dead_code` warnings. The Makefile's `RUSTFLAGS=-Dwarnings`
// promotes those to errors, so suppress at the wrapper module — the
// allow stays scoped to the generated tree, and re-runs of
// `cargo run -p tikv-client-proto-build` don't have to inject the
// attribute themselves.
#[allow(clippy::doc_lazy_continuation, dead_code)]
mod protos {
    include!("generated/mod.rs");
}
