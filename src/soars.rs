use crate::{AsSoaRef, SoaArray, SoaDeref, SoaRaw};

/// Provides [`Soa`] compatibility.
///
/// # Safety
///
/// [`Soars::Deref`] mut be `#[repr(transparent)]` with [`Slice<Self::Raw>`].
/// This trait should be derived using the derive macro.
///
/// [`Slice<Self::Raw>`]: crate::Slice
/// [`Soa`]: crate::Soa
pub unsafe trait Soars: AsSoaRef<Item = Self> {
    /// Implements internal, unsafe, low-level routines used by [`Soa`]
    ///
    /// [`Soa`]: crate::Soa
    type Raw: SoaRaw<Item = Self>;

    /// [`Slice`] dereferences to this type to provide getters for the individual
    /// fields as slices.
    ///
    /// [`Slice`]: crate::Slice
    type Deref: ?Sized + SoaDeref<Item = Self>;

    /// A reference to an element of a [`Slice`].
    ///
    /// For each field with type `T`, this type has a field with type `&T`.
    ///
    /// [`Slice`]: crate::Slice
    type Ref<'a>: Copy + Clone + AsSoaRef<Item = Self>
    where
        Self: 'a;

    /// A reference to an element of a [`Slice`].
    ///
    /// For each field with type `T`, this type has a field with type `&mut T`.
    ///
    /// [`Slice`]: crate::Slice
    type RefMut<'a>: AsSoaRef<Item = Self>
    where
        Self: 'a;

    /// The SoA array type.
    ///
    /// For each field with type `T`, this type has a field with type `[T; N]`.
    type Array<const N: usize>: SoaArray<Item = Self>;

    /// The slices that make up a [`Slice`].
    ///
    /// For each field with type `T`, this type has a field with type `&[T]`.
    ///
    /// [`Slice`]: crate::Slice
    type Slices<'a>
    where
        Self: 'a;

    /// The mutable slices that make up a [`Slice`].
    ///
    /// For each field with type `T`, this type has a field with type `&mut [T]`.
    ///
    /// [`Slice`]: crate::Slice
    type SlicesMut<'a>
    where
        Self: 'a;
}
