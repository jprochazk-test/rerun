// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/re_types/definitions/rerun/datatypes/float32.fbs".

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// **Datatype**: A single-precision 32-bit IEEE 754 floating point number.
#[derive(Clone, Debug, Copy, PartialEq, PartialOrd)]
pub struct Float32(pub f32);

impl From<f32> for Float32 {
    #[inline]
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Float32> for f32 {
    #[inline]
    fn from(value: Float32) -> Self {
        value.0
    }
}

impl<'a> From<Float32> for ::std::borrow::Cow<'a, Float32> {
    #[inline]
    fn from(value: Float32) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a Float32> for ::std::borrow::Cow<'a, Float32> {
    #[inline]
    fn from(value: &'a Float32) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl ::re_types_core::Loggable for Float32 {
    type Name = ::re_types_core::DatatypeName;

    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.Float32".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Float32
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
    ) -> ::re_types_core::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        re_tracing::profile_function!();
        use ::arrow2::{array::*, datatypes::*};
        use ::re_types_core::{Loggable as _, ResultExt as _};
        Ok({
            let (somes, data0): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    let datum = datum.map(|datum| {
                        let Self(data0) = datum.into_owned();
                        data0
                    });
                    (datum.is_some(), datum)
                })
                .unzip();
            let data0_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            PrimitiveArray::new(
                Self::arrow_datatype(),
                data0.into_iter().map(|v| v.unwrap_or_default()).collect(),
                data0_bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn from_arrow_opt(
        arrow_data: &dyn ::arrow2::array::Array,
    ) -> ::re_types_core::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        re_tracing::profile_function!();
        use ::arrow2::{array::*, buffer::*, datatypes::*};
        use ::re_types_core::{Loggable as _, ResultExt as _};
        Ok(arrow_data
            .as_any()
            .downcast_ref::<Float32Array>()
            .ok_or_else(|| {
                ::re_types_core::DeserializationError::datatype_mismatch(
                    DataType::Float32,
                    arrow_data.data_type().clone(),
                )
            })
            .with_context("rerun.datatypes.Float32#value")?
            .into_iter()
            .map(|opt| opt.copied())
            .map(|v| v.ok_or_else(::re_types_core::DeserializationError::missing_data))
            .map(|res| res.map(|v| Some(Self(v))))
            .collect::<::re_types_core::DeserializationResult<Vec<Option<_>>>>()
            .with_context("rerun.datatypes.Float32#value")
            .with_context("rerun.datatypes.Float32")?)
    }
}
