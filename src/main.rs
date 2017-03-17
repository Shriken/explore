#[macro_use] extern crate rusterize;
extern crate sdl2;

use rusterize::LoopState;
use rusterize::ScreenConfig;
use rusterize::object::Object;
use rusterize::renderer::Renderer;
use rusterize::screen::Screen;
use rusterize::types::*;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode as SdlKeycode;

use std::cmp::min;
use std::error;
use std::f64;
use std::process;


pub const SCREEN_WIDTH:  u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;
pub const TARGET_FPS:    u32 = 60;
pub const TIME_PER_TICK: f64 = 1. / (TARGET_FPS as f64);

pub const MOVE_SPEED: f64 = 0.2;
pub const TURN_SPEED: f64 = 2.;

struct WorldState {
    time: f64,
    objects: Vec<Object>,
    camera: Camera,

    input: Input,
}

struct Camera {
    heading: f64,
    pos: Point,
}

#[derive(Default)]
struct Input {
    move_left:    bool,
    move_right:   bool,
    move_forward: bool,
    move_back:    bool,

    turn_right: bool,
    turn_left:  bool,
}


fn main() {
    let result = rusterize::main_loop(
        ScreenConfig {
            title:      "explore",
            width:      SCREEN_WIDTH,
            height:     SCREEN_HEIGHT,
            target_fps: TARGET_FPS,
        },
        init,
        parse_event,
        update,
        render,
    );

    if let Err(e) = result {
        println!("error: {}", e);
        process::exit(-1);
    }
}

fn init<S: Screen>(renderer: &mut Renderer<S>)
    -> Result<WorldState, Box<error::Error>>
{
    // Set perspective transform.
    renderer.set_transform({
        let screen_scale = (min(SCREEN_WIDTH, SCREEN_HEIGHT) / 2) as f64;
        let screen_transform =
            Transform::translate(pt_2d![
                (SCREEN_WIDTH  / 2) as Coord,
                (SCREEN_HEIGHT / 2) as Coord
            ]) * Transform::scale(screen_scale, screen_scale, 1.);
        let perspective_transform = Transform::perspective();
        let camera_transform = Transform::identity();
        screen_transform * perspective_transform * camera_transform
    });
    renderer.set_light_pos(pt![0., -20., 0.]);

    // Load objects.
    let objects = {
        let mut objects = Vec::new();

        objects.push({
            Object::from_file("res/cube.obj")?
                .scaled(3., 3., 3.)
                .translated(pt![0., 0., -20.])
        });

        objects
    };

    Ok(WorldState {
        time: 0.,
        objects: objects,
        camera: Camera {
            heading: 0.,
            pos: pt![0., 0., 0.],
        },

        input: Default::default(),
    })
}

fn parse_event(
    loop_state: &mut LoopState,
    world_state: &mut WorldState,
    event: SdlEvent
) {
    let ref mut input = world_state.input;
    match event {
        SdlEvent::Quit { .. } |
        SdlEvent::KeyDown { keycode: Some(SdlKeycode::Escape), .. } => {
            loop_state.running = false;
        },

        SdlEvent::KeyDown { keycode: Some(keycode), .. } => {
            match keycode {
                SdlKeycode::P => loop_state.paused = !loop_state.paused,
                SdlKeycode::Space => loop_state.step = true,
                SdlKeycode::W => input.move_forward = true,
                SdlKeycode::A => input.move_left    = true,
                SdlKeycode::S => input.move_back    = true,
                SdlKeycode::D => input.move_right   = true,
                SdlKeycode::Q => input.turn_right   = true,
                SdlKeycode::E => input.turn_left    = true,
                _ => {}
            }
        },

        SdlEvent::KeyUp { keycode: Some(keycode), .. } => {
            match keycode {
                SdlKeycode::W => input.move_forward = false,
                SdlKeycode::A => input.move_left    = false,
                SdlKeycode::S => input.move_back    = false,
                SdlKeycode::D => input.move_right   = false,
                SdlKeycode::Q => input.turn_right   = false,
                SdlKeycode::E => input.turn_left    = false,
                _ => {}
            }
        },

        _ => {}
    }
}


fn update(world_state: &mut WorldState) -> bool {
    world_state.time += TIME_PER_TICK;

    let ref mut camera = world_state.camera;
    let th = camera.heading;
    if world_state.input.move_left {
        camera.pos = camera.pos + pt![-th.cos(), 0., -th.sin()] * MOVE_SPEED;
    }
    if world_state.input.move_right {
        camera.pos = camera.pos + pt![ th.cos(), 0.,  th.sin()] * MOVE_SPEED;
    }
    if world_state.input.move_forward {
        camera.pos = camera.pos + pt![ th.sin(), 0., -th.cos()] * MOVE_SPEED;
    }
    if world_state.input.move_back {
        camera.pos = camera.pos + pt![-th.sin(), 0.,  th.cos()] * MOVE_SPEED;
    }

    if world_state.input.turn_left {
        camera.heading += TIME_PER_TICK * TURN_SPEED
    }
    if world_state.input.turn_right {
        camera.heading -= TIME_PER_TICK * TURN_SPEED
    }

    true
}

fn render<S: Screen>(
    renderer: &mut Renderer<S>,
    world_state: &WorldState
)
    -> Result<(), Box<error::Error>>
{
    renderer.clear();
    let camera_transform = Transform::rotate_y(world_state.camera.heading)
        * Transform::translate(-world_state.camera.pos);
    for obj in &world_state.objects {
        obj.render_with_transform(renderer, camera_transform);
    }
    try!(renderer.display());
    Ok(())
}
