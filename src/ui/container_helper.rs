use ratatui::Frame;
use ratatui::layout::Rect;
use std::cmp::{min, max};
use std::io::Result;

pub struct Margin {
    pub left: u16,
    pub right: u16,
    pub top: u16,
    pub bottom: u16
}

impl Margin {
    pub fn new(top: u16, right: u16, bottom:u16, left: u16) -> Margin {
        Margin {
            top,
            right,
            bottom,
            left
        }
    }
}

/// Gets the rect with applied margins
pub fn get_main_layout_rect(frame: &Frame) -> Rect {
    let frame_rect = frame.size();
    let margin_x_percent = 1;
    let margin_y_percent = 2;

    apply_margins(frame_rect, Margin::new(
        frame_rect.height * margin_y_percent / 100 / 2,
        frame_rect.width * margin_x_percent / 100 /2,
        frame_rect.height * margin_y_percent / 100/ 2,
        frame_rect.width * margin_x_percent / 100 /2
    ))
}
pub fn apply_margins(rect: Rect, margins: Margin) -> Rect {
    let width = rect.width;
    let height = rect.height;

    let offset_x = margins.left;
    let offset_y = margins.top;

    let actual_width = max((width as i64) - (offset_x + margins.right) as i64,  0) as u16;
    let actual_height = max((height as i64) - (offset_y + margins.bottom) as i64, 0) as u16;
    Rect::new(offset_x, offset_y, actual_width, actual_height)
}