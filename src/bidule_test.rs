use bidule::Stream;

fn simple_observer () {
    let my_stream: Stream<i32> = Stream::new();
    my_stream.observe(|sig| {
        // print the signal on stdout each time it’s flowing in
        println!("signal: {:?}", sig);
    });
    my_stream.send(&1);
    my_stream.send(&2);
    my_stream.send(&3);
}

fn even_number_observer () {
    let int_stream: Stream<i32> = Stream::new();
    let even_stream = int_stream.filter(|x| x % 2 == 0);
    even_stream.observe(|sig| {
        // print the signal on stdout each time it’s flowing in
        println!("even number: {:?}", sig);
    });
    for i in 0..10 {
        int_stream.send(&i);
    }
}

fn even_odd_number_observers () {
    let int_stream: Stream<i32> = Stream::new();
    let even_stream = int_stream.filter(|x| x % 2 == 0);
    even_stream.observe(|sig| {
        // print the signal on stdout each time it’s flowing in
        println!("even number: {:?}", sig);
    });
    let odd_stream = int_stream.filter(|x| x % 2 != 0);
    odd_stream.observe(|sig| {
        // print the signal on stdout each time it’s flowing in
        println!("odd number: {:?}", sig);
    });
    for i in 0..10 {
        int_stream.send(&i);
    }
}

fn stream_map_observer () {
    let int_stream: Stream<i32> = Stream::new();
    let even_stream = int_stream.filter(|x| x % 2 == 0);
    let str_stream = even_stream.map(|x| if *x <= 10 { "Hello, world!" } else { "See you!" });
    str_stream.observe(|sig| {
        // print the signal on stdout each time it’s flowing in
        println!("{:?}", sig);
    });
    for i in 0..20 {
        int_stream.send(&i);
    }
}


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

fn btn_to_string(button: &Button) -> String {
    match *button {
        Button::Released => String::from("released"),
        _ => String::from("pressed")
    }
}

fn button_example () {
    let minus = Stream::new();
    let plus = Stream::new();
    let counter =
        minus.filter_map(|b| unbuttonify(b, -1))
            .merge(&plus.filter_map(|b| unbuttonify(b, 1)))
            .fold(0, |a, x| a + x);
    minus.observe(|sig| {
        println!("- {}", btn_to_string(sig));
    });
    plus.observe(|sig| {
        println!("+ {}", btn_to_string(sig));
    });
    counter.observe(|sig| {
        println!("Counter {:?}", sig);
    });
    plus.send(&Button::Pressed);
    plus.send(&Button::Released);
    minus.send(&Button::Pressed);
    minus.send(&Button::Released);
    plus.send(&Button::Pressed);
    plus.send(&Button::Released);
    plus.send(&Button::Pressed);
    plus.send(&Button::Released);
    plus.send(&Button::Pressed);
    minus.send(&Button::Pressed);
    plus.send(&Button::Released);
    minus.send(&Button::Released);
}

pub fn bidule_tests () {
    simple_observer();
    even_number_observer();
    even_odd_number_observers();
    stream_map_observer();
    button_example();
}
