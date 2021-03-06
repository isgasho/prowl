/// Renders Entities
use specs::{
    ReadStorage,
    Entities,
    System,
};

use crate::components::{
    CharRenderer,
    // Health,
    Money,
    // Named,
    Player,
    Position,
    TileMap,
};

#[allow(unused_imports)]
use crate::resources::{
    Window,
    game_data::{
        GameData,
        StateChangeRequest,
    },
};
use crate::console::resource::Console;

use crate::ui::{
    panel::Panel,
};

use crate::renderer::tcod_renderer;
use tcod_renderer as renderer;

use crate::shared::Vector2;

use tcod::{
    colors::Color,
    console::*
};

pub struct RenderingSystem;

impl<'a> System<'a> for RenderingSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, CharRenderer>,
        ReadStorage<'a, Money>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, TileMap>,
        ReadStorage<'a, Panel>,
        specs::Read<'a, Console>,
        specs::Write<'a, Window>,
        specs::Write<'a, GameData>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (positions,
             char_renderers,
             _moneys,
             players,
             tilemaps,
             panels,
             _console,
             mut window,
             mut game_data,
             entities,
             ) = data;

        // TODO store player position in game data or something?
        // Then we can render without reading entities.
        let mut offset = Vector2::new(-window.size.x/3, -window.size.y/2);
        for (_player, entity) in (&players, &entities).join() {
            match positions.get(entity) {
                Some(pos) => offset += pos.value,
                None => (),
            }
        }
        let offset = offset;

        if window.root.window_closed() {
            game_data.state_change_request = Some(StateChangeRequest::QuitGame);
        }

        let mut root = Offscreen::new(window.size.x, window.size.y);
        renderer::init(&mut root);
        // Render map.
        for tilemap in (&tilemaps).join() {
            renderer::put_map(
                &mut root,
                &tilemap,
                &Vector2::zero(),
                &offset,
                &window.size);
        }
        // Render characters, ignoring any that aren't in view.
        let lower_right_corner = window.size + offset;
        for (position, char_renderer) in (&positions, &char_renderers)
            .join()
            .into_iter()
            .filter(|(pos, _)| {
                (*pos).value.x >= offset.x &&
                (*pos).value.y >= offset.y &&
                (*pos).value.x <= lower_right_corner.x &&
                (*pos).value.y <= lower_right_corner.y
            }) {
                let (x, y) = (position.value - offset).to_tuple();
                let x = x as i32;
                let y = y as i32;
                renderer::put_char(
                    &mut root,
                    Vector2::new(x, y),
                    &char_renderer,
                );
            }
        // Render UI
        for panel in (&panels).join() {
            renderer::put_panel(&mut root, &panel)
        }
        // INSTRUCTIONS TODO replace with panel?
        renderer::put_text(
            &mut root,
            Vector2::new(2, (*window).size.y - 2),
            Vector2::new(200, 5),
            &Color::new(0x00, 0x50, 0x80),
            "[wasd] to move, [esc] to quit",
        );
        // Draw to the screen
        window.blit(&root);
    }
}
