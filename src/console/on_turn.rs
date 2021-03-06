use specs::{
    Write,
    System,
};
pub struct OnTurnSystem;
#[allow(unused_imports)]
use crate::console::resource::{Console, Log, LogLevel};

impl<'a> System<'a> for OnTurnSystem {
    type SystemData = (Write<'a, Console>);

    fn run(&mut self, data: Self::SystemData) {
        let mut console = data;
        /*
        console.log(Log::new(
                LogLevel::Game,
                "."
                ));
                */
        // puts most recent console log at bottom of window.
        console.scroll_to_bottom();
    }
}
