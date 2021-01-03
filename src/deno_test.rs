use anyhow::anyhow;
use deno_core::json_op_sync;
use deno_core::JsRuntime;
use deno_core::Op;
use deno_core::serde_json::Value;
use std::io::Write;

pub fn deno_tests() {
    // Initialize a runtime instance
    let mut runtime = JsRuntime::new(Default::default());


    // The first thing we do is define two ops.  They will be used to show how to
    // pass data to Rust and back to JavaScript.
    //
    // The first one is used to print data to stdout, because by default the
    // JavaScript console functions are just stubs (they don't do anything).
    //
    // The second one just transforms some input and returns it to JavaScript.

    // Register the op for outputting bytes to stdout.
    // It can be invoked with Deno.core.dispatch and the id this method returns
    // or Deno.core.dispatchByName and the name provided.
    runtime.register_op(
        "op_print",
        // The op_fn callback takes a state object OpState
        // and a vector of ZeroCopyBuf's, which are mutable references
        // to ArrayBuffer's in JavaScript.
        |_state, zero_copy| {
            let mut out = std::io::stdout();

            // Write the contents of every buffer to stdout
            for buf in zero_copy {
                out.write_all(&buf).unwrap();
            }

            Op::Sync(Box::new([])) // No meaningful result
        },
    );

    // Register the JSON op for summing a number array.
    // A JSON op is just an op where the first ZeroCopyBuf is a serialized JSON
    // value, the return value is also a serialized JSON value.  It can be invoked
    // with Deno.core.jsonOpSync and the name.
    runtime.register_op(
        "op_sum",
        // The json_op_sync function automatically deserializes
        // the first ZeroCopyBuf and serializes the return value
        // to reduce boilerplate
        json_op_sync(|_state, json, zero_copy| {
            // We check that we only got the JSON value,
            // and that it's of the right type.
            if !zero_copy.is_empty() {
                Err(anyhow!("Expected exactly one argument"))
            } else if !json.is_array() {
                Err(anyhow!("Argument is not of type array"))
            } else if !json
                .as_array()
                .unwrap()
                .iter()
                .all(|value| value.is_number())
            {
                Err(anyhow!("Argument is not array of numbers"))
            } else {
                // And if everything checks out we do our actual task
                let sum = json
                    .as_array()
                    .unwrap()
                    .iter()
                    .fold(0.0, |a, v| a + v.as_f64().unwrap());

                // Finally we return a JSON value
                Ok(Value::from(sum))
            }
        }),
    );


    runtime.execute(
        "<init>",
        r#"
Deno.core.ops();

const _newline = new Uint8Array([10]);

function print(value) {
  Deno.core.dispatchByName('op_print', Deno.core.encode(value.toString()), _newline);
}

Deno.core.registerErrorClass('Error', Error);

        "#).unwrap();


    // Now we can finally use this in an example.
    runtime
        .execute(
            "<usage>",
            r#"
const arr = [1, 2, 3];
print("The sum of");
print(arr);
print("is");
print(Deno.core.jsonOpSync('op_sum', arr));
// And incorrect usage
try {
  print(Deno.core.jsonOpSync('op_sum', 0));
} catch(e) {
  print('Exception:');
  print(e);
}
"#,
        )
        .unwrap();

}
