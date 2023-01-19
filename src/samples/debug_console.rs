use cursive::{views::Dialog, Cursive, reexports::log};

use crate::traits::Sample;

pub struct DebugConsole;

impl Sample for DebugConsole {
  fn run(&self, siv: &mut Cursive) {
    cursive::logger::init();
    log::error!("Err");
    log::warn!("Warn");
    log::debug!("Debug");
    log::info!("Starting");
    
    siv.add_layer(Dialog::text("Press ~ to open the console.\nPress l to generate logs.\nPress q to quit."));
    siv.add_global_callback('q', Cursive::quit);
    siv.add_global_callback('~', Cursive::toggle_debug_console);
    siv.add_global_callback('l', |_| log::trace!("Trace"));
  }
}