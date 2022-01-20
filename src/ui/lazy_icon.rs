use druid::widget::{Svg, SvgData};
use once_cell::unsync::Lazy;
use std::thread::LocalKey;

pub type LazyIcon = Lazy<SvgData>;

pub trait IconLoader {
    fn load(self) -> SvgData;
}

impl IconLoader for &'static str {
    fn load(self) -> SvgData {
        self.parse::<SvgData>().unwrap()
    }
}

pub trait IconExtensions {
    fn to_svg(&'static self) -> Svg;
}

impl IconExtensions for LocalKey<LazyIcon> {
    fn to_svg(&'static self) -> Svg {
        Svg::new(self.with(|icon| Lazy::force(icon).clone()))
    }
}
