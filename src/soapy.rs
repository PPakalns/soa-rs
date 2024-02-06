use crate::{SoaRaw, WithRef};

/// Provides SOA data structure compatibility.
///
/// This trait should be derived using the `soapy-derive` crate.
pub unsafe trait Soapy: Sized {
    /// Implements internal, unsafe, low-level routines used by `Soa`
    type Raw: SoaRaw<Item = Self>;

    /// Soa dereferences to this type to provide getters for the individual
    /// fields as slices.
    ///
    /// # Safety
    ///
    /// Must be `#[repr(transparent)]` with a [`SliceData<Self::Raw>`].
    ///
    /// [`SliceData<Self::Raw>`]: crate::SliceData
    type Deref;

    /// For each field with type `F` in `T`, `Ref` has a field with type
    /// `&F`
    type Ref<'a>: WithRef<Item = Self>
    where
        Self: 'a;

    /// For each field with type `F` in `T`, `RefMut` has a field with type
    /// `&mut F`
    type RefMut<'a>: WithRef<Item = Self>
    where
        Self: 'a;
}