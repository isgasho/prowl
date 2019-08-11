/// Builds other ships.
use crate::components::{
    ai::{Goal, AI},
    markers::Ship,
    pending_actions::PendingActions,
    CharRenderer, Health, Money, Named, Position, Weapon,
};
use crate::file_io::read_file;
use crate::shared::{random::random_range, Vector2};
use specs::prelude::*;
use tcod::colors::*;
// TODO maybe don't do this
use crate::MAP_SIZE;

// TODO what determines parameters ? ?
// What can ships do?
// This is all just placeholder!
pub fn make_ship(world: &mut World) {
    let names = read_file("names.txt");
    let name = names[random_range(0, names.len())].clone();
    let health = random_range(80, 200) as i64;
    let weapon = random_range(1, 10) as u64;
    let money = random_range(30, 300) as u64;
    let max = (MAP_SIZE - 1) as usize;
    let x = random_range(0, max) as i32;
    let y = random_range(0, max) as i32;
    let position = Vector2::new(x, y);
    let renderer = (
        &name.chars().next().unwrap().clone(),
        Color::new(
            random_range(0x88, 0xff) as u8,
            random_range(0x88, 0xff) as u8,
            random_range(0x88, 0xff) as u8,
        ),
    );
    world
        .create_entity()
        .with(Named::new(&name))
        .with(Health::new(health, health))
        // does nothing yet
        .with(Weapon::new(weapon))
        // does nothing yet
        .with(Money::new(money))
        .with(Position::new(position))
        .with(CharRenderer::new(*renderer.0, renderer.1))
        .with(PendingActions::default())
        .with(AI::with_goal(Goal::MoveRandom))
        .with(Ship::default())
        .build();
}

pub fn make_ship_new<'a>(
    entities: Entities<'a>,
    names: ReadStorage<'a, Named>,
    healths: ReadStorage<'a, Health>,
    weapons: ReadStorage<'a, Weapon>,
    renderers: ReadStorage<'a, CharRenderer>,
) {
    let possible_names = read_file("names.txt");
    let name = possible_names[random_range(0, possible_names.len())].clone();
}