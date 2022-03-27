pub mod bitflags;
pub mod command;
pub mod context;
pub mod enums;
pub mod feature;

#[cfg(feature = "gl10")]
mod gl10;
#[cfg(feature = "gl11")]
mod gl11;
#[cfg(feature = "gl12")]
mod gl12;
#[cfg(feature = "gl13")]
mod gl13;
#[cfg(feature = "gl14")]
mod gl14;
#[cfg(feature = "gl15")]
mod gl15;
#[cfg(feature = "gl20")]
mod gl20;
#[cfg(feature = "gl21")]
mod gl21;
#[cfg(feature = "gl30")]
mod gl30;
#[cfg(feature = "gl31")]
mod gl31;
#[cfg(feature = "gl32")]
mod gl32;
#[cfg(feature = "gl33")]
mod gl33;
#[cfg(feature = "gl40")]
mod gl40;
#[cfg(feature = "gl41")]
mod gl41;
#[cfg(feature = "gl42")]
mod gl42;
#[cfg(feature = "gl43")]
mod gl43;
#[cfg(feature = "gl44")]
mod gl44;
#[cfg(feature = "gl45")]
mod gl45;
#[cfg(feature = "gl46")]
mod gl46;
