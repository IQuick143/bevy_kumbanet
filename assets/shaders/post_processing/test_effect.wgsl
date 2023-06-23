#import bevy_sprite::mesh2d_view_bindings
#import bevy_pbr::utils

@group(1) @binding(0)
var texture: texture_2d<f32>;

@group(1) @binding(1)
var our_sampler: sampler;

@fragment
fn fragment(
	@builtin(position) position: vec4<f32>,
	#import bevy_sprite::mesh2d_vertex_output
) -> @location(0) vec4<f32> {
	// uv is included in bevy_sprite::mesh2d_vertex_output
	//let uv = coords_to_viewport_uv(position.xy, view.viewport);
	return vec4<f32>(uv.x, uv.y, 0.0, 1.0);
}
