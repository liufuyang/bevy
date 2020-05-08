use crate::{BevyPathfinderDevice, BevyResourceLoader};
use bevy_asset::AssetStorage;
use bevy_render::{
    render_graph::{Node, ResourceSlots},
    renderer::RenderContext,
    shader::Shader,
};
use legion::prelude::{Resources, World};
use pathfinder_canvas::{vec2f, Canvas, CanvasFontContext, ColorF, Path2D, RectF, Vector2I};
use pathfinder_renderer::{
    concurrent::{rayon::RayonExecutor, scene_proxy::SceneProxy},
    gpu::{
        options::{DestFramebuffer, RendererOptions},
        renderer::Renderer,
    },
    options::BuildOptions,
};

#[derive(Default)]
pub struct PathfinderNode;

impl Node for PathfinderNode {
    fn update(
        &mut self,
        _world: &World,
        resources: &Resources,
        render_context: &mut dyn RenderContext,
        _input: &ResourceSlots,
        _output: &mut ResourceSlots,
    ) {
        println!("run");
        let mut shaders = resources.get_mut::<AssetStorage<Shader>>().unwrap();
        let device = BevyPathfinderDevice::new(render_context, &mut shaders);
        let window_size = Vector2I::new(640 as i32, 480 as i32);
        let mut renderer = Renderer::new(
            device,
            &BevyResourceLoader::new(),
            DestFramebuffer::full_window(window_size),
            RendererOptions {
                background_color: Some(ColorF::white()),
            },
        );

        // Make a canvas. We're going to draw a house.
        let mut canvas = Canvas::new(window_size.to_f32())
            .get_context_2d(CanvasFontContext::from_system_source());

        // Set line width.
        canvas.set_line_width(10.0);

        // Draw walls.
        canvas.stroke_rect(RectF::new(vec2f(75.0, 140.0), vec2f(150.0, 110.0)));

        // Draw door.
        canvas.fill_rect(RectF::new(vec2f(130.0, 190.0), vec2f(40.0, 60.0)));

        // Draw roof.
        let mut path = Path2D::new();
        path.move_to(vec2f(50.0, 140.0));
        path.line_to(vec2f(150.0, 60.0));
        path.line_to(vec2f(250.0, 140.0));
        path.close_path();
        canvas.stroke_path(path);

        // Render the canvas to screen.
        let scene = SceneProxy::from_scene(canvas.into_canvas().into_scene(), RayonExecutor);
        scene.build_and_render(&mut renderer, BuildOptions::default());
        // TODO: submit command buffers?
    }
}