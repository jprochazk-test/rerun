// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/testing/datatypes/fuzzy.fbs"

#include <arrow/api.h>

#include "flattened_scalar.hpp"

namespace rr {
    namespace datatypes {
        std::shared_ptr<arrow::DataType> FlattenedScalar::to_arrow_datatype() {
            return arrow::struct_({
                arrow::field("value", arrow::float32(), false, nullptr),
            });
        }
    } // namespace datatypes
} // namespace rr
