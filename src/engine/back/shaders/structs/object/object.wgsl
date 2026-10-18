struct Object {
    rot_mat1: vec4<f32>, // color
    rot_mat2: vec4<f32>, // old color
    rot_mat3: vec4<f32>, // id

    center: vec4<f32>, // material[0]
    size: vec4<f32>, // material[1]
}