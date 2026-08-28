@group(0) @binding(0)
var<storage, read> globals: Globals;

@group(0) @binding(1)
var<storage, read> camera: Camera;

@group(0) @binding(2)
var<storage, read> objects: array<Object>;

struct GBufferOutput {
    @location(0) data: vec4<f32>,
    @location(1) color: vec4<f32>,
    @location(2) uv: vec2<f32>,
}

@fragment
fn fs_main(in: VertexOutput) -> GBufferOutput {
    var output: GBufferOutput;

    output.data = vec4<f32>(in.world_normal.xy, in.material);
    output.color = vec4<f32>(in.color.rgb, f32 (in.texture_id) + 0.1 * select(0.0, 1.0, in.world_normal.z >= 0));
    output.uv = in.uv;
    return output;
}