#![allow(warnings)]
mod camera_controller;
use std::f32::consts::PI;
mod next_peice;
use bevy::{
    app::AppExit,
    core::FixedTimestep,
    math::{vec2, vec3},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;
use next_peice::*;
mod kind;
use camera_controller::*;
use kind::*;

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const SIZE: usize = WIDTH * HEIGHT;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct KindInit;

#[derive(Component)]
struct PeiceQueue;

struct DropNextDespawn;
struct DropNextSpawn;

fn main() {
    App::new()
        .add_event::<DropNextDespawn>()
        .add_event::<DropNextSpawn>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(CameraControllerPlugin)
        .insert_resource(NextPeieces::new())
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_system(despawn_queue)
        .add_system(spawn_queue.after(despawn_queue))
        .add_system(spawn_peice)
        .add_system(move_peice)
        .add_system(exit)
        .add_system(drop_next)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(move_down),
        )
        .run();
}

fn setup(mut commands: Commands, mut drop_next: EventWriter<DropNextSpawn>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5.0, 10.0, 30.0),
        ..Default::default()
    });
    //.insert(CameraController::default());

    drop_next.send(DropNextSpawn);
}

fn spawn_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut assets: ResMut<AssetServer>,
) {
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(WIDTH as f32 / 2.0, -0.25, 0.0),

            mesh: meshes.add(Mesh::from(shape::Quad {
                size: vec2(WIDTH as f32, 0.5),
                flip: false,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..Default::default()
            }),
            ..default()
        })
        .insert(Name::new("Board Bottom"));

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(-0.25, HEIGHT as f32 / 2.0, 0.0),

            mesh: meshes.add(Mesh::from(shape::Quad {
                size: vec2(0.5, HEIGHT as f32),
                flip: false,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..Default::default()
            }),
            ..default()
        })
        .insert(Name::new("Board Left"));

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(WIDTH as f32 + 0.25, HEIGHT as f32 / 2.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Quad {
                size: vec2(0.5, HEIGHT as f32),
                flip: false,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::BLACK,
                ..Default::default()
            }),
            ..default()
        })
        .insert(Name::new("Board Right"));
}

fn spawn_peice(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &Kind), With<KindInit>>,
) {
    for (e, k) in query.iter() {
        commands
            .entity(e)
            .insert(Name::new(format!("{:?}", k)))
            .with_children(|parent| {
                for pos in k.locations() {
                    parent.spawn_bundle(PbrBundle {
                        transform: Transform {
                            translation: pos,
                            rotation: Quat::from_axis_angle(Vec3::X, PI / 2.0),
                            ..default()
                        },
                        mesh: meshes.add(Mesh::from(shape::Plane { size: 0.9 })),
                        material: materials.add(StandardMaterial {
                            base_color: k.color(),
                            ..Default::default()
                        }),
                        ..default()
                    });
                }
            });
    }
}

fn move_peice(keyboard: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Player>>) {
    if keyboard.just_pressed(KeyCode::A) {
        for mut t in query.iter_mut() {
            t.translation -= vec3(1.0, 0.0, 0.0);
        }
    }

    if keyboard.just_pressed(KeyCode::D) {
        for mut t in query.iter_mut() {
            t.translation -= vec3(-1.0, 0.0, 0.0);
        }
    }
}

fn move_down(mut query: Query<&mut Transform, With<Player>>) {
    for mut t in query.iter_mut() {
        t.translation += vec3(0.0, -1.0, 0.0);
    }
}

fn despawn_queue(
    mut commands: Commands,
    mut drop_next_despawn: EventReader<DropNextDespawn>,
    mut drop_next_spawn: EventWriter<DropNextSpawn>,
    mut query: Query<Entity, With<PeiceQueue>>,
) {
    for _ in drop_next_despawn.iter() {
        for e in query.iter() {
            commands.entity(e).despawn_recursive();
        }
        drop_next_spawn.send(DropNextSpawn);
    }
}

fn spawn_queue(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_peices: ResMut<NextPeieces>,
    mut drop_next: EventReader<DropNextSpawn>,
) {
    for _ in drop_next.iter() {

        commands
            .spawn_bundle(TransformBundle {
                local: Transform::from_xyz(WIDTH as f32 + 3.0, HEIGHT as f32 - 3.0, 0.0),
                ..default()
            })
            .insert(PeiceQueue)
            //.insert(Name::new("Peice Queue"))
            .with_children(|parent| {
                let mut start = vec3(0.0, 0.0, 0.0);
                for i in (0..next_peices.peices.len()) {
                    
                    
                    parent
                        .spawn_bundle(TransformBundle {
                            local: Transform::from_translation(start),
                            ..default()
                        })
                        .insert(next_peices.peices[i])
                        .insert(KindInit);

                    start += vec3(0.0, -4.0, 0.0)
                }
            });
    }
}

fn drop_next(
    mut commands: Commands,
    mut next_peices: ResMut<NextPeieces>,
    keyboard: Res<Input<KeyCode>>,
    mut drop_next: EventWriter<DropNextDespawn>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        let p = next_peices.pop();

        drop_next.send(DropNextDespawn);

        commands
            .spawn()
            .insert(Transform::from_xyz(
                WIDTH as f32 * 0.5 + 0.5,
                HEIGHT as f32,
                0.0,
            ))
            .insert(GlobalTransform::default())
            .insert(KindInit)
            .insert(Player)
            .insert(p);
    }
}

fn exit(input: Res<Input<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if input.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit);
    }
}
