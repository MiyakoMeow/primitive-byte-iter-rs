use glam::{
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A,
    Mat4, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3,
    Vec3A, Vec4,
};

use crate::ByteIter;

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_u16_ne(&mut self) -> Option<u16> {
        let mut values: [u8; core::mem::size_of::<u16>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(u16::from_ne_bytes(values))
    }
    pub fn next_i16_ne(&mut self) -> Option<i16> {
        let mut values: [u8; core::mem::size_of::<i16>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(i16::from_ne_bytes(values))
    }
    pub fn next_u32_ne(&mut self) -> Option<u32> {
        let mut values: [u8; core::mem::size_of::<u32>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(u32::from_ne_bytes(values))
    }
    pub fn next_i32_ne(&mut self) -> Option<i32> {
        let mut values: [u8; core::mem::size_of::<i32>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(i32::from_ne_bytes(values))
    }
    pub fn next_f32_ne(&mut self) -> Option<f32> {
        let mut values: [u8; core::mem::size_of::<f32>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(f32::from_ne_bytes(values))
    }
    pub fn next_u64_ne(&mut self) -> Option<u64> {
        let mut values: [u8; core::mem::size_of::<u64>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(u64::from_ne_bytes(values))
    }
    pub fn next_i64_ne(&mut self) -> Option<i64> {
        let mut values: [u8; core::mem::size_of::<i64>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(i64::from_ne_bytes(values))
    }
    pub fn next_f64_ne(&mut self) -> Option<f64> {
        let mut values: [u8; core::mem::size_of::<f64>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(f64::from_ne_bytes(values))
    }
    pub fn next_usize_ne(&mut self) -> Option<usize> {
        let mut values: [u8; core::mem::size_of::<usize>()] = Default::default();
        for val in values.iter_mut() {
            *val = self.next_u8()?;
        }
        Some(usize::from_ne_bytes(values))
    }
    pub fn next_char_ne(&mut self) -> Option<char> {
        char::from_u32(self.next_u32_ne()?)
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_i16vec2_ne(&mut self) -> Option<I16Vec2> {
        Some(I16Vec2::new(self.next_i16_ne()?, self.next_i16_ne()?))
    }
    pub fn next_ivec2_ne(&mut self) -> Option<IVec2> {
        Some(IVec2::new(self.next_i32_ne()?, self.next_i32_ne()?))
    }
    pub fn next_i64vec2_ne(&mut self) -> Option<I64Vec2> {
        Some(I64Vec2::new(self.next_i64_ne()?, self.next_i64_ne()?))
    }
    pub fn next_i16vec3_ne(&mut self) -> Option<I16Vec3> {
        Some(I16Vec3::new(
            self.next_i16_ne()?,
            self.next_i16_ne()?,
            self.next_i16_ne()?,
        ))
    }
    pub fn next_ivec3_ne(&mut self) -> Option<IVec3> {
        Some(IVec3::new(
            self.next_i32_ne()?,
            self.next_i32_ne()?,
            self.next_i32_ne()?,
        ))
    }
    pub fn next_i64vec3_ne(&mut self) -> Option<I64Vec3> {
        Some(I64Vec3::new(
            self.next_i64_ne()?,
            self.next_i64_ne()?,
            self.next_i64_ne()?,
        ))
    }
    pub fn next_i16vec4_ne(&mut self) -> Option<I16Vec4> {
        Some(I16Vec4::new(
            self.next_i16_ne()?,
            self.next_i16_ne()?,
            self.next_i16_ne()?,
            self.next_i16_ne()?,
        ))
    }
    pub fn next_ivec4_ne(&mut self) -> Option<IVec4> {
        Some(IVec4::new(
            self.next_i32_ne()?,
            self.next_i32_ne()?,
            self.next_i32_ne()?,
            self.next_i32_ne()?,
        ))
    }
    pub fn next_i64vec4_ne(&mut self) -> Option<I64Vec4> {
        Some(I64Vec4::new(
            self.next_i64_ne()?,
            self.next_i64_ne()?,
            self.next_i64_ne()?,
            self.next_i64_ne()?,
        ))
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_u16vec2_ne(&mut self) -> Option<U16Vec2> {
        Some(U16Vec2::new(self.next_u16_ne()?, self.next_u16_ne()?))
    }
    pub fn next_uvec2_ne(&mut self) -> Option<UVec2> {
        Some(UVec2::new(self.next_u32_ne()?, self.next_u32_ne()?))
    }
    pub fn next_u64vec2_ne(&mut self) -> Option<U64Vec2> {
        Some(U64Vec2::new(self.next_u64_ne()?, self.next_u64_ne()?))
    }
    pub fn next_u16vec3_ne(&mut self) -> Option<U16Vec3> {
        Some(U16Vec3::new(
            self.next_u16_ne()?,
            self.next_u16_ne()?,
            self.next_u16_ne()?,
        ))
    }
    pub fn next_uvec3_ne(&mut self) -> Option<UVec3> {
        Some(UVec3::new(
            self.next_u32_ne()?,
            self.next_u32_ne()?,
            self.next_u32_ne()?,
        ))
    }
    pub fn next_u64vec3_ne(&mut self) -> Option<U64Vec3> {
        Some(U64Vec3::new(
            self.next_u64_ne()?,
            self.next_u64_ne()?,
            self.next_u64_ne()?,
        ))
    }
    pub fn next_u16vec4_ne(&mut self) -> Option<U16Vec4> {
        Some(U16Vec4::new(
            self.next_u16_ne()?,
            self.next_u16_ne()?,
            self.next_u16_ne()?,
            self.next_u16_ne()?,
        ))
    }
    pub fn next_uvec4_ne(&mut self) -> Option<UVec4> {
        Some(UVec4::new(
            self.next_u32_ne()?,
            self.next_u32_ne()?,
            self.next_u32_ne()?,
            self.next_u32_ne()?,
        ))
    }
    pub fn next_u64vec4_ne(&mut self) -> Option<U64Vec4> {
        Some(U64Vec4::new(
            self.next_u64_ne()?,
            self.next_u64_ne()?,
            self.next_u64_ne()?,
            self.next_u64_ne()?,
        ))
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_vec2_ne(&mut self) -> Option<Vec2> {
        Some(Vec2::new(self.next_f32_ne()?, self.next_f32_ne()?))
    }
    pub fn next_vec3_ne(&mut self) -> Option<Vec3> {
        Some(Vec3::new(
            self.next_f32_ne()?,
            self.next_f32_ne()?,
            self.next_f32_ne()?,
        ))
    }
    pub fn next_vec3a_ne(&mut self) -> Option<Vec3A> {
        Some(Vec3A::new(
            self.next_f32_ne()?,
            self.next_f32_ne()?,
            self.next_f32_ne()?,
        ))
    }
    pub fn next_vec4_ne(&mut self) -> Option<Vec4> {
        Some(Vec4::new(
            self.next_f32_ne()?,
            self.next_f32_ne()?,
            self.next_f32_ne()?,
            self.next_f32_ne()?,
        ))
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    pub fn next_mat2_ne(&mut self) -> Option<Mat2> {
        Some(Mat2::from_cols(self.next_vec2_ne()?, self.next_vec2_ne()?))
    }
    pub fn next_mat3_ne(&mut self) -> Option<Mat3> {
        Some(Mat3::from_cols(
            self.next_vec3_ne()?,
            self.next_vec3_ne()?,
            self.next_vec3_ne()?,
        ))
    }
    pub fn next_mat3a_ne(&mut self) -> Option<Mat3A> {
        Some(Mat3A::from_cols(
            self.next_vec3a_ne()?,
            self.next_vec3a_ne()?,
            self.next_vec3a_ne()?,
        ))
    }
    pub fn next_mat4_ne(&mut self) -> Option<Mat4> {
        Some(Mat4::from_cols(
            self.next_vec4_ne()?,
            self.next_vec4_ne()?,
            self.next_vec4_ne()?,
            self.next_vec4_ne()?,
        ))
    }
}
