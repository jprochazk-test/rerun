# NOTE: This file was autogenerated by re_types_builder; DO NOT EDIT.

from __future__ import annotations

from typing import TYPE_CHECKING, Any, Sequence, Union

import pyarrow as pa
from attrs import define, field

from .. import datatypes
from .._baseclasses import (
    BaseExtensionArray,
    BaseExtensionType,
)

__all__ = [
    "TranslationRotationScale3D",
    "TranslationRotationScale3DArray",
    "TranslationRotationScale3DArrayLike",
    "TranslationRotationScale3DLike",
    "TranslationRotationScale3DType",
]


@define
class TranslationRotationScale3D:
    """Representation of an affine transform via separate translation, rotation & scale."""

    translation: datatypes.Vec3D | None = field(default=None)
    """
    3D translation vector, applied last.
    """

    rotation: datatypes.Rotation3D | None = field(default=None)
    """
    3D rotation, applied second.
    """

    scale: datatypes.Scale3D | None = field(default=None)
    """
    3D scale, applied first.
    """

    from_parent: bool | None = field(default=None)
    """
    If true, the transform maps from the parent space to the space where the transform was logged.
    Otherwise, the transform maps from the space to its parent.
    """


if TYPE_CHECKING:
    TranslationRotationScale3DLike = TranslationRotationScale3D

    TranslationRotationScale3DArrayLike = Union[
        TranslationRotationScale3D,
        Sequence[TranslationRotationScale3DLike],
    ]
else:
    TranslationRotationScale3DLike = Any
    TranslationRotationScale3DArrayLike = Any


# --- Arrow support ---


class TranslationRotationScale3DType(BaseExtensionType):
    def __init__(self) -> None:
        pa.ExtensionType.__init__(
            self,
            pa.struct(
                [
                    pa.field("translation", pa.list_(pa.field("item", pa.float32(), False, {}), 3), True, {}),
                    pa.field(
                        "rotation",
                        pa.dense_union(
                            [
                                pa.field(
                                    "Quaternion", pa.list_(pa.field("item", pa.float32(), False, {}), 4), False, {}
                                ),
                                pa.field(
                                    "AxisAngle",
                                    pa.struct(
                                        [
                                            pa.field(
                                                "axis",
                                                pa.list_(pa.field("item", pa.float32(), False, {}), 3),
                                                False,
                                                {},
                                            ),
                                            pa.field(
                                                "angle",
                                                pa.dense_union(
                                                    [
                                                        pa.field("Radians", pa.float32(), False, {}),
                                                        pa.field("Degrees", pa.float32(), False, {}),
                                                    ]
                                                ),
                                                False,
                                                {},
                                            ),
                                        ]
                                    ),
                                    False,
                                    {},
                                ),
                            ]
                        ),
                        True,
                        {},
                    ),
                    pa.field(
                        "scale",
                        pa.dense_union(
                            [
                                pa.field("ThreeD", pa.list_(pa.field("item", pa.float32(), False, {}), 3), False, {}),
                                pa.field("Uniform", pa.float32(), False, {}),
                            ]
                        ),
                        True,
                        {},
                    ),
                    pa.field("from_parent", pa.bool_(), True, {}),
                ]
            ),
            "rerun.datatypes.TranslationRotationScale3D",
        )


class TranslationRotationScale3DArray(BaseExtensionArray[TranslationRotationScale3DArrayLike]):
    _EXTENSION_NAME = "rerun.datatypes.TranslationRotationScale3D"
    _EXTENSION_TYPE = TranslationRotationScale3DType

    @staticmethod
    def _native_to_pa_array(data: TranslationRotationScale3DArrayLike, data_type: pa.DataType) -> pa.Array:
        raise NotImplementedError


TranslationRotationScale3DType._ARRAY_TYPE = TranslationRotationScale3DArray

# TODO(cmc): bring back registration to pyarrow once legacy types are gone
# pa.register_extension_type(TranslationRotationScale3DType())