#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

// Gold Noise ©2015 dcerisano@standard3d.com
// - based on the Golden Ratio
// - uniform normalized distribution
// - fastest static noise generator function (also runs at low precision)
// - use with indicated fractional seeding method. 

const PHI: f32 = 1.61803398874989484820459;  // Φ = Golden Ratio   

fn gold_noise(xy: vec2<f32>, seed: f32) -> f32 {
       return fract(tan(distance(xy*PHI, xy)*seed)*xy.x);
}

@fragment
fn fragment(
	@builtin(position) position: vec4<f32>,
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	let uv = coords_to_viewport_uv(position.xy, view.viewport);

	let C = textureSample(texture, our_sampler, vec2<f32>(uv.x, uv.y));

	let noise = vec3<f32>(gold_noise(position.xy, 1.1 + C.r), gold_noise(position.xy, 2.1 + C.g), gold_noise(position.xy, 3.1 + C.b)) - 0.5;
//	let threshold = vec3<f32>(0.1);

	let quant_step = 0.30;

	let colour = textureSample(texture, our_sampler, vec2<f32>(uv.x, uv.y)) - vec4<f32>(quant_step * noise, 0.0);

	return vec4<f32>(round(colour.rgb / quant_step) * quant_step, colour.a);
}
