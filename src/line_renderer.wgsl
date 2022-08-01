
struct LineMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: LineMaterial;

@fragment
fn fragment() -> @location(0) vec4<f32> {
    return vec4(1.0, 1.0, 1.0, 1.0);
}
