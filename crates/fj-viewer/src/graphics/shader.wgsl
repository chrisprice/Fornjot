struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] normal: vec3<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] local_position: vec2<f32>;
};

struct Uniforms {
    transform: mat4x4<f32>;
    transform_normals: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[stage(vertex)]]
fn vertex(
    [[builtin(vertex_index)]] vertex_index: u32,
    [[location(0)]] position: vec3<f32>,
    [[location(1)]] normal: vec3<f32>,
    [[location(2)]] color: vec4<f32>,
)
    -> VertexOutput
{
    var out: VertexOutput;
    out.normal = (uniforms.transform_normals * vec4<f32>(normal, 0.0)).xyz;
    out.position = uniforms.transform * vec4<f32>(position, 1.0);
    // We use premultiplied alpha blending.
    out.color = vec4<f32>(color.rgb * color.a, color.a);

    var triangle_vertex_index: u32 = vertex_index % 6u;
    if (triangle_vertex_index == 0u || triangle_vertex_index == 4u) {
        out.local_position = vec2<f32>(1.0, 0.0);
    } else if (triangle_vertex_index == 1u || triangle_vertex_index == 3u) {
        out.local_position = vec2<f32>(-1.0, 0.0);
    } else if (triangle_vertex_index == 2u) {
        out.local_position = vec2<f32>(0.0, 1.0);
    } else if (triangle_vertex_index == 5u) {
        out.local_position = vec2<f32>(0.0, -1.0);
    }

    return out;
}

let pi: f32 = 3.14159265359;
let root_2: f32 = 1.41421356237;

[[stage(fragment)]]
fn frag_model(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let light = vec3<f32>(0.0, 0.0, -1.0);

    let angle = acos(dot(light, -in.normal));
    let f_angle = angle / (pi / 2.0);

    let f_normal = max(1.0 - f_angle, 0.0);

    let color = vec4<f32>(in.color.rgb * f_normal, in.color.a);

    return color;
}

[[stage(fragment)]]
fn frag_mesh(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0 - in.color.rgb, in.color.a);
}

[[stage(fragment)]]
fn frag_lines(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color.rgb, in.color.a);
}

[[stage(fragment)]]
fn frag_circles(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var sdf: f32 = step(length(in.local_position), 1.0 / root_2);
    return vec4<f32>(in.color.rgb, in.color.a * sdf);
}
