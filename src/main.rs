#[macro_use] extern crate rusterize;
extern crate sdl2;

use rusterize::LoopState;
use rusterize::ScreenConfig;
use rusterize::object::Object;
use rusterize::renderer::Renderer;
use rusterize::screen::Screen;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode as SdlKeycode;

use std::error;
use std::process;


pub const TARGET_FPS:    u32 = 60;
pub const TIME_PER_TICK: f64 = 1. / (TARGET_FPS as f64);


struct WorldState {
    time: f64,
    objects: Vec<Object>
}


fn main() {
    let result = rusterize::main_loop(
        ScreenConfig {
            title:      "explore",
            width:      800,
            height:     600,
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
    let objects = {
        let mut objects = Vec::new();
        objects
    };

    Ok(WorldState {
        time: 0.,
        objects: objects,
    })
}

fn parse_event(
    loop_state: &mut LoopState,
    world_state: &mut WorldState,
    event: SdlEvent
) {
    match event {
        SdlEvent::Quit { .. } |
        SdlEvent::KeyDown { keycode: Some(SdlKeycode::Escape), .. } => {
            loop_state.running = false;
        },

        _ => {}
    }
}


fn update(world_state: &mut WorldState) -> bool {
    world_state.time += TIME_PER_TICK;

    true
}

fn render<S: Screen>(
    renderer: &mut Renderer<S>,
    world_state: &WorldState
)
    -> Result<(), Box<error::Error>>
{
    renderer.clear();
    for obj in &world_state.objects {
        obj.render(renderer);
    }
    try!(renderer.display());
    Ok(())
}
