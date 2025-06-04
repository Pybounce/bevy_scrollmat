#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_bindings::globals,
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct ScrollMatExtension {
    scroll_speed: vec2f,
}

@group(2) @binding(100)
var<uniform> scrollmat_extension: ScrollMatExtension;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var in2 = in;
    //in2.uv += (scrollmat_extension.scroll_speed * (globals.time % 1.0)) % 1.0;
    in2.uv += (globals.time * scrollmat_extension.scroll_speed) % 1.0;

    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in2, is_front);


    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    // in deferred mode we can't modify anything after that, as lighting is run in a separate fullscreen shader.
    var out = deferred_output(in2, pbr_input);
#else
    var out: FragmentOutput;
    // apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // apply in-shader post processing (fog, alpha-premultiply, and also tonemapping, debanding if the camera is non-hdr)
    // note this does not include fullscreen postprocessing effects like bloom.
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
#endif

    if in2.uv.y > 0.9 {
        out.color = vec4f(0.0, 0.0, 0.0, 1.0);
    }
    else {
        out.color = vec4f(1.0, 1.0, 1.0, 1.0);
    }
    return out;
}
