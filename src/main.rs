use bevy::{
    app::App,
    asset::{AddAsset, AssetServer, Assets},
    ecs::system::{Commands, IntoSystem, Res, ResMut},
    input::system::exit_on_esc_system,
    math::Vec3,
    pbr::{render_graph::LightsNode, AmbientLight, LightBundle},
    reflect::TypeUuid,
    render::{
        color::Color,
        entity::{MeshBundle, PerspectiveCameraBundle},
        pass::ClearColor,
        pipeline::{PipelineDescriptor, RenderPipeline, RenderPipelines},
        render_graph::{base, AssetRenderResourcesNode, RenderGraph},
        renderer::RenderResources,
        shader::{Shader, ShaderStages},
    },
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

const MAX_LIGHTS: usize = 10;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "1e08866c-0b8a-437e-8bce-37733b25127e"]
struct CelMaterial {
    pub albedo_color: Color,
}

fn main() -> anyhow::Result<()> {
    configure_logging()?;

    App::build()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(WindowDescriptor {
            title: "Cel Shading".to_string(),
            width: 1280.,
            height: 800.,
            resizable: false,
            ..WindowDescriptor::default()
        })
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 20.0f32,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(ObjPlugin)
        .add_startup_system(setup.system())
        .add_system(exit_on_esc_system.system())
        .add_asset::<CelMaterial>()
        .run();

    Ok(())
}

#[allow(clippy::needless_pass_by_value)]
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<CelMaterial>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    asset_server.watch_for_changes().unwrap();

    // Setup shaders
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: asset_server.load::<Shader, _>("shaders/cel.vert"),
        fragment: Some(asset_server.load::<Shader, _>("shaders/cel.frag")),
    }));

    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<CelMaterial>::new(true),
    );
    render_graph.add_system_node("lights", LightsNode::new(MAX_LIGHTS));

    render_graph
        .add_node_edge("my_material", base::node::MAIN_PASS)
        .unwrap();
    render_graph
        .add_node_edge("lights", base::node::MAIN_PASS)
        .unwrap();

    let material = materials.add(CelMaterial {
        albedo_color: Color::rgb(0.0, 1.0, 0.0),
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 4.0, 0.0),
        ..LightBundle::default()
    });
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(5., 10., -20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..PerspectiveCameraBundle::default()
    });

    commands
        .spawn_bundle(MeshBundle {
            mesh: asset_server.load("meshes/teapot.obj"),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..Transform::default()
            },
            ..MeshBundle::default()
        })
        .insert(material);
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
        // naga::front::spv spews ocasional logs about unexpected annotations. These are caused by
        // bevy, so I will ignore
        .logger(
            Logger::builder()
                .appender("console")
                .build("naga::front::spv", LevelFilter::Error),
        )
        .build(Root::builder().appender("console").build(LevelFilter::Info))?;
    log4rs::init_config(config)?;
    Ok(())
}
