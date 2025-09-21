use std::{
    fmt::{Display, Formatter, Result},
    ops::{Add, Deref, DerefMut, Div, Mul, Sub},
};

#[macro_export]
macro_rules! n {
    ($val:expr) => {
        AnyNum::from($val)
    };
}

#[derive(Debug, Clone, Copy)]
pub struct Num<T, const N: u8>(T);

impl<T, const N: u8> Deref for Num<T, N> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: u8> DerefMut for Num<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: Display, const N: u8> Display for Num<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.0)
    }
}
impl<T: Add<Output = T>, const N: u8> Add for Num<T, N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Num::<T, N>(self.0 + rhs.0)
    }
}
#[repr(u8)]
enum NumEnums {
    I8 = 0,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F32,
    F64,
}

type NumI8 = Num<i8, { NumEnums::I8 as u8 }>;
type NumI16 = Num<i16, { NumEnums::I16 as u8 }>;
type NumI32 = Num<i32, { NumEnums::I32 as u8 }>;
type NumI64 = Num<i64, { NumEnums::I64 as u8 }>;
type NumI128 = Num<i128, { NumEnums::I128 as u8 }>;

type NumU8 = Num<u8, { NumEnums::U8 as u8 }>;
type NumU16 = Num<u16, { NumEnums::U16 as u8 }>;
type NumU32 = Num<u32, { NumEnums::U32 as u8 }>;
type NumU64 = Num<u64, { NumEnums::U64 as u8 }>;
type NumU128 = Num<u128, { NumEnums::U128 as u8 }>;

type NumF32 = Num<f32, { NumEnums::F32 as u8 }>;
type NumF64 = Num<f64, { NumEnums::F64 as u8 }>;

pub enum AnyNum {
    I8(NumI8),
    I16(NumI16),
    I32(NumI32),
    I64(NumI64),
    I128(NumI128),
    U8(NumU8),
    U16(NumU16),
    U32(NumU32),
    U64(NumU64),
    U128(NumU128),
    F32(NumF32),
    F64(NumF64),
}

impl Display for AnyNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AnyNum::I8(x) => write!(f, "{x}"),
            AnyNum::I16(x) => write!(f, "{x}"),
            AnyNum::I32(x) => write!(f, "{x}"),
            AnyNum::I64(x) => write!(f, "{x}"),
            AnyNum::I128(x) => write!(f, "{x}"),
            AnyNum::U8(x) => write!(f, "{x}"),
            AnyNum::U16(x) => write!(f, "{x}"),
            AnyNum::U32(x) => write!(f, "{x}"),
            AnyNum::U64(x) => write!(f, "{x}"),
            AnyNum::U128(x) => write!(f, "{x}"),
            AnyNum::F32(x) => write!(f, "{x}"),
            AnyNum::F64(x) => write!(f, "{x}"),
        }
    }
}

impl PartialEq for AnyNum {
    fn eq(&self, other: &Self) -> bool {
        // todo!(); // implement partial eq for cross type and same type with proc macros
        match (self, other) {
            (AnyNum::U32(x), AnyNum::U32(y)) => x.0 == y.0,
            _ => true,
        }
    }
}

impl From<u32> for AnyNum {
    fn from(value: u32) -> Self {
        // todo!(); // implement proc macro for each variant's from implementation
        AnyNum::U32(Num::<u32, { NumEnums::U32 as u8 }>(value))
    }
}

impl Add for AnyNum {
    type Output = AnyNum;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (AnyNum::U32(a), AnyNum::U32(b)) => AnyNum::U32(a + b),
            _ => AnyNum::U8(Num(0)),
        }
    }
}

//TODO: IMPLEMENT SUB, MUL, DIV, POW
