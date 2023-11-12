#import bevy_core_pipeline::tonemapping
#import bevy_sprite::mesh2d
#import bevy_sprite::mesh2d_functions as mesh_functions
// #import bevy_sprite::mesh2d_vertex_output::VertexOutput
// #import bevy_render::globals::Globals
// #import bevy_render::view::View

struct ColorGrading {
    exposure: f32,
    gamma: f32,
    pre_saturation: f32,
    post_saturation: f32,
}
struct View {
    view_proj: mat4x4<f32>,
    unjittered_view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    // viewport(x_origin, y_origin, width, height)
    viewport: vec4<f32>,
    frustum: array<vec4<f32>, 6>,
    color_grading: ColorGrading,
    mip_bias: f32,
};

struct Globals {
    // The time since startup in seconds
    // Wraps to 0 after 1 hour.
    time: f32,
    // The delta time since the previous frame in seconds
    delta_time: f32,
    // Frame count since the start of the app.
    // It wraps to zero when it reaches the maximum value of a u32.
    frame_count: u32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: f32
#endif
};


@group(0) @binding(0) var<uniform> view: View;
@group(0) @binding(1) var<uniform> globals: Globals;

struct VertexOutput {
    // this is `clip position` when the struct is used as a vertex stage output 
    // and `frag coord` when used as a fragment stage input
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    #ifdef VERTEX_TANGENTS
    @location(3) world_tangent: vec4<f32>,
    #endif
    #ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
    #endif
}
struct Vertex {
    @builtin(instance_index) instance_index: u32,
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
};


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_POSITIONS
    var model = mesh_functions::get_model_matrix(vertex.instance_index);
    out.world_position = mesh_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.position = mesh_functions::mesh2d_position_world_to_clip(out.world_position);
#endif

#ifdef VERTEX_NORMALS
    out.world_normal = mesh_functions::mesh2d_normal_local_to_world(vertex.normal, vertex.instance_index);
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_functions::mesh2d_tangent_local_to_world(
        model,
        vertex.tangent
    );
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif
    return out;
}

fn palette(t: f32, a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>) -> vec3<f32> {
    return a + b * (cos(6.28318 * (c * t + d)));
}

fn palette1(t: f32) -> vec3<f32> {
    var a = vec3(0.5, 0.5, 0.5);
    var b = vec3(0.5, 0.5, 0.5);
    var c = vec3(1.0, 1.0, 1.0);
    var d = vec3(0.263, 0.416, 0.557);
    return palette(t, a, b, c, d);
}

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    var fragCoord = in.position.xy;
    var iResolution = view.viewport.zw;
    var iTime = globals.time;
    var finalColor = vec3(0.0);

    var uv = (fragCoord * 2.0 - iResolution) / iResolution.y;
    var uv0 = uv;

    for (var i: i32 = 0; i < 4; i++) {
        // uv = fract(uv * 2.0) - 0.5;
        uv = fract(uv * 1.5) - 0.5;
        // var d = length(uv);
        var d = length(uv) * exp(-length(uv0));

        var color = palette1(length(uv0) + f32(i) * 0.4 + iTime * 0.4);

        d = sin(d * 8. + iTime) / 8.;
        d = pow(0.01 / abs(d), 1.8);


        finalColor += color * d;
    }
    return vec4<f32>(finalColor, 1.0);
    // return vec4<f32>(uv.xy, 0.0, 1.0);
}



