pub fn interpolate<T>(time: f32, items: &Vec<T>) -> T::Value where T: Interpolatable, T: core::fmt::Debug//, R: std::ops::Mul<f32>
{
    let translation_far_index = items.iter().filter(|v| v.time() <= time).count();
    let translation_far_index = if translation_far_index == items.len() { translation_far_index - 1 } else { translation_far_index };
    let translation_near_index = if translation_far_index == 0 { 0 } else { translation_far_index - 1 };
    let ref near_translation = items[translation_near_index];
    let ref far_translation = items[translation_far_index];
    let near_position = near_translation.get_value();
    let far_position = far_translation.get_value();

    let translation = if translation_far_index == translation_near_index
    {
        near_position
    }
    else
    {
        let interval_length = far_translation.time() - near_translation.time();
        let normalised_time = time - near_translation.time();
        let far_translation_weight = normalised_time / interval_length;
        let near_translation_weight = 1.0 - far_translation_weight;
        let t = near_position * near_translation_weight + far_position * far_translation_weight;
        t
    };

    translation
}
