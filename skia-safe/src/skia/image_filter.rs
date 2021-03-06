// TODO: complete the implementation

use crate::prelude::*;
use skia_bindings::{
    SkImageFilter,
    SkRefCntBase
};

pub type ImageFilter = RCHandle<SkImageFilter>;

impl NativeRefCountedBase for SkImageFilter {
    type Base = SkRefCntBase;
    fn ref_counted_base(&self) -> &Self::Base {
        &self._base._base._base
    }
}