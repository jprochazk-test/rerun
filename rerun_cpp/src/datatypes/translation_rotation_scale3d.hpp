// NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.
// Based on "crates/re_types/definitions/rerun/datatypes/translation_rotation_scale3d.fbs"

#pragma once

#include <cstdint>
#include <memory>
#include <optional>

#include "../datatypes/rotation3d.hpp"
#include "../datatypes/scale3d.hpp"
#include "../datatypes/vec3d.hpp"

namespace arrow {
    class DataType;
}

namespace rr {
    namespace datatypes {
        /// Representation of an affine transform via separate translation, rotation & scale.
        struct TranslationRotationScale3D {
            /// 3D translation vector, applied last.
            std::optional<rr::datatypes::Vec3D> translation;

            /// 3D rotation, applied second.
            std::optional<rr::datatypes::Rotation3D> rotation;

            /// 3D scale, applied first.
            std::optional<rr::datatypes::Scale3D> scale;

            /// If true, the transform maps from the parent space to the space where the transform
            /// was logged. Otherwise, the transform maps from the space to its parent.
            bool from_parent;

          public:
            /// Returns the arrow data type this type corresponds to.
            static std::shared_ptr<arrow::DataType> to_arrow_datatype();
        };
    } // namespace datatypes
} // namespace rr
