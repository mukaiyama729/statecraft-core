#![no_std]
//! Riverpod-inspired reactive container implemented in no_std + alloc.
//!
//! One task owns a ProviderContainer. Share it across cores using message passing.
//! Do not treat ProviderContainer as Sync.
extern crate alloc;
#[cfg(test)]
extern crate std;

mod container;
mod dependency;
pub(crate) mod element;
mod provider;
mod reference;
mod subscription;
