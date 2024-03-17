//@fragment
//fn fs_main(@location(0) frag_color: vec4<f32>, @binding(0) color_to_replace: vec4<f32>, @binding(1) replacement_color: vec4<f32>) -> @location(0) vec4<f32> {
////    let color_to_replace = vec3(1.0, 1.0, 1.0); // RGB for white
////    let replacement_color = vec3(1.0, 0.0, 0.0); // RGB for red, as an example
//
//    if (distance(frag_color.rgb, color_to_replace) < 0.1) { // Adjust the threshold as needed
//        return vec4(replacement_color, 1.0);
//    } else {
//        return frag_color;
//    }
//}

@fragment
fn fs_main(@location(0) tex_coords: vec2<f32>, @location(1) frag_color: vec4<f32>, @binding(0) texture: texture_2d<f32>, @binding(1) color_to_replace: vec4<f32>, @binding(2) replacement_color: vec4<f32>) -> @location(0) vec4<f32> {
    let original_color = textureSample(texture, tex_coords);
    if (distance(original_color, color_to_replace) < 0.1) { // Adjust threshold as needed
        return replacement_color;
    } else {
        return original_color;
    }
}