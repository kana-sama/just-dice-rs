use crate::prelude::*;

use crate::services::Random;

use crate::utils::pool::{Pool, Poolable};
use crate::utils::vector_2d::Vector2D;
use crate::utils::cached::Cached;
use crate::utils::animated::{Animated, Looping};
use crate::utils::easing;

const DIE_SIZE: i32 = 55;

const SELECTION_DURATION: u32 = 300;
const SELECTION_OFFSET: f32 = 4.0;

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

    shake_effect: Pool<ShakeEffect, 3>,
    roll_effect: RollEffect,
}

impl Die {
    pub fn new(theme: ThemeRef) -> Die {
        let die_sprite = Sprite::new();
        die_sprite.set_z_index(ZIndex::Die as _);
        die_sprite.move_to(100.0, 100.0);
        die_sprite.add();

        let shadow_sprite = Sprite::new();
        shadow_sprite.set_z_index(ZIndex::DieShadow as _);
        shadow_sprite.move_to(105.0, 105.0);
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
            angle: 0.0,

            selected: false,

            bitmaps: Cached::new(),

            die_sprite,
            shadow_sprite,

            animated_position,
            animated_selected,

            shake_effect: Pool::new(),
            roll_effect: RollEffect::new(),
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
        let floating = if selected > 0.0 { animated_floating.value() } else { 0.0 };

        self.die_sprite.move_to(
            position.x.floor(),
            (position.y - selected + floating).floor(),
        );

        let shadow_is_visible = selected > 0.0;
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
            0.0,
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
            0.0,
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

    pub fn randomize(&mut self, random: &mut Random) {
        self.value = random.die_value();
    }

    pub fn play_shake_effect(&mut self, random: &mut Random) {
        if let Some(effect) = self.shake_effect.get() {
            effect.play(random);
        }
    }

    pub fn play_roll_effect(&mut self, random: &mut Random) {
        self.roll_effect.play(random);
    }
}



fn prerendered_die(value: u8, angle: f32, draw_mode: BitmapDrawMode) -> DieBitmaps {
    let table: &_ = unsafe {
        static mut DIE_BITMAP_TABLE: Option<BitmapTable> = None;
        DIE_BITMAP_TABLE.get_or_insert_with(|| BitmapTable::load("assets/images/die-55").unwrap())
    };

    let angle_index = (angle % 180.0 / 3.0) as i32;


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

struct ShakeEffect(SamplePlayer);

impl ShakeEffect {
    fn new() -> Self {
        let shake_effect: &_ = unsafe {
            static mut SHAKE_EFFECT: Option<Sample> = None;
            SHAKE_EFFECT.get_or_insert_with(|| pd::sound::sample::Sample::new_from_file("assets/sounds/shake").unwrap())
        };

        let player = pd::sound::player::SamplePlayer::new().unwrap();
        player.set_sample(&shake_effect);
        ShakeEffect(player)
    }

    fn play(&self, random: &mut Random) {
        self.0.set_offset(random.in_range(0.0 .. 0.1));
        self.0.set_volume(random.in_range(0.8 .. 1.0), random.in_range(0.8 .. 1.0));
        self.0.play(Repeat::Loops(1), random.in_range(0.5 .. 1.5));
    }
}

impl Poolable for ShakeEffect {
    fn new() -> Self {
        ShakeEffect::new()
    }

    fn is_free(&self) -> bool {
        !self.0.is_playing()
    }
}


struct RollEffect(SamplePlayer);

impl RollEffect {
    fn new() -> Self {
        let roll_effect: &_ = unsafe {
            static mut ROLL_EFFECT: Option<Sample> = None;
            ROLL_EFFECT.get_or_insert_with(|| pd::sound::sample::Sample::new_from_file("assets/sounds/roll-one").unwrap())
        };

        let player = pd::sound::player::SamplePlayer::new().unwrap();
        player.set_sample(&roll_effect);
        RollEffect(player)
    }

    fn play(&self, random: &mut Random) {
        self.0.stop();
        self.0.set_offset(random.in_range(0.0 .. 0.1));
        self.0.set_volume(random.in_range(0.8 .. 1.0), random.in_range(0.8 .. 1.0));
        self.0.play(Repeat::Loops(1), random.in_range(0.9 .. 1.5));
    }
}
