use crate::components::*;
use specs::{Builder, World, WorldExt};

// Create a wall entity
pub fn create_wall(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_image("/images/wall.png".to_string()))
        .with(Wall {})
        .with(Immovable)
        .build();
}

pub fn create_floor(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 5, ..position })
        .with(Renderable::new_image("/images/floor.png".to_string()))
        .build();
}

pub fn create_box(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_sprite(vec![
            format!("/images/box_{}_1.png", colour.to_string()),
            format!("/images/box_{}_2.png", colour.to_string()),
        ]))
        .with(Box { colour })
        .with(Movable)
        .build();
}

pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) {
    world
        .create_entity()
        .with(Position { z: 9, ..position })
        .with(Renderable::new_image(format!(
            "/images/box_spot_{}.png",
            colour.to_string()
        )))
        .with(BoxSpot { colour })
        .build();
}

pub fn create_player(world: &mut World, position: Position) {
    world
        .create_entity()
        .with(Position { z: 10, ..position })
        .with(Renderable::new_sprite(vec![
            "/images/player_1.png".to_string(),
            "/images/player_2.png".to_string(),
            "/images/player_3.png".to_string(),
        ]))
        .with(Player {})
        .with(Movable)
        .build();
}
