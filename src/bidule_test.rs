use bidule::Stream;
use serde_json::{Value,json};
use strum::EnumString;

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

fn send_to_other_stream() {

    let property_receiver :Stream<Value> = Stream::new();
    property_receiver.observe(|v| println!("{}", v));

    {
        let property_sender :Stream<Value> = Stream::new();


        let value = Value::from(String::from("Should not be printed"));
        property_sender.send(&value);

        // let receiver = property_receiver.recv();
        property_sender.observe(move |sig| {
            property_receiver.send(sig);
        });

        let value = Value::from(String::from("Should be printed"));
        property_sender.send(&value);

        let s: String = "Should be printed 2".to_string();
        let value: Value = s.into();
        property_sender.send(&value);
    }

    // property_sender no more alive

}

#[derive(Debug, PartialEq, EnumString)]
enum OperatorPosition {
    LHS,
    RHS
}

#[derive(Copy, Clone)]
struct Expression<LHS, RHS> {
    lhs: LHS,
    rhs: RHS
}

struct ExpressionResult<T, LHS, RHS> {
    symbol: String,
    expression: Expression<LHS, RHS>,
    result: T,
}

fn logical_gate_test() {
    let mut output_1:Stream<Value> = Stream::new();
    output_1.observe(|v| println!("{}", v));

    let lhs :Stream<Value> = Stream::new();
    let rhs :Stream<Value> = Stream::new();
    lhs.observe(|v| println!("{} => lhs",v));
    rhs.observe(|v| println!("{} => rhs",v));

    let lhs_b = lhs.map(|v| match v.as_bool() {
        Some(b) => (OperatorPosition::LHS, b),
        None => (OperatorPosition::LHS, false)
    });
    let rhs_b = rhs.map(|v| match v.as_bool() {
        Some(b) => (OperatorPosition::RHS, b),
        None => (OperatorPosition::RHS, false)
    });
    // lhs_b.observe(| (o, v) | println!("[{:?} {}] => lhs_b", o, v));
    // rhs_b.observe(| (o, v) | println!("[{:?} {}] => rhs_b", o, v));

    let expression_b = lhs_b.merge(&rhs_b);
    expression_b.observe(| (o, v) | println!("[{:?} {}] => expression", o, v));

    let expression = lhs_b
        .merge(&rhs_b)
        .fold(
            Expression { lhs: false, rhs: false },
            | old_state, (o, value) | {
                // println!("expression [{:?} {}]", o, value);
                match *o {
                    OperatorPosition::LHS => {
                        // println!("LHS: {:?} {}", o, value);
                        Expression { lhs: *value, rhs: old_state.rhs }
                    },
                    OperatorPosition::RHS => {
                        // println!("RHS: {:?} {}", o, value);
                        Expression { lhs: old_state.lhs, rhs: *value }
                    }
                }
            }
        );
    expression.observe(| e | println!("({:?},{:?}) => expression", e.lhs, e.rhs));

    // let and_operator = | e: Expression<bool, bool> | -> bool { e.lhs && e.rhs };
    let and_operator_2 = | expression: Expression<bool, bool> | -> ExpressionResult<bool, bool, bool> {
        ExpressionResult {
            symbol: String::from("&&"),
            expression,
            result: expression.lhs && expression.rhs
        }
    };

    // let expression: Expression<bool, bool> = Expression { lhs: true, rhs: true };
    // let result: ExpressionResult<bool, bool, bool> = ExpressionResult {
    //     expression,
    //     result: e.lhs && e.rhs
    // };

    // let result_1 = expression.map(| e | e.lhs && e.rhs);
    // let result_2 = expression.map(and_operator_2);
    let result_2 = expression.map(move | e |
        ExpressionResult {
            symbol: String::from("&&"),
            expression: e.clone(),
            result: e.lhs && e.rhs
        }
    );
    // let output_1_b = expression.map(| e | e.lhs && e.rhs);
    // result_1.observe(| result | println!("Result: {:?}", result));
    result_2.observe(| result | println!("{} {} {} => {}", result.expression.lhs, result.symbol, result.expression.rhs, result.result));

    lhs.send(&json!(true));
    println!("---");
    rhs.send(&json!(false));
    println!("---");
    lhs.send(&json!(false));
    println!("---");
    rhs.send(&json!(true));
    println!("---");
    lhs.send(&json!(true));
    println!("---");

}

pub fn bidule_tests () {
    simple_observer();
    even_number_observer();
    even_odd_number_observers();
    stream_map_observer();
    button_example();
    send_to_other_stream();
    logical_gate_test();
}
