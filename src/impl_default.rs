use glam::{
    I16Vec2, I16Vec3, I16Vec4, I64Vec2, I64Vec3, I64Vec4, IVec2, IVec3, IVec4, Mat2, Mat3, Mat3A,
    Mat4, U16Vec2, U16Vec3, U16Vec4, U64Vec2, U64Vec3, U64Vec4, UVec2, UVec3, UVec4, Vec2, Vec3,
    Vec3A, Vec4,
};

use crate::ByteIter;

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u16(&mut self) -> Option<u16> {
        self.next_u16_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i16(&mut self) -> Option<i16> {
        self.next_i16_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u32(&mut self) -> Option<u32> {
        self.next_u32_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i32(&mut self) -> Option<i32> {
        self.next_i32_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_f32(&mut self) -> Option<f32> {
        self.next_f32_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u64(&mut self) -> Option<u64> {
        self.next_u64_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i64(&mut self) -> Option<i64> {
        self.next_i64_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_f64(&mut self) -> Option<f64> {
        self.next_f64_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_char(&mut self) -> Option<char> {
        self.next_char_be()
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i16vec2(&mut self) -> Option<I16Vec2> {
        self.next_i16vec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_ivec2(&mut self) -> Option<IVec2> {
        self.next_ivec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i64vec2(&mut self) -> Option<I64Vec2> {
        self.next_i64vec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i16vec3(&mut self) -> Option<I16Vec3> {
        self.next_i16vec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_ivec3(&mut self) -> Option<IVec3> {
        self.next_ivec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i64vec3(&mut self) -> Option<I64Vec3> {
        self.next_i64vec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i16vec4(&mut self) -> Option<I16Vec4> {
        self.next_i16vec4_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_ivec4(&mut self) -> Option<IVec4> {
        self.next_ivec4_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_i64vec4(&mut self) -> Option<I64Vec4> {
        self.next_i64vec4_be()
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u16vec2(&mut self) -> Option<U16Vec2> {
        self.next_u16vec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_uvec2(&mut self) -> Option<UVec2> {
        self.next_uvec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u64vec2(&mut self) -> Option<U64Vec2> {
        self.next_u64vec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u16vec3(&mut self) -> Option<U16Vec3> {
        self.next_u16vec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_uvec3(&mut self) -> Option<UVec3> {
        self.next_uvec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u64vec3(&mut self) -> Option<U64Vec3> {
        self.next_u64vec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u16vec4(&mut self) -> Option<U16Vec4> {
        self.next_u16vec4_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_uvec4(&mut self) -> Option<UVec4> {
        self.next_uvec4_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_u64vec4(&mut self) -> Option<U64Vec4> {
        self.next_u64vec4_be()
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_vec2(&mut self) -> Option<Vec2> {
        self.next_vec2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_vec3(&mut self) -> Option<Vec3> {
        self.next_vec3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_vec3a(&mut self) -> Option<Vec3A> {
        self.next_vec3a_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_vec4(&mut self) -> Option<Vec4> {
        self.next_vec4_be()
    }
}

impl<'a, T: Iterator<Item = &'a u8>> ByteIter<'a, T> {
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_mat2(&mut self) -> Option<Mat2> {
        self.next_mat2_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_mat3(&mut self) -> Option<Mat3> {
        self.next_mat3_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_mat3a(&mut self) -> Option<Mat3A> {
        self.next_mat3a_be()
    }
    /// Use **BIG ENDIAN(be)** by default.
    pub fn next_mat4(&mut self) -> Option<Mat4> {
        self.next_mat4_be()
    }
}
