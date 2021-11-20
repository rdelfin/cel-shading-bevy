use bevy::{
    app::App,
    asset::{AssetServer, Assets},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    input::system::exit_on_esc_system,
    math::Vec3,
    pbr::{prelude::StandardMaterial, LightBundle, PbrBundle},
    render::{color::Color, draw::Visible, entity::PerspectiveCameraBundle, pass::ClearColor},
    transform::components::Transform,
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_egui::EguiPlugin;
use bevy_obj::ObjPlugin;
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder,
};

fn main() -> anyhow::Result<()> {
    configure_logging()?;

    App::build()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            title: "Cel Shading".to_string(),
            width: 1280.,
            height: 800.,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ObjPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .run();

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // this material renders the texture normally
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 1.0, 0.0),
        ..StandardMaterial::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(2.0, 4.0, -5.0),
        ..LightBundle::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5., 10., -20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load("meshes/teapot.obj"),
        material: material_handle,
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            ..Transform::default()
        },
        visible: Visible {
            // is_transparent: true,
            ..Visible::default()
        },
        ..PbrBundle::default()
    });
}

fn configure_logging() -> anyhow::Result<()> {
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h([{l}] {d(%Y-%m-%d %H:%M:%S %Z)(utc)})} [{M}]: {m}{n}",
        )))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        // gfx_backend_vulkan spews a lot of logs on `Info` during startup, so it gets to die
        .logger(
            Logger::builder()
                .appender("console")
                .build("gfx_backend_vulkan", LevelFilter::Warn),
        )
        .build(Root::builder().appender("console").build(LevelFilter::Info))?;
    log4rs::init_config(config)?;
    Ok(())
}
