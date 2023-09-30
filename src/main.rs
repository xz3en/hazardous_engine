//use std::sync::mpsc::Receiver;
use glium::{self, Surface};

struct Game {
    event_loop: glium::glutin::event_loop::EventLoop<()>,
    display: glium::Display,
}

impl Game {
    fn new() -> Game {
        let event_loop = glium::glutin::event_loop::EventLoop::new();

        let wb = glium::glutin::window::WindowBuilder::new()
            .with_inner_size(glium::glutin::dpi::LogicalSize::new(800.0, 600.0))
            .with_title("Game");

        let cb = glium::glutin::ContextBuilder::new();

        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        Game {
            event_loop,
            display,
        }
    }

    fn run(self) {
        self.event_loop
            .run(move |event, _, control_flow| match event {
                glium::glutin::event::Event::WindowEvent { event, .. } => {
                    // Events go here

                    if event == glium::glutin::event::WindowEvent::CloseRequested {
                        *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    }
                }
                glium::glutin::event::Event::MainEventsCleared => {
                    
                }
                glium::glutin::event::Event::RedrawRequested(_) => {
                    let mut frame = self.display.draw();

                    frame.clear_color(0.2, 0.3, 0.3, 1.0);

                    

                    let _ = frame.finish();
                }
                _ => (),
            });
    }
}

fn main() {
    let game = Game::new();
    game.run();
}
