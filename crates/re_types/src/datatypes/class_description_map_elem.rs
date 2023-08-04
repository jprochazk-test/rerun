// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

#![allow(trivial_numeric_casts)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::iter_on_single_items)]
#![allow(clippy::map_flatten)]
#![allow(clippy::match_wildcard_for_single_variants)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::unnecessary_cast)]

/// A helper type for mapping class IDs to class descriptions.
///
/// This is internal to the `AnnotationContext` structure.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ClassDescriptionMapElem {
    pub class_id: crate::components::ClassId,
    pub class_description: crate::datatypes::ClassDescription,
}

impl<'a> From<ClassDescriptionMapElem> for ::std::borrow::Cow<'a, ClassDescriptionMapElem> {
    #[inline]
    fn from(value: ClassDescriptionMapElem) -> Self {
        std::borrow::Cow::Owned(value)
    }
}

impl<'a> From<&'a ClassDescriptionMapElem> for ::std::borrow::Cow<'a, ClassDescriptionMapElem> {
    #[inline]
    fn from(value: &'a ClassDescriptionMapElem) -> Self {
        std::borrow::Cow::Borrowed(value)
    }
}

impl crate::Loggable for ClassDescriptionMapElem {
    type Name = crate::DatatypeName;
    type Item<'a> = Option<Self>;
    type Iter<'a> = Box<dyn Iterator<Item = Self::Item<'a>> + 'a>;
    #[inline]
    fn name() -> Self::Name {
        "rerun.datatypes.ClassDescriptionMapElem".into()
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    #[inline]
    fn to_arrow_datatype() -> arrow2::datatypes::DataType {
        use ::arrow2::datatypes::*;
        DataType::Struct(vec![
            Field {
                name: "class_id".to_owned(),
                data_type: <crate::components::ClassId>::to_arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
            Field {
                name: "class_description".to_owned(),
                data_type: <crate::datatypes::ClassDescription>::to_arrow_datatype(),
                is_nullable: false,
                metadata: [].into(),
            },
        ])
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_to_arrow_opt<'a>(
        data: impl IntoIterator<Item = Option<impl Into<::std::borrow::Cow<'a, Self>>>>,
        extension_wrapper: Option<&str>,
    ) -> crate::SerializationResult<Box<dyn ::arrow2::array::Array>>
    where
        Self: Clone + 'a,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let (somes, data): (Vec<_>, Vec<_>) = data
                .into_iter()
                .map(|datum| {
                    let datum: Option<::std::borrow::Cow<'a, Self>> = datum.map(Into::into);
                    (datum.is_some(), datum)
                })
                .unzip();
            let bitmap: Option<::arrow2::bitmap::Bitmap> = {
                let any_nones = somes.iter().any(|some| !*some);
                any_nones.then(|| somes.into())
            };
            StructArray::new(
                (if let Some(ext) = extension_wrapper {
                    DataType::Extension(
                        ext.to_owned(),
                        Box::new(<crate::datatypes::ClassDescriptionMapElem>::to_arrow_datatype()),
                        None,
                    )
                } else {
                    <crate::datatypes::ClassDescriptionMapElem>::to_arrow_datatype()
                })
                .to_logical_type()
                .clone(),
                vec![
                    {
                        let (somes, class_id): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self { class_id, .. } = &**datum;
                                    class_id.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_id_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        PrimitiveArray::new(
                            {
                                _ = extension_wrapper;
                                DataType::UInt16.to_logical_type().clone()
                            },
                            class_id
                                .into_iter()
                                .map(|datum| {
                                    datum
                                        .map(|datum| {
                                            let crate::components::ClassId(data0) = datum;
                                            data0
                                        })
                                        .unwrap_or_default()
                                })
                                .collect(),
                            class_id_bitmap,
                        )
                        .boxed()
                    },
                    {
                        let (somes, class_description): (Vec<_>, Vec<_>) = data
                            .iter()
                            .map(|datum| {
                                let datum = datum.as_ref().map(|datum| {
                                    let Self {
                                        class_description, ..
                                    } = &**datum;
                                    class_description.clone()
                                });
                                (datum.is_some(), datum)
                            })
                            .unzip();
                        let class_description_bitmap: Option<::arrow2::bitmap::Bitmap> = {
                            let any_nones = somes.iter().any(|some| !*some);
                            any_nones.then(|| somes.into())
                        };
                        {
                            _ = class_description_bitmap;
                            _ = extension_wrapper;
                            crate::datatypes::ClassDescription::try_to_arrow_opt(
                                class_description,
                                None::<&str>,
                            )?
                        }
                    },
                ],
                bitmap,
            )
            .boxed()
        })
    }

    #[allow(unused_imports, clippy::wildcard_imports)]
    fn try_from_arrow_opt(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Vec<Option<Self>>>
    where
        Self: Sized,
    {
        use crate::Loggable as _;
        use ::arrow2::{array::*, datatypes::*};
        Ok({
            let data = data
                .as_any()
                .downcast_ref::<::arrow2::array::StructArray>()
                .ok_or_else(|| crate::DeserializationError::DatatypeMismatch {
                    expected: data.data_type().clone(),
                    got: data.data_type().clone(),
                    backtrace: ::backtrace::Backtrace::new_unresolved(),
                })
                .map_err(|err| crate::DeserializationError::Context {
                    location: "rerun.datatypes.ClassDescriptionMapElem".into(),
                    source: Box::new(err),
                })?;
            if data.is_empty() {
                Vec::new()
            } else {
                let (data_fields, data_arrays, data_bitmap) =
                    (data.fields(), data.values(), data.validity());
                let is_valid = |i| data_bitmap.map_or(true, |bitmap| bitmap.get_bit(i));
                let arrays_by_name: ::std::collections::HashMap<_, _> = data_fields
                    .iter()
                    .map(|field| field.name.as_str())
                    .zip(data_arrays)
                    .collect();
                let class_id = {
                    let data = &**arrays_by_name["class_id"];

                    data.as_any()
                        .downcast_ref::<UInt16Array>()
                        .unwrap()
                        .into_iter()
                        .map(|opt| opt.map(|v| crate::components::ClassId(*v)))
                };
                let class_description = {
                    let data = &**arrays_by_name["class_description"];

                    crate::datatypes::ClassDescription::try_from_arrow_opt(data)
                        .map_err(|err| crate::DeserializationError::Context {
                            location: "rerun.datatypes.ClassDescriptionMapElem#class_description"
                                .into(),
                            source: Box::new(err),
                        })?
                        .into_iter()
                };
                :: itertools :: izip ! (class_id , class_description) . enumerate () . map (| (i , (class_id , class_description)) | is_valid (i) . then (|| Ok (Self { class_id : class_id . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.ClassDescriptionMapElem#class_id" . into () , source : Box :: new (err) , }

) ? , class_description : class_description . ok_or_else (|| crate :: DeserializationError :: MissingData { backtrace : :: backtrace :: Backtrace :: new_unresolved () , }

) . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.ClassDescriptionMapElem#class_description" . into () , source : Box :: new (err) , }

) ? , }

)) . transpose ()) . collect :: < crate :: DeserializationResult < Vec < _ >> > () . map_err (| err | crate :: DeserializationError :: Context { location : "rerun.datatypes.ClassDescriptionMapElem" . into () , source : Box :: new (err) , }

) ?
            }
        })
    }

    #[inline]
    fn try_iter_from_arrow(
        data: &dyn ::arrow2::array::Array,
    ) -> crate::DeserializationResult<Self::Iter<'_>>
    where
        Self: Sized,
    {
        Ok(Box::new(Self::try_from_arrow_opt(data)?.into_iter()))
    }

    #[inline]
    fn convert_item_to_self(item: Self::Item<'_>) -> Option<Self> {
        item
    }
}

impl crate::Datatype for ClassDescriptionMapElem {}
