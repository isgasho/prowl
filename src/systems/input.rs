use specs::{
    System,
    WriteStorage,
    ReadStorage,
    Read,
    Write,
    Join
};
use crate::components::{
    Position,
    pending_actions::{
        PendingActions,
        Action,
    },
    Player,
};
use crate::resources::{
    RendererResource,
    Quit,
    input::{
        InputCode,
        UserInput
    },
    console::{
        Console,
        LogLevel,
        Log
    }
};

#[derive(Default)]
pub struct UserInputSystem;

impl<'a> System<'a> for UserInputSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        WriteStorage<'a, Position>,
        // WriteStorage<'a, PendingActions>,
        Read<'a, UserInput>,
        Write<'a, Quit>,
        Write<'a, Console>,
        // TODO get this out of here...
        specs::Write<'a, RendererResource>,
        );

    fn run(&mut self, data: Self::SystemData) {
        let (
            players,
            mut positions,
            // mut pending_actionses,
            input,
            mut quit,
            mut console,
            mut renderer,
        ) = data;

        for (
                _,
                position,
                // pending_actions
            ) in (
                &players,
                &mut positions,
                // &mut pending_actionses,
            ).join() {
            // for some reason input has to be dereferenced.
            // presumably any resource with methods would have to
            // be dereferenced?
            // TODO: input resource may be possible as just Input, need to test
            // so like: `Input::get()`
            // input.get() is blocking, nothing can happen while waiting for
            // input. We will have to re-evaluate input if we want a
            // non-blocking method.

            // NOTE For some reason trying to read pending_actions mutably
            // makes this system non-blocking? WTF
            // let key: InputCode = (*input).get();
            // while let key = (*input).get());
            // let mut key = (*input).get();
            // while key == InputCode::None { key = (*input).get(); }
            let key = UserInput::get(&mut renderer.root);
            (*console).log(Log::new(LogLevel::Debug, "Input Registered"));
            use InputCode::*;
            // let mut delta = (0, 0);
            match key {
                Up => {
                    // delta.1 = -1;
                    position.y -= 1;
                    // pending_actions.actions.push(Action::Move { delta });
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Up"));
                },
                Down => {
                    position.y += 1;
                    // delta.1 = 1;
                    // pending_actions.actions.push(Action::Move { delta });
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Down"));
                },
                Left => {
                    position.x -= 1;
                    // delta.0 = -1;
                    // pending_actions.actions.push(Action::Move { delta });
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Left"));
                },
                Right => {
                    position.x += 1;
                    // delta.0 = 1;
                    // pending_actions.actions.push(Action::Move { delta });
                    (*console).log(Log::new(LogLevel::Debug, "Player Moved Right"));
                },
                Quit => quit.0 = true,
                _ => (),
            }
            // if delta != (0, 0) { pending_actions.actions.push(Action::Move { delta }); }
        }
    }
}
