use crate::prelude::*;

use crate::components;
use crate::services;

const INITIAL_DICE: usize = 3;
const MAX_DICE: usize = 9;
const UNINITIALIZED_CURSOR_INDEX: usize = usize::MAX;

static BOARDS: [Board; 100] = BytesDecoder::decode(include_bytes!("../assets/positions.bin"));

pub struct Game {
    random: services::Random,
    input: services::Input,
    shaking: services::Shaking,
    settings: services::Settings,
    theme_manager: services::ThemeManager,

    fade: components::Fade,

    dice: Vec<components::Die>,
    removed_dice: Vec<components::Die>,

    board: Board,

    cursor: components::Cursor,
    cursor_index: usize,
    next_die_table: [usize; MAX_DICE],
    prev_die_table: [usize; MAX_DICE],

    in_selection_mode: bool,
    selected_dice: [bool; MAX_DICE],

    is_locked: bool,
    lock: components::Lock,
}

impl Game {
    pub fn new() -> Self {
        pd::display::set_refresh_rate(50.0);

        let settings = services::Settings::load();
        let theme_manager = services::ThemeManager::new(settings.dark_theme());
        let mut random = services::Random::new();

        pd::graphics::set_background_color(theme_manager.theme().borrow().background);
        pd::graphics::clear(Color::Solid(theme_manager.theme().borrow().background));

        let dice = (0..INITIAL_DICE).map(|_| {
            let mut die = components::Die::new(theme_manager.theme());
            die.set_value(random.die_value());
            return die;
        }).collect();

        let is_locked = !pd::controls::crank::docked();

        let mut game = Self {
            input: services::Input::new(),
            shaking: services::Shaking::new(),
            fade: components::Fade::new(theme_manager.theme()),
            settings,

            dice,
            removed_dice: vec![],
            board: random.element(&BOARDS).clone(),

            in_selection_mode: false,

            cursor: components::Cursor::new(theme_manager.theme()),
            cursor_index: UNINITIALIZED_CURSOR_INDEX,
            selected_dice: [false; MAX_DICE],
            next_die_table: [0; MAX_DICE],
            prev_die_table: [0; MAX_DICE],

            is_locked,
            lock: components::Lock::new(is_locked, theme_manager.theme()),

            theme_manager,
            random,
        };

        game.apply_positions();

        return game;
    }

    pub fn on_pause(&self) {
        pd::menu::set_menu_image(components::Stat::render(self.theme_manager.theme(), self.random.die_value_stat()), 100);
    }

    pub fn logic(&mut self) {
        self.is_locked = !pd::controls::crank::docked();

        if self.is_locked {
            return;
        }

        if self.shaking.just_started() {
            self.fade.enter();
        }

        if self.shaking.just_stopped() {
            self.fade.leave();
            self.shuffle_dice();
        }

        if self.shaking.right_now() && self.shaking.in_extremum() {
            println!("extremum");
        }

        if self.in_selection_mode {
            if self.input.pressed.left() {
                self.move_cursor_left();
            }

            if self.input.pressed.right() {
                self.move_cursor_right();
            }

            if self.input.pressed.a() {
                self.toggle_die_under_cursor();
            }

            if self.input.pressed.b() {
                self.cancel_selection();
            }
        } else {
            if self.input.pressed.left() {
                self.remove_die();
            }

            if self.input.pressed.right() {
                self.add_new_die();
            }

            if self.input.pressed.a() {
                self.start_selection();
            }
        }
    }

    pub fn update(&mut self) {
        self.input.update();
        self.shaking.update();
        self.settings.update();
        self.theme_manager.set_dark_theme(self.settings.dark_theme());

        self.logic();

        if self.in_selection_mode {
            self.cursor.move_to(self.dice[self.cursor_index].bounds());
        }

        self.lock.set(self.is_locked);
        self.lock.update();

        self.cursor.update();
        self.fade.update();

        self.removed_dice.retain(|die| !die.can_be_removed());

        for die in self.dice.iter_mut().chain(self.removed_dice.iter_mut()) {
            die.update();
        }

        pd::sprite::update_and_draw_sprites();

        if self.settings.draw_fps() {
            pd::system::draw_fps(0, 0);
        }
    }


    // Dice management

    pub fn add_new_die(&mut self) {
        if self.dice.len() < MAX_DICE {
            self.randomize_next_free_position();

            let mut die = components::Die::new(self.theme_manager.theme());
            die.set_value(self.random.die_value());
            self.dice.push(die);

            self.apply_positions();
        }
    }

    pub fn remove_die(&mut self) {
        if self.dice.len() > 1 {
            let mut die = self.dice.remove(self.dice.len() - 1);
            die.start_removing();
            self.removed_dice.push(die);
        }
    }

    pub fn shuffle_dice(&mut self) {
        self.board = self.random.element(&BOARDS).clone();

        for (i, ((die, position), selected)) in self.dice.iter_mut().zip(self.board.positions).zip(self.selected_dice).enumerate() {
            let DiePosition{position, angle} = position;

            if selected {
                die.move_to(position, angle);

                if self.in_selection_mode && i == self.cursor_index {
                    self.cursor.immediately_move_to(die.bounds());
                }
            } else {
                die.set_value(self.random.die_value());
                die.roll_to(position, angle);
            }
        }

        if self.in_selection_mode {
            self.build_cursor_tables();
        }
    }

    pub fn randomize_next_free_position(&mut self) {
        let i = self.dice.len();
        let j = self.random.in_range(self.dice.len()..MAX_DICE);
        (self.board.positions[i], self.board.positions[j]) = (self.board.positions[j], self.board.positions[i]);
    }

    pub fn apply_positions(&mut self) {
        for (i, die) in self.dice.iter_mut().enumerate() {
            let DiePosition{position, angle} = self.board.positions[i];

            if (die.position(), die.angle()) != (position, angle) {
                die.roll_to(position, angle);
            }
        }
    }


    // Selection

    fn start_selection(&mut self) {
        self.in_selection_mode = true;
        self.selected_dice.fill(false);
        self.build_cursor_tables();
        self.cursor.show(self.dice[self.cursor_index].bounds());
    }

    fn build_cursor_tables(&mut self) {
        let mut dice = self.dice.iter().enumerate().collect::<Vec<_>>();
        dice.sort_by_key(|(_, die)| die.position().x as i32);

        for (index_sorted, (index_in_dice, _)) in dice.iter().enumerate() {
            self.next_die_table[*index_in_dice] = dice[(index_sorted + 1) % dice.len()].0;
            self.prev_die_table[*index_in_dice] = dice[(index_sorted + dice.len() - 1) % dice.len()].0;
        }

        if self.cursor_index == usize::MAX {
            self.cursor_index = dice.first().unwrap().0;
        }

        if self.cursor_index >= self.dice.len() {
            self.cursor_index = dice.last().unwrap().0;
        }
    }

    fn cancel_selection(&mut self) {
        self.in_selection_mode = false;
        self.cursor.hide();
        self.selected_dice.fill(false);

        for die in self.dice.iter_mut() {
            die.deselect();
        }
    }

    fn move_cursor_left(&mut self) {
        self.cursor_index = self.prev_die_table[self.cursor_index];
    }

    fn move_cursor_right(&mut self) {
        self.cursor_index = self.next_die_table[self.cursor_index];
    }

    fn toggle_die_under_cursor(&mut self) {
        self.selected_dice[self.cursor_index] = !self.selected_dice[self.cursor_index];

        if self.selected_dice[self.cursor_index] {
            self.dice[self.cursor_index].select();
        } else {
            self.dice[self.cursor_index].deselect();
        }
    }
}


#[derive(Clone, Copy, Debug)]
struct DiePosition {
    position: Vector2D,
    angle: f32,
}

#[derive(Clone, Copy, Debug)]
struct Board {
    positions: [DiePosition; MAX_DICE],
}

impl const BytesDecode for DiePosition {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        DiePosition {
            position: decoder.take(),
            angle: decoder.take(),
        }
    }
}

impl const BytesDecode for Board {
    fn decode(decoder: &mut BytesDecoder) -> Self {
        Board { positions: decoder.take() }
    }
}
