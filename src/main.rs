use bevy::{prelude::*, render::camera::ScalingMode};

pub const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
pub const RESOLUTION: f32 = 9.0 / 16.0;

fn main() {
    let width = 1280.0;
    App::new()
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            title: "game".to_string(),
            width,
            height: width * RESOLUTION,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_ascii)
        .add_plugins(DefaultPlugins)
        .run()
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.3, 0.3, 0.9);
    sprite.custom_size = Some(Vec2::splat(1.0));

    commands.spawn_bundle(SpriteSheetBundle {
        sprite,
        texture_atlas: ascii.0.clone(),
        transform: Transform { 
            translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default() 
        },
        ..Default::default()
    }).insert(Name::new("Player"));
}

fn spawn_camera(mut commands: Commands){
    let mut camera = OrthographicCameraBundle::new_2d();

    camera.orthographic_projection.top = 1.0;
    camera.orthographic_projection.bottom = -1.0;
    camera.orthographic_projection.right = 1.0 * RESOLUTION;
    camera.orthographic_projection.left = -1.0 * RESOLUTION;

    camera.orthographic_projection.scaling_mode = ScalingMode::None;

    commands.spawn_bundle(camera);
}

struct AsciiSheet(Handle<TextureAtlas>);

fn load_ascii(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atleses: ResMut<Assets<TextureAtlas>>) {
        let image = assets.load("tileset.png");
        let atlas = TextureAtlas::from_grid(
            image,
            Vec2::splat(8.0),
            17,
            17
        );
        let atlas_handle = texture_atleses.add(atlas);
        commands.insert_resource(AsciiSheet(atlas_handle));
    }
