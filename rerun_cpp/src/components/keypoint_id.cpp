// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/components/keypoint_id.fbs"

#include <arrow/api.h>

#include "keypoint_id.hpp"

namespace rr {
    namespace components {
        std::shared_ptr<arrow::DataType> KeypointId::to_arrow_datatype() {
            return arrow::uint16();
        }
    } // namespace components
} // namespace rr
