use crate::app::core::bool_expression::expression_option::{
    BoolOption, OrdOption, Value, Variable,
};
use crate::app::core::bool_expression::generator::ExpressionGenerator;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn basic_expressions() {
    assert_eq!(
        false,
        BoolOption::And(vec![
            BoolOption::Const(true),
            BoolOption::Const(false)
        ])
        .get_result()
    );

    assert_eq!(
        true,
        BoolOption::Or(vec![
            BoolOption::Const(true),
            BoolOption::Const(false)
        ])
        .get_result()
    );
}

#[test]
fn recursive_expressions() {
    let expression = BoolOption::And(vec![
        BoolOption::Const(true),
        BoolOption::Const(true),
    ]);
    let expression = BoolOption::And(vec![
        expression,
        BoolOption::Const(true),
    ]);

    assert_eq!(true, expression.get_result());
}

#[test]
fn ord_expression() {
    let expression = BoolOption::OrdExpr(Box::from(OrdOption::Greater(
        Value::Const(5),
        Value::Const(2),
    )));
    assert_eq!(true, expression.get_result())
}

#[test]
fn basic_expression_to_string() {
    let and_expr = BoolOption::And(vec![
        BoolOption::Const(true),
        BoolOption::Const(false),
    ]);
    let or_expr = BoolOption::Or(vec![
        BoolOption::Const(true),
        BoolOption::Const(false),
    ]);
    let not_expr = BoolOption::Not(Box::from(BoolOption::Const(false)));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(
        Value::Const(5),
        Value::Const(2),
    )));

    assert_eq!("true && false", and_expr.to_string());
    assert_eq!("true || false", or_expr.to_string());
    assert_eq!("!false", not_expr.to_string());
    assert_eq!("5 > 2", ord_expr.to_string())
}

#[test]
fn recursive_expression() {
    let five_greater_two = BoolOption::OrdExpr(Box::from(OrdOption::GreaterEqual(
        Value::Const(5),
        Value::Const(2),
    )));
    let for_equals_two = BoolOption::OrdExpr(Box::from(OrdOption::Equal(
        Value::Const(4),
        Value::Const(2),
    )));
    let true_or_for_equals_two = BoolOption::Or(vec![
        BoolOption::Const(true),
        for_equals_two,
    ]);
    let expression = BoolOption::Not(Box::from(BoolOption::And(vec![
        five_greater_two,
        true_or_for_equals_two,
        BoolOption::Const(true),
    ])));

    assert_eq!(
        "!(5 >= 2 && (true || 4 == 2) && true)",
        expression.to_string()
    )
}

#[test]
fn variable_print() {
    let variable = Rc::new(RefCell::new(Variable::new(String::from("a"), 2)));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(
        Value::Const(5),
        Value::Compute(variable),
    )));

    assert_eq!("5 > a", ord_expr.to_string());
}

#[test]
fn variable_expression() {
    let variable = Rc::new(RefCell::new(Variable::new(String::from("a"), 2)));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(
        Value::Const(5),
        Value::Compute(Rc::clone(&variable)),
    )));

    assert_eq!(true, ord_expr.get_result());
    variable.borrow_mut().set_value(10);
    assert_eq!(false, ord_expr.get_result());
}

#[test]
fn generate_expression() {
    let expression = ExpressionGenerator::build();
    println!("{}", expression.to_string());
}

#[test]
fn generate_ten_expressions() {
    for i in 0..10 {
        let expression = ExpressionGenerator::build();
        println!("{}: {}", i, expression.to_string());
    }
}

#[test]
fn generate_thousand_expressions() {
    for i in 0..1000 {
        let expression = ExpressionGenerator::build();
        println!("{}: {}", i, expression.to_string());
    }
}

#[test]
#[ignore]
fn generate_max_expressions() {
    for _ in 0..99999 {
        ExpressionGenerator::build();
    }
}
