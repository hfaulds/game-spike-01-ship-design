#import bevy_sprite::mesh2d_view_bindings
#import bevy_sprite::mesh2d_bindings

fn grid(uv: vec2<f32>, space: f32, gridWidth: f32) -> f32 {
	let p = uv - vec2<f32>(0.5);

	let size = vec2<f32>(gridWidth);
	let a1 = ((p - size) % (space));
	let a2 = ((p + size) % (space));

	let a = a2 - a1;
	let g = min(a.x, a.y);

	return 1.0 - clamp(g, 0., 1.);
}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let col = vec3<f32>(1.0);

    let pos = vec2<f32>(in.clip_position.x, in.clip_position.y);
    let alph = max(
          grid(pos, 20.0, 0.5),
          grid(pos, 100.0, 1.0)
    );
    return vec4<f32>(col, alph);
}
