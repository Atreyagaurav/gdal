use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use std::ptr;
use std::ptr::NonNull;

use gdal_sys::{
    GDALDEMProcessingOptions, GDALDEMProcessingOptionsFree, GDALDEMProcessingOptionsNew,
};

use crate::cpl::CslStringList;
use crate::errors;
use crate::utils::_last_null_pointer_err;

/// Payload for [`GDALDEMProcessing`]. Intended for internal use only.
pub struct GdalDEMProcessingOptions<'opts>(
    NonNull<GDALDEMProcessingOptions>,
    PhantomData<&'opts CslStringList>,
);

impl<'opts> GdalDEMProcessingOptions<'opts> {
    pub fn new(opts: &'opts CslStringList) -> errors::Result<Self> {
        let popts = unsafe { GDALDEMProcessingOptionsNew(opts.as_ptr(), ptr::null_mut()) };
        if popts.is_null() {
            return Err(_last_null_pointer_err("GDALDEMProcessingOptionsNew"));
        }
        Ok(Self(unsafe { NonNull::new_unchecked(popts) }, PhantomData))
    }

    pub fn as_ptr(&self) -> *const GDALDEMProcessingOptions {
        self.0.as_ptr()
    }
}

impl Drop for GdalDEMProcessingOptions<'_> {
    fn drop(&mut self) {
        unsafe { GDALDEMProcessingOptionsFree(self.0.as_ptr()) };
    }
}

/// DEM processor mode, to stringify and pass to [`gdal_sys::GDALDEMProcessing`].
#[derive(Debug, Clone, Copy)]
pub enum DemAlg {
    Aspect,
    ColorRelief,
    Hillshade,
    Roughness,
    Slope,
    Tpi,
    Tri,
}

impl Display for DemAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColorRelief => f.write_str("color-relief"),
            _ => {
                let s = format!("{self:?}").to_lowercase();
                f.write_str(&s)
            }
        }
    }
}

/// Slope and slope-related (aspect, hillshade) processing algorithms.
///
/// The literature suggests `ZevenbergenThorne` to be more suited to smooth landscapes,
/// whereas `Horn` performs better on rougher terrain.
#[derive(Debug, Clone, Copy)]
pub enum DemSlopeAlg {
    Horn,
    ZevenbergenThorne,
}

impl Display for DemSlopeAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{self:?}"))
    }
}

macro_rules! common_dem_options {
    () => {
        /// Specify which band in the input [`Dataset`][crate::Dataset] to read from.
        ///
        /// Defaults to the first band.
        pub fn with_input_band(&mut self, band: NonZeroUsize) -> &mut Self {
            self.input_band = Some(band);
            self
        }

        /// Fetch the specified input band to read from.
        pub fn input_band(&self) -> Option<NonZeroUsize> {
            self.input_band
        }

        /// Explicitly specify output raster format.
        ///
        /// This is equivalent to the `-of <format>` CLI flag accepted by many GDAL tools.
        ///
        /// The value of `format` must be the identifier of a driver supported by the runtime
        /// environment's GDAL library (e.g. `COG`, `JPEG`, `VRT`, etc.). A list of these identifiers
        /// is available from `gdalinfo --formats`:
        ///
        /// ```text
        /// ❯ gdalinfo --formats
        /// Supported Formats:
        ///   VRT -raster,multidimensional raster- (rw+v): Virtual Raster
        ///   DERIVED -raster- (ro): Derived datasets using VRT pixel functions
        ///   GTiff -raster- (rw+vs): GeoTIFF
        ///   COG -raster- (wv): Cloud optimized GeoTIFF generator
        ///   NITF -raster- (rw+vs): National Imagery Transmission Format
        /// ...
        /// ```
        ///
        pub fn with_output_format(&mut self, format: &str) -> &mut Self {
            self.output_format = Some(format.to_owned());
            self
        }

        /// Fetch the specified output format driver identifier.
        pub fn output_format(&self) -> Option<String> {
            self.output_format.clone()
        }

        /// Compute values at image edges.
        ///
        /// If true, causes interpolation of values at image edges or if a no-data value is found
        /// in the 3x3 processing window.
        pub fn with_compute_edges(&mut self, state: bool) -> &mut Self {
            self.compute_edges = state;
            self
        }

        /// Fetch the compute edges mode.
        pub fn compute_edges(&self) -> bool {
            self.compute_edges
        }

        /// Additional generic options to be included.
        pub fn with_additional_options(&mut self, extra_options: CslStringList) -> &mut Self {
            self.additional_options.extend(&extra_options);
            self
        }

        /// Fetch additional options.
        pub fn additional_options(&self) -> &CslStringList {
            &self.additional_options
        }

        /// Private utility to convert common options into [`CslStringList`] options.
        fn store_common_options_to(&self, opts: &mut CslStringList) {
            if self.compute_edges {
                opts.add_string("-compute_edges").unwrap();
            }

            if let Some(band) = self.input_band {
                opts.add_string("-b").unwrap();
                opts.add_string(&band.to_string()).unwrap();
            }

            if let Some(of) = &self.output_format {
                opts.add_string("-of").unwrap();
                opts.add_string(of).unwrap();
            }

            if !self.additional_options.is_empty() {
                opts.extend(&self.additional_options);
            }
        }
    };
}

pub(crate) use common_dem_options;
