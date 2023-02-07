// lhs is reused as output
@group(0) @binding(0)
var<storage, read_write> lhs: array<f32>;

@group(0) @binding(1)
var<storage, read> rhs: array<f32>;

@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
     let len = arrayLength(&lhs);
     let i = global_invocation_id.x;

     if (i >= len) {
        return;
     }

     lhs[i] = lhs[i] + rhs[i];
}
