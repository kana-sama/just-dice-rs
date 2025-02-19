use crate::prelude::*;

const DIE_SIZE: i32 = 55;

const SELECTION_DURATION: u32 = 300;
const SELECTION_OFFSET: f32 = 4.;

#[derive(Clone, Copy, PartialEq)]
struct DieBitmapsCacheKey {
    value: u8,
    angle: f32,
    draw_mode: BitmapDrawMode,
}

struct DieBitmaps {
    die: Bitmap,
    shadow: Bitmap,
}

pub struct Die {
    theme: ThemeRef,

    value: u8,
    angle: f32,

    selected: bool,

    bitmaps: Cached<DieBitmapsCacheKey, DieBitmaps>,

    die_sprite: Sprite,
    shadow_sprite: Sprite,

    animated_position: Animated<Vector2D>,
    animated_selected: Animated<f32>,
}

fn prerendered_die(value: u8, angle: f32, draw_mode: BitmapDrawMode) -> DieBitmaps {
    let table: &_ = unsafe {
        static mut DIE_BITMAP_TABLE: Option<BitmapTable> = None;
        DIE_BITMAP_TABLE.get_or_insert_with(|| BitmapTable::load("assets/images/die-55").unwrap())
    };

    let angle_index = (angle % 180. / 3.) as i32;


    let shadow = table.get(6 * 60 + angle_index).unwrap().clone().render(|shadow| {
        let _ = pd::graphics::set_draw_mode(draw_mode);
        shadow.draw(0, 0, BitmapFlip::Unflipped);
    });

    let die_fg: Bitmap = table.get((value as i32 - 1) * 60 + angle_index).unwrap();
    let die_bg: Bitmap = table.get(7 * 60 + angle_index).unwrap();
    let die = die_bg.clone().render(|die_bg| {
        let _ = pd::graphics::set_draw_mode(draw_mode);
        die_bg.draw(0, 0, BitmapFlip::Unflipped);
        die_fg.draw(0, 0, BitmapFlip::Unflipped);
    });

    return DieBitmaps { die, shadow };
}

impl Die {
    pub fn new(theme: ThemeRef) -> Die {
        let die_sprite = Sprite::new();
        die_sprite.set_z_index(ZIndex::Die as _);
        die_sprite.move_to(100., 100.);
        die_sprite.add();

        let shadow_sprite = Sprite::new();
        shadow_sprite.set_z_index(ZIndex::DieShadow as _);
        shadow_sprite.move_to(105., 105.);
        shadow_sprite.add();

        let animated_position = Animated::new(
            easing::OUT_CIRC,
            500,
            Vector2D::zero(),
            Vector2D::zero(),
        );

        let animated_selected = Animated::default();

        Die {
            theme,

            value: 1,
            angle: 0.,

            selected: false,

            bitmaps: Cached::new(),

            die_sprite,
            shadow_sprite,

            animated_position,
            animated_selected,
        }
    }

    pub fn update(&mut self) {
        let DieBitmaps { die, shadow } = self.bitmaps.get(
            DieBitmapsCacheKey { value: self.value, angle: self.angle, draw_mode: self.theme.borrow().image_draw_mode() },
            |key| prerendered_die(key.value, key.angle, key.draw_mode),
        );

        self.die_sprite.set_image(die, BitmapFlip::Unflipped);
        self.shadow_sprite.set_image(shadow, BitmapFlip::Unflipped);

        let position = self.animated_position.value();

        let animated_floating: &mut _ = unsafe {
            static mut ANIMATED_FLOATING: Option<Animated<f32>> = None;
            ANIMATED_FLOATING.get_or_insert_with(|| {
                let mut floating = Animated::new(easing::IN_OUT_QUAD, 700, -1.5, 1.5);
                floating.set_looping(Looping::LoopAndReverse);
                floating.start();
                floating
            })
        };

        let selected = self.animated_selected.value();
        let floating = if selected > 0. { animated_floating.value() } else { 0. };

        self.die_sprite.move_to(
            position.x.floor(),
            (position.y - selected + floating).floor(),
        );

        let shadow_is_visible = selected > 0.;
        if shadow_is_visible {
            self.shadow_sprite.move_to(
                (position.x + selected - floating).floor(),
                (position.y + selected - floating).floor(),
            );
            self.shadow_sprite.add();
        } else {
            self.shadow_sprite.remove();
        }
    }

    pub fn roll_to(&mut self, position: Vector2D, angle: f32) {
        self.angle = angle;

        self.animated_position.set_start_value(Vector2D::new(-DIE_SIZE as f32, -DIE_SIZE as f32));
        self.animated_position.set_end_value(position);
        self.animated_position.start();
    }

    pub fn move_to(&mut self, position: Vector2D, angle: f32) {
        self.angle = angle;

        self.animated_position.set_end_value(position);
        self.animated_position.finish();
    }

    pub fn start_removing(&mut self) {
        let current_position = self.animated_position.value();

        self.animated_position.set_start_value(current_position);
        self.animated_position.set_end_value(Vector2D::new(
            pd::display::width() as f32 + DIE_SIZE as f32,
            pd::display::height() as f32 + DIE_SIZE as f32,
        ));

        self.animated_position.start();
    }

    pub fn can_be_removed(&self) -> bool {
        self.animated_position.is_finished()
    }

    pub fn select(&mut self) {
        if self.selected {
            return;
        } else {
            self.selected = true;
        }

        self.animated_selected = Animated::new(
            easing::LINEAR,
            SELECTION_DURATION,
            0.,
            SELECTION_OFFSET,
        );

        self.animated_selected.start();
    }

    pub fn deselect(&mut self) {
        if self.selected {
            self.selected = false;
        } else {
            return;
        }

        self.animated_selected = Animated::new(
            easing::LINEAR,
            SELECTION_DURATION - self.animated_selected.remaining_duration(),
            self.animated_selected.value(),
            0.,
        );

        self.animated_selected.start();
    }

    pub fn bounds(&mut self) -> PDRect {
        let mut bounds = self.die_sprite.bounds();
        bounds.set_center(self.animated_position.value());
        bounds
    }

    pub fn position(&self) -> Vector2D {
        self.animated_position.get_end_value()
    }

    pub fn angle(&self) -> f32 {
        self.angle
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }
}
