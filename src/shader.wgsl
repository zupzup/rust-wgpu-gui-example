struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) rect: vec4<f32>,
    @location(3) border_color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) rect: vec4<f32>,
    @location(2) border_color: vec3<f32>,
}

@vertex
fn vertex(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.rect = model.rect;
    out.border_color = model.border_color;

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var border_width: f32 = 2.0;
    var top: f32 = in.rect[0];
    var left: f32 = in.rect[1];
    var bottom: f32 = in.rect[2];
    var right: f32 = in.rect[3];

    if (
    ((in.clip_position.x > left && in.clip_position.x < (left + border_width))
    || (in.clip_position.x > (right - border_width) && in.clip_position.x < right))
    || ((in.clip_position.y > top && in.clip_position.y < (top + border_width))
    || (in.clip_position.y > (bottom - border_width) && in.clip_position.y < bottom )))  {
        return vec4<f32>(in.border_color, 1.0);
    }
    return vec4<f32>(in.color, 1.0);
}
