#![crate_name = "gdal"]
#![crate_type = "lib"]

//! [GDAL](http://gdal.org/) is a translator and processing library for various raster and vector geospatial data formats.
//!
//! This crate provides safe, idiomatic [Rust](http://www.rust-lang.org/) bindings for GDAL.
//!
//! ## Capabilities
//!
//! GDAL is an incredibly powerful library. For a general understanding of its capabilities,
//! a good place to get started is the [GDAL User-oriented documentation](https://gdal.org/user/index.html).
//! These features include:
//!
//! * Opening raster and vector file formats for reading/writing
//! * Translating between file formats
//! * Reading and writing metadata in raster and vector datasets
//! * Accessing raster bands and their metadata
//! * Reading and writing geospatial coordinate system and projection values
//! * Warping (resampling and re-projecting) between coordinate systems
//!
//! ## Usage
//!
//! This crate provides high-level, idiomatic Rust bindings for GDAL.
//! To do that, it uses [`gdal-sys`](gdal-sys) internally, a low-level interface to the GDAL C library,
//! which is generated using [`bindgen`](https://rust-lang.github.io/rust-bindgen/).
//! Using the `gdal-sys` crate directly is normally not needed, but it can be useful in order to call APIs that have not yet been exposed in `gdal`.
//!
//! Building this crate assumes a compatible version of GDAL is installed with the corresponding header files and shared libraries.
//! This repository includes pre-generated bindings for GDAL 2.4 through 3.5 (see the`gdal-sys/prebuilt-bindings` directory).
//! If you're compiling against a later version of GDAL, you can enable the `bindgen` feature flag to have new bindings generated on the fly.
//!
//! ## Show Me Code!
//!
//! To get you started with GDAL (without having to read the whole manual!),
//! take a look at the examples in the [`raster`](raster#example) and [`vector`](vector#example) modules,
//! but for the maximally impatient, here you go:
//!
//! ```rust, no_run
//! use gdal::{Dataset, Metadata};
//! # fn main() -> gdal::errors::Result<()> {
//! let ds = Dataset::open("fixtures/m_3607824_se_17_1_20160620_sub.tif")?;
//! println!("This {} is in '{}' has {} bands.", ds.driver().long_name(), ds.spatial_ref()?.name()?, ds.raster_count());
//! # Ok(())
//! # }
//! ```
//! ```text
//! This GeoTIFF is in 'NAD83 / UTM zone 17N' has 4 bands.
//! ```
//!
//! ## Data Model
//!
//! At the top level, GDAL uses the same data model to access both vector and raster data sets.
//! There are several shared data model constructs at this level, but the first ones to become
//! familiar with are [`Driver`], [`Dataset`], and [`Metadata`].
//!
//! ### Driver
//!
//! One of GDAL's major strengths is the vast number of data formats it's able to work with.
//! The GDAL Manual has a full list of available [raster](https://gdal.org/drivers/raster/index.html)
//! and [vector](https://gdal.org/drivers/vector/index.html) drivers.
//!
//! The [`Driver` API][Driver] provides the requisite access points for working GDAL's drivers.
//!
//! ### Dataset
//!
//! [`Dataset`] is the top-level container for accessing all data within a data set, whether raster or vector.
//! Some methods and traits on `Dataset` are shared between raster and vector datasets,
//! and (due to historical reasons) some associated functions are only applicable to one context or the other.
//! The [`raster`] and [`vector`] modules cover these specifics.
//!
//! ### Metadata
//!
//! Metadata in GDAL takes a number of forms, some of which are specific to purpose
//! (e.g. pixel interpretation, spatial reference system),
//! and other more general-purpose (e.g. acquisition date-time). The former will be covered in
//! relevant sections of the [`raster`] and [`vector`] modules, and the general-purpose data model
//! in the [`Metadata`] API.
//!

pub use version::version_info;

pub mod config;
pub mod cpl;
mod dataset;
mod driver;
pub mod errors;
mod gdal_major_object;
mod metadata;
pub mod programs;
pub mod raster;
pub mod spatial_ref;
#[cfg(test)]
pub mod test_utils;
mod utils;
pub mod vector;
pub mod version;
pub mod vsi;

pub use dataset::{
    Dataset, DatasetOptions, GdalOpenFlags, GeoTransform, GeoTransformEx, LayerIterator,
    LayerOptions, Transaction,
};
pub use driver::Driver;
pub use metadata::Metadata;

#[cfg(test)]
fn assert_almost_eq(a: f64, b: f64) {
    let f: f64 = a / b;
    assert!(f < 1.00001);
    assert!(f > 0.99999);
}
