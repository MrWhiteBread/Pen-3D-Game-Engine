fn slerp(q1: vec4<f32>, q2: vec4<f32>, t: f32) -> vec4<f32> {
    var cos_theta = dot(q1, q2);

    var q2p = q2;
    if (cos_theta < 0.0) {
        q2p = -q2;
        cos_theta = -cos_theta;
    }

    if (cos_theta > 0.9995) {
        return normalize(q1 * (1.0 - t) + q2p * t);
    }

    let theta = acos(cos_theta);
    let sin_theta = sin(theta);

    let w1 = sin((1.0 - t) * theta) / sin_theta;
    let w2 = sin(t * theta) / sin_theta;

    return q1 * w1 + q2p * w2;
}