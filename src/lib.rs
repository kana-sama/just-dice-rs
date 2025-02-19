#![no_std]

#![allow(static_mut_refs)]
#![allow(internal_features)]

#![feature(core_intrinsics)]
#![feature(trait_alias)]
#![feature(let_chains)]
#![feature(generic_arg_infer)]
#![feature(slice_as_chunks)]
#![feature(const_trait_impl)]
#![feature(maybe_uninit_array_assume_init)]
#![feature(maybe_uninit_uninit_array)]

#[allow(unused_imports)]
#[macro_use]
extern crate alloc;

#[macro_use]
extern crate playdate as pd;

mod components;
mod game;
mod prelude;
mod services;
mod utils;

use core::ptr::NonNull;

use pd::sys::ffi::PlaydateAPI;
use pd::sys::EventLoopCtrl;
use pd::system::prelude::*;
use pd::system::update::UpdateCtrl;

use crate::game::Game;

impl Update for game::Game {
    fn update(&mut self) -> UpdateCtrl {
        self.update();
        UpdateCtrl::Continue
    }
}

#[allow(static_mut_refs)]
#[no_mangle]
pub fn event_handler(_api: NonNull<PlaydateAPI>, event: SystemEvent, _sim_key_code: u32) -> EventLoopCtrl {
    pub static mut GAME: Option<Game> = None;

    match event {
        SystemEvent::Init => {
            let game = game::Game::new();
            let game = unsafe { GAME.insert(game) };
            Update::set_update_handler(game);
        }

        SystemEvent::Pause => {
            let game = unsafe { GAME.as_mut() }.unwrap();
            game.on_pause();
        }

        _ => {}
    };
    EventLoopCtrl::Continue
}

ll_symbols!();


// TODO:
// - ui sounds
// - shake and roll sounds
