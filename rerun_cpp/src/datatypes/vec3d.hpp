// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/vec3d.fbs"

#pragma once

#include <cstdint>
#include <memory>

namespace arrow {
    class DataType;
}

namespace rr {
    namespace datatypes {
        /// A vector in 3D space.
        struct Vec3D {
            float xyz[3];

          public:
            /// Returns the arrow data type this type corresponds to.
            static std::shared_ptr<arrow::DataType> to_arrow_datatype();
        };
    } // namespace datatypes
} // namespace rr
