use crate::component_group_store::ComponentGroupStore;

mod component_group;
mod component_group_store;

#[derive(Debug)]
struct Name(&'static str);

#[derive(Debug)]
struct Position(f32, f32);

#[derive(Debug)]
struct Player;

#[derive(Debug)]
struct Enemy;

#[derive(Debug)]
struct Obstacle;

fn main() {
    let mut component_group_store: ComponentGroupStore = ComponentGroupStore::default();
    component_group_store.push((Player, Name("Player"), Position(5.0, 15.0)));
    component_group_store.push((Obstacle, Name("Wall"), Position(3.0, 10.0)));
    component_group_store.push((Obstacle, Name("Wall"), Position(4.0, 10.0)));
    component_group_store.push((Obstacle, Name("Wall"), Position(5.0, 10.0)));
    component_group_store.push((Obstacle, Name("Wall"), Position(6.0, 10.0)));
    component_group_store.push((Obstacle, Name("Wall"), Position(7.0, 10.0)));
    component_group_store.push((Obstacle, Name("Wall")));
    component_group_store.push((Enemy, Name("Enemy"), Position(5.0, 5.0)));

    for position in component_group_store.get_1::<Position>() {
        println!("position: {:?}", position);
    }

    for (obstacle, name) in component_group_store.get_2::<Obstacle, Name>() {
        println!("obstacle: {:?}, name: {:?}", obstacle, name);
    }

    for (enemy, name, position) in component_group_store.get_3::<Enemy, Name, Position>() {
        println!(
            "enemy: {:?}, name: {:?}, position: {:?}",
            enemy, name, position
        );
    }
}