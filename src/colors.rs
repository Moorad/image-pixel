fn color_distance(color_1: [u8; 3], color_2: [u8; 3]) -> f32 {
    let r = (color_1[0] as f32 - color_2[0] as f32).powf(2.0);
    let g = (color_1[1] as f32 - color_2[1] as f32).powf(2.0);
    let b = (color_1[2] as f32 - color_2[2] as f32).powf(2.0);

    (r + g + b).sqrt()
}
