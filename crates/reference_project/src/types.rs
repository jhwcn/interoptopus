//! All supported type patterns.

use interoptopus::lang::rust::CTypeInfo;
use interoptopus::patterns::slice::FFISlice;
use interoptopus::patterns::string::CStrPointer;
use interoptopus::{callback, ffi_type};
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait Helper {}

impl Helper for u8 {}

/// Empty structs are only allowed as opaques.
#[ffi_type(opaque)]
pub struct Empty {}

#[ffi_type(opaque)]
#[allow(dead_code)]
pub struct Opaque {
    _internal: *const Vec3f32,
}

#[ffi_type]
pub struct Tupled(pub u8);

#[ffi_type(transparent)]
#[allow(dead_code)]
pub struct Transparent(Tupled);

#[ffi_type]
pub struct Generic<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: &'a T,
}

#[ffi_type(opaque)]
pub struct Generic2<T>
where
    T: CTypeInfo,
{
    pub x: T,
}

#[ffi_type(opaque, name = "Generic3")]
pub struct Generic3<T> {
    pub x: T,
}

#[ffi_type(opaque, name = "Generic4")]
pub struct Generic4<T>
where
    T: Helper,
{
    pub x: T,
}

#[ffi_type(name = "StructRenamed")]
pub struct StructRenamedXYZ {
    pub e: EnumRenamedXYZ,
}

#[ffi_type(skip(p))]
pub struct Phantom<'a, T>
where
    T: 'static,
    T: CTypeInfo,
{
    pub x: u32,
    pub p: PhantomData<&'a T>,
}

#[ffi_type]
#[derive(Copy, Clone, Default)]
pub struct Vec3f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
pub struct Array {
    pub data: [u8; 16],
}

#[ffi_type]
pub struct GenericArray<T>
where
    T: CTypeInfo,
{
    pub data: [T; 16],
}

// TODO
// #[ffi_type]
// #[repr(C)]
// pub struct ConstGenericArray<T, const N: usize> {
//     data: [T; N]
// }

/// Documented enum.
#[ffi_type]
pub enum EnumDocumented {
    /// Variant A.
    A,
    /// Variant B.
    B,
    /// Variant B.
    C,
}

#[ffi_type(name = "EnumRenamed")]
pub enum EnumRenamedXYZ {
    X,
}

/// Documented struct.
#[ffi_type]
pub struct StructDocumented {
    /// Documented field.
    pub x: f32,
}

#[ffi_type]
pub struct ExtraType<T> {
    pub x: T,
}

#[ffi_type]
pub struct UseAsciiStringPattern<'a> {
    pub ascii_string: CStrPointer<'a>,
}

/// This can also be used for the `class` pattern.
#[ffi_type(opaque)]
#[allow(unused)]
pub struct SomeContext {
    pub(crate) some_field: u32,
}

#[ffi_type]
pub struct Weird1<T: Clone>
where
    T: Copy + Copy,
{
    x: T,
}

#[ffi_type]
pub struct Weird2<'a, T: Clone, const N: usize>
where
    T: Copy + Copy + 'a,
    T: Debug,
{
    t: T,
    a: [T; N],
    r: &'a u8,
}

#[ffi_type(visibility(pblc = "public", prvt = "private"))]
pub struct Visibility1 {
    // Be conservative with naming since some languages don't like `public` as a field.
    pblc: u8,
    pub prvt: u8,
}

#[ffi_type(visibility(_all = "public"))]
pub struct Visibility2 {
    pblc1: u8,
    pblc2: u8,
}

#[ffi_type(packed)]
pub struct Packed1 {
    pub x: u8,
    pub y: u16,
}

#[ffi_type(packed)]
pub struct Packed2 {
    pub y: u16,
    pub x: u8,
}

// UNSUPPORTED FOR NOW - At least C# and Python seem to have issues doing this correctly.
// #[ffi_type(align = 2)]
// pub struct Aligned1 {
//     pub x: u8,
//     pub y: u16,
// }
//
// #[ffi_type(align = 64)]
// pub struct Aligned2 {
//     pub x: u8,
//     pub y: u16,
// }

#[ffi_type]
#[derive(Debug, Default, Clone, Copy)]
pub struct BooleanAlignment {
    pub a: i32,
    pub b: i16,
    pub c: i16,
    pub d: u8,
    pub e: u8,
    pub f: u8,
    pub g: u8,
    pub h: u8,
    pub i: u8,
    pub j: u8,
    pub k: u8,
    pub id: u64,
    pub is_valid: bool,
    pub datum: u64,
}

// Doesn't need annotations.
pub type Callbacku8u8 = extern "C" fn(u8) -> u8;

// This does not work since we can't express the for<'x> bounds in our CTypeInfo implementation.
// pub type CallbackFFISlice = extern "C" fn(FFISlice<u8>) -> u8;

callback!(CallbackFFISlice(slice: FFISlice<u8>) -> u8);

pub mod ambiguous1 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec1")]
    pub struct Vec {
        pub x: f32,
        pub y: f32,
    }

    #[ffi_type(name = "Status1")]
    pub enum Status {
        X = 1,
        Y = 2,
    }
}

pub mod ambiguous2 {
    use interoptopus::ffi_type;

    #[ffi_type(name = "Vec2")]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }

    #[ffi_type(name = "Status2")]
    pub enum Status {
        X = 100,
        Z = 200,
    }
}

pub mod common {
    use interoptopus::ffi_type;

    #[ffi_type(namespace = "common")]
    pub struct Vec {
        pub x: f64,
        pub z: f64,
    }
}

pub mod associated_types {
    use interoptopus::ffi_type;

    pub trait Helper {
        type X;
    }

    #[ffi_type]
    pub struct Chicken(u8);

    #[ffi_type]
    pub struct Cow(u16);

    impl Helper for Chicken {
        type X = Cow;
    }

    #[ffi_type]
    pub struct FieldsViaAssociatedType {
        pub x: <Chicken as Helper>::X,
    }
}