use std::time::Duration;

// TODO: Also have a minimum height
pub(super) fn scale_card(frame_width: f32, frame_height: f32, rows: usize) -> (f32, f32) {
    let scaling_with_width = {
        let new_width = frame_width / 4.0;
        let new_height = (cardgen::CARDHEIGHT as f32) * (new_width / (cardgen::CARDWIDTH as f32));
        (new_width, new_height)
    };

    let scaling_with_height = {
        let divide = (1 + std::cmp::max(4, rows)) as f32;
        let new_height = frame_height / divide;
        let new_width = (cardgen::CARDWIDTH as f32) * (new_height / (cardgen::CARDHEIGHT as f32));
        (new_width, new_height)
    };

    if scaling_with_height.0 < scaling_with_width.0 {
        scaling_with_height
    } else {
        scaling_with_width
    }
}

pub(super) fn standard_format(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;

    format!("{:02}:{:02}", minutes, seconds)
}
