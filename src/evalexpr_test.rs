use evalexpr::*;

fn evaluate_expressions() {
    assert_eq!(eval("1 + 2 + 3"), Ok(Value::from(6)));
    // `eval` returns a variant of the `Value` enum,
    // while `eval_[type]` returns the respective type directly.
    // Both can be used interchangeably.
    assert_eq!(eval_int("1 + 2 + 3"), Ok(6));
    assert_eq!(eval("1 - 2 * 3"), Ok(Value::from(-5)));
    assert_eq!(eval("1.0 + 2 * 3"), Ok(Value::from(7.0)));
    assert_eq!(eval("true && 4 > 2"), Ok(Value::from(true)));
}

fn chain_expressions() {
    let mut context = HashMapContext::new();
    // Assign 5 to a like this
    assert_eq!(eval_empty_with_context_mut("a = 5", &mut context), Ok(EMPTY_VALUE));
    // The HashMapContext is type safe, so this will fail now
    assert_eq!(eval_empty_with_context_mut("a = 5.0", &mut context), Err(EvalexprError::expected_int(Value::from(5.0))));
    // We can check which value the context stores for a like this
    assert_eq!(context.get_value("a"), Some(&Value::from(5)));
    // And use the value in another expression like this
    assert_eq!(eval_int_with_context_mut("a = a + 2; a", &mut context), Ok(7));
    // It is also possible to safe a bit of typing by using an operator-assignment operator
    assert_eq!(eval_int_with_context_mut("a += 2; a", &mut context), Ok(9));
}

pub fn evalexpr_tests() {
    evaluate_expressions();
    chain_expressions();

}
