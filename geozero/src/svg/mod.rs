//! SVG conversions.
mod svg;

pub use svg::*;

pub(crate) mod conversion {
    use super::svg::*;
    use crate::error::Result;
    use crate::FeatureProcessor;
    use crate::{GeozeroDatasource, GeozeroDatasourceReader, GeozeroGeometry};
    use std::io::Read;

    /// Convert to SVG.
    ///
    /// # Usage example:
    ///
    /// Convert a geo-types `Polygon` to an SVG document:
    ///
    /// ```
    /// use geozero::ToSvg;
    /// use geo_types::polygon;
    ///
    /// let geom: geo_types::Geometry<f64> = polygon![
    ///     (x: 220., y: 10.),
    ///     (x: 300., y: 210.),
    ///     (x: 170., y: 250.),
    ///     (x: 123., y: 234.),
    /// ]
    /// .into();
    ///
    /// println!("{}", &geom.to_svg_document().unwrap());
    /// ```
    pub trait ToSvg {
        /// Convert to SVG geometry.
        fn to_svg(&self) -> Result<String>;
        /// Convert to SVG document.
        fn to_svg_document(&self) -> Result<String>;
    }

    impl<T: GeozeroGeometry> ToSvg for T {
        fn to_svg(&self) -> Result<String> {
            let mut svg_data: Vec<u8> = Vec::new();
            let mut svg = SvgWriter::new(&mut svg_data, false);
            self.process_geom(&mut svg)?;
            String::from_utf8(svg_data).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
        fn to_svg_document(&self) -> Result<String> {
            let mut svg_data: Vec<u8> = Vec::new();
            let mut svg = SvgWriter::new(&mut svg_data, false);
            // svg.set_dimensions(bbox.get(0), bbox.get(1), bbox.get(2), bbox.get(3), 800, 400);
            svg.dataset_begin(None)?;
            svg.feature_begin(0)?;
            self.process_geom(&mut svg)?;
            svg.feature_end(0)?;
            svg.dataset_end()?;
            String::from_utf8(svg_data).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
    }

    /// Consume features as SVG.
    pub trait ProcessToSvg {
        /// Consume features as SVG String.
        fn to_svg(&mut self) -> Result<String>;
    }

    impl<T: GeozeroDatasource> ProcessToSvg for T {
        fn to_svg(&mut self) -> Result<String> {
            let mut svg_data: Vec<u8> = Vec::new();
            let mut svg = SvgWriter::new(&mut svg_data, false);
            self.process(&mut svg)?;
            String::from_utf8(svg_data).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
    }

    /// Read features as SVG.
    pub trait ReadAsSvg {
        /// Consume features as SVG String.
        fn read_as_svg<R: Read>(reader: R) -> Result<String>;
    }

    impl<T: GeozeroDatasourceReader> ReadAsSvg for T {
        fn read_as_svg<R: Read>(reader: R) -> Result<String> {
            let mut svg_data: Vec<u8> = Vec::new();
            let mut svg = SvgWriter::new(&mut svg_data, false);
            T::read(reader, &mut svg)?;
            String::from_utf8(svg_data).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
    }
}
