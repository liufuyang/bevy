use crate::{
    shader::{FieldBindType, FieldInfo, GetFieldBindType, Uniforms},
    texture::Texture,
};
use bevy_asset::Handle;
use bevy_core::bytes::Bytes;

static TRANSFORM_FIELD_INFOS: &[FieldInfo] = &[FieldInfo {
    name: "transform",
    uniform_name: "Transform",
    texture_name: "",
    sampler_name: "",
}];

impl Uniforms for bevy_transform::prelude::Transform {
    fn get_field_infos() -> &'static [FieldInfo] {
        TRANSFORM_FIELD_INFOS
    }

    fn get_shader_defs(&self) -> Option<Vec<String>> {
        None
    }
    fn get_field_bind_type(&self, name: &str) -> Option<FieldBindType> {
        match name {
            "transform" => self.value.get_bind_type(),
            _ => None,
        }
    }
    fn get_uniform_texture(&self, _name: &str) -> Option<Handle<Texture>> {
        None
    }

    fn write_uniform_bytes(&self, name: &str, buffer: &mut [u8]) {
        match name {
            "Transform" => self.value.write_bytes(buffer),
            _ => {}
        }
    }
    fn uniform_byte_len(&self, name: &str) -> usize {
        match name {
            "Transform" => self.value.byte_len(),
            _ => 0,
        }
    }
}