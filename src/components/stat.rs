use crate::prelude::*;

pub struct Stat {}

impl Stat {
    pub fn render(theme: ThemeRef, stat: [usize; 6]) -> Bitmap {
        let die_table: &_ = unsafe {
            static mut DIE_BITMAP_TABLE: Option<BitmapTable> = None;
            DIE_BITMAP_TABLE.get_or_insert_with(|| BitmapTable::load("assets/images/die-21").unwrap())
        };

        let font: &_ = unsafe {
            static mut FONT: Option<Font> = None;
            FONT.get_or_insert_with(|| pd::graphics::text::load_font("assets/fonts/Roobert-11-Mono-Condensed").unwrap())
        };

        let bg: &_ = unsafe {
            static mut BG: Option<Bitmap> = None;
            BG.get_or_insert_with(|| Bitmap::load("assets/images/stat").unwrap())
        };

        Bitmap::new(400, 240, Color::CLEAR).unwrap().render(|_| {
            let _ = pd::graphics::set_draw_mode(theme.borrow().image_draw_mode());

            bg.draw_tiled(0, 0, 400, 240, BitmapFlip::Unflipped);

            pd::graphics::text::set_font(&font);

            const LINE_HEIGHT: i32 = 35;
            const LINE_X: i32 = 20 + 100;
            const LINE_W: i32 = 160;

            const START_Y: i32 = (240 - LINE_HEIGHT * 6) / 2;
            const DIE_SIZE: i32 = 21;

            const DIE_Y_OFFSET: i32 = (LINE_HEIGHT - DIE_SIZE) / 2;

            const FONT_HEIGHT: i32 = 16;
            const TEXT_Y_OFFSET: i32 = (LINE_HEIGHT - FONT_HEIGHT) / 2 + 2;

            const COUNT_X_OFFSET: i32 = LINE_X + 55;
            const COUNT_WIDTH: i32 = 50;

            const PERCENTAGE_X_OFFSET: i32 = LINE_X + 120;
            const PERCENTAGE_WIDTH: i32 = 30;

            let total = stat.iter().copied().sum::<usize>();

            for (i, count) in stat.iter().copied().enumerate() {
                let percentage = (count as f32 / total as f32 * 100.0) as i32;
                let percentage = format!("{percentage}%");

                let die: Bitmap = die_table.get(i as i32).unwrap();

                let line_y = START_Y + LINE_HEIGHT * i as i32;

                die.draw(LINE_X, line_y + DIE_Y_OFFSET, BitmapFlip::Unflipped);

                let prev_draw_mode = pd::graphics::set_draw_mode(theme.borrow().text_draw_mode());

                pd::graphics::text::draw_text_in_rect(
                    count.to_string(),
                    COUNT_X_OFFSET, line_y + TEXT_Y_OFFSET,
                    COUNT_WIDTH, FONT_HEIGHT,
                    TextWrappingMode::Clip, TextAlignment::Center,
                ).unwrap();

                pd::graphics::text::draw_text_in_rect(
                    percentage,
                    PERCENTAGE_X_OFFSET, line_y + TEXT_Y_OFFSET,
                    PERCENTAGE_WIDTH, FONT_HEIGHT,
                    TextWrappingMode::Clip, TextAlignment::Right,
                ).unwrap();

                let _ = pd::graphics::set_draw_mode(prev_draw_mode);

                if i != stat.len() - 1 {
                    pd::graphics::fill_rect(LINE_X, line_y + LINE_HEIGHT -1, LINE_W, 1, theme.borrow().foreground as _);
                }
            }
        })
    }
}
