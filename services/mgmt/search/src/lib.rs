#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![doc = "generated by AutoRust"]
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-tag")))]
pub use package_2020_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-tag")))]
pub use package_2020_03::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-08")]
pub mod package_2015_08;
#[cfg(all(feature = "package-2015-08", not(feature = "no-default-tag")))]
pub use package_2015_08::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2015-02")]
pub mod package_2015_02;
#[cfg(all(feature = "package-2015-02", not(feature = "no-default-tag")))]
pub use package_2015_02::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2021-04-preview")]
pub mod package_2021_04_preview;
#[cfg(all(feature = "package-2021-04-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2020-08-preview")]
pub mod package_2020_08_preview;
#[cfg(all(feature = "package-2020-08-preview", not(feature = "no-default-tag")))]
pub use package_2020_08_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "no-default-tag")))]
pub use package_2019_10_preview::{models, operations, operations::Client, operations::ClientBuilder, operations::Error};
