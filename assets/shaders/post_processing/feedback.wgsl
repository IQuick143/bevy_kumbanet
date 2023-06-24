#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture_1: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler_1: sampler;

@group(1) @binding(2)
var texture_2: texture_2d<f32>;

@group(1) @binding(3)
var our_sampler_2: sampler;

struct Time {
	value: f32,
	pad_a: f32,
	pad_b: f32,
	pad_c: f32
}

@group(1) @binding(4)
var<uniform> time: Time;

@fragment
fn fragment(
	@builtin(position) position: vec4<f32>,
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
//	let uv = coords_to_viewport_uv(position.xy, view.viewport);

	let alfa = 0.420;
	let beta = 0.1;
	let omega = 0.05;
	let d_omega = 0.01;

	let col_1 = textureSample(texture_1, our_sampler_1, vec2<f32>(uv.x, uv.y)).rgb;
	let col_2 = textureSample(texture_2, our_sampler_2, vec2<f32>(uv.x, uv.y)).rgb;
	let A = ((1.0+beta) * col_1 - beta * col_2.gbr).rb;
	let duv = vec2<f32>(sin(omega * (1.0 + d_omega) * time.value) * A.x + 0.1 * A.y, cos(omega * time.value) * A.y + 0.1 * A.x);
	let colour = ((1.0 - alfa) * col_1 + alfa * textureSample(texture_2, our_sampler_2, vec2<f32>(uv.x, uv.y) + duv).rgb);

	return vec4<f32>(colour, 1.0);
}
