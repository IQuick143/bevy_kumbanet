#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

fn a(x: u32) -> f32 {
	var a = 1.0;
	if (x == 0u) {
		a = 0.5;
	}
	return a;
}

@fragment
fn fragment(
	@builtin(position) position: vec4<f32>,
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	var c = vec4<f32>(0.0,0.0,0.0,1.0); // SAMPLE_TEXTURE2D(_MainTex, sampler_MainTex, i.texcoord);
	let block_id = floor(position.xy / 8.0) * 8.0;
	let pixel_id = vec2<u32>(position.xy) % 8u;

	var value = vec4<f32>(0.0);
	for (var x = 0u; x < 8u; x++) {
		for (var y = 0u; y < 8u; y++) {
			let uv = coords_to_viewport_uv(vec2<f32>(f32(x),f32(y))+0.5+block_id, view.viewport);
			value += 
				textureSample(texture, our_sampler, vec2<f32>(uv.x, uv.y)) *
				cos(f32(pixel_id.x) / 8.0 * PI * (f32(x) + 0.5)) * 
				cos(f32(pixel_id.y) / 8.0 * PI * (f32(y) + 0.5));
		}
	}
	c = vec4<f32>(a(pixel_id.x) * a(pixel_id.y) * value.xyz / 4., c.a);

//	if ((u32(position.y) % 8u) == 0u) {
//		c = vec4(0.0, 1.0, 0.0, 1.0);
//	}

//	if ((u32(position.x) % 8u) == 0u) {
//		c = vec4(0.0, 0.0, 1.0, 1.0);
//	}

	return c;
}
