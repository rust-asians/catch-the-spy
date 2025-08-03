use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_magical_orb)
        .add_systems(Update, make_orbs_more_magical)
        .run();
}

#[derive(Component)]
struct MagicalOrb;

fn spawn_magical_orb(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);
    let pure_light = materials.add(Color::hsl(0.0, 0.0, 1.0));
    let magical_orb = meshes.add(Circle::new(50.0));
    commands.spawn((
        Mesh2d(magical_orb),
        MeshMaterial2d(pure_light),
        Transform::from_xyz(0.0, 0.0, 0.0),
        MagicalOrb,
    ));
}

fn make_orbs_more_magical(time: Res<Time>, mut query: Query<&mut Transform, With<MagicalOrb>>) {
    for mut transform in &mut query {
        let scale = ops::sin(time.elapsed_secs() / 4.0) + 2.0;
        transform.scale.x = scale;
        transform.scale.y = scale;
    }
}
