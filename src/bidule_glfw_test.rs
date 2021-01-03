extern crate glfw;

use bidule::Stream;

use glfw::{Action, Context};


enum Button {
    Pressed,
    Released
}

fn unbuttonify(button: &Button, v: i32) -> Option<i32> {
    match *button {
        Button::Released => Some(v),
        _ => None
    }
}

fn window_event_to_type_string(event: &glfw::WindowEvent) -> String {
    match event {
        glfw::WindowEvent::Pos(_, _) => String::from("Pos"),
        glfw::WindowEvent::Size(_, _) => String::from("Size"),
        glfw::WindowEvent::Close => String::from("Close"),
        glfw::WindowEvent::Refresh => String::from("Refresh"),
        glfw::WindowEvent::Focus(_) => String::from("Focus"),
        glfw::WindowEvent::MouseButton(_, _, _) => String::from("MouseButton"),
        glfw::WindowEvent::CursorPos(_, _) => String::from("CursorPos"),
        glfw::WindowEvent::Key(_, _, _, _) => String::from("Key"),
        glfw::WindowEvent::Char(_) => String::from("Char"),
        _ => String::from("")
    }
}

pub fn bidule_glfw_tests () {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    let window_event_stream : Stream<glfw::WindowEvent> = Stream::new();
    // window_event_stream.observe(|event| {
    //     println!("Window Event: {}", window_event_to_type_string(event));
    // });
    let key_event_stream = window_event_stream.filter(|event|
        match event {
            glfw::WindowEvent::Key(_, _, _, _) => true,
            _ => false
        }
    );
    // key_event_stream.observe(|event| {
    //     println!("Key Event");
    // });
    let key_release_event_stream = key_event_stream.filter(|event|
        match event {
            glfw::WindowEvent::Key(_, _, Action::Release, _) => true,
            _ => false
        }
    );
    key_release_event_stream.observe(|event| {
        println!("Key Up");
    });

    let mouse_button_event_stream = window_event_stream.filter(|event|
        match event {
            glfw::WindowEvent::MouseButton(_, Action::Release, _) => true,
            _ => false
        }
    );
    let mouse_button_release_event_stream = mouse_button_event_stream.filter(|event|
        match event {
            glfw::WindowEvent::MouseButton(_, Action::Release, _) => true,
            _ => false
        }
    );
    mouse_button_release_event_stream.observe(|event| {
        println!("Mouse Button Release");
    });

    let (mut window, events) = glfw.create_window(
        1024,
        768,
        "BIDULE + GLFW",
        glfw::WindowMode::Windowed
    ).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_all_polling(true);

    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            window_event_stream.send(&event);
        }
    }
}
