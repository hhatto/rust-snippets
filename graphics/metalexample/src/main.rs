extern crate image;
extern crate metal_rs as metal;

use metal::*;
use std::mem;
use image::{DynamicImage, save_buffer, ColorType};

const PROGRAM: &str = "#include <metal_stdlib>

using namespace metal;

struct Vertex {
	float4 position [[position]];
	float4 color;
};

vertex Vertex VertexShader(
	uint vertexID [[vertex_id]],
	device Vertex * vertices [[buffer(0)]]
) {
	return vertices[vertexID];
}

fragment float4 FragmentShader(Vertex in [[stage_in]]) {
	return in.color;
}";

fn main() {
    let device = Device::system_default();

    let options = CompileOptions::new();
    let library = device.new_library_with_source(PROGRAM, &options).expect("new library");

    let vs = library.get_function("VertexShader", None).expect("get_function(vertex)");
    let fs = library.get_function("FragmentShader", None).expect("get_function(fragment)");

    let rpld = RenderPipelineDescriptor::new();
    rpld.set_vertex_function(Some(&vs));
    rpld.set_fragment_function(Some(&fs));
    rpld.color_attachments().object_at(0).unwrap().set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    let rps = device.new_render_pipeline_state(&rpld).unwrap();

    let vbuf = {
        let vertex_data = [
             0.0f32, 0.75, 0.0, 1.0, 1.0, 0.0, 1.0, 1.0,
            -0.75,  -0.75, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0,
             0.75,  -0.75, 0.0, 1.0, 0.0, 0.5, 1.0, 1.0,
        ];

        device.new_buffer_with_data(
            unsafe { mem::transmute(vertex_data.as_ptr()) },
            (vertex_data.len() * mem::size_of::<f32>()) as u64,
            MTLResourceOptions::StorageModeManaged)
    };

    let td = TextureDescriptor::new();
    td.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    td.set_width(512);
    td.set_height(512);
    td.set_storage_mode(MTLStorageMode::Managed);

    let texture = device.new_texture(&td);

    let cq = device.new_command_queue();
    let cb = cq.new_command_buffer();

    let rpd = RenderPassDescriptor::new();
    rpd.color_attachments().object_at(0).unwrap().set_load_action(MTLLoadAction::Clear);
    rpd.color_attachments().object_at(0).unwrap().set_store_action(MTLStoreAction::Store);
    rpd.color_attachments().object_at(0).unwrap().set_clear_color(MTLClearColor::new(0.35, 0.65, 0.85, 0.5));
    rpd.color_attachments().object_at(0).unwrap().set_texture(Some(&texture));

    let rce = cb.new_render_command_encoder(&rpd);
    rce.set_render_pipeline_state(&rps);
    rce.set_vertex_buffer(0, 0, Some(&vbuf));
    rce.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
    rce.end_encoding();

    let bce = cb.new_blit_command_encoder();
    bce.synchronize_resource(&texture);
    bce.end_encoding();

    cb.commit();
    cb.wait_until_completed();

    let ref mut img = DynamicImage::new_rgba8(texture.width() as u32, texture.height() as u32);
    let bytes_per_row = 4 * texture.width();
    let region = MTLRegion{
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size: MTLSize {
            width: texture.width(),
            height: texture.height(),
            depth: 1,
        }
    };
    let buf = img.clone().into_rgba8();
    texture.get_bytes(
        unsafe { mem::transmute(buf.as_ptr()) },
        region, 0, bytes_per_row);
    let _ = save_buffer("triangle.png", &buf, texture.width() as u32, texture.height() as u32, ColorType::Rgba8);
}
