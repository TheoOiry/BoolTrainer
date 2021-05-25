use crate::app::action::bool_expression::expression::{BoolOption, Variable, OrdOption, Value};
use serde::de::Unexpected::Bool;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn basic_expressions() {
    assert_eq!(false, BoolOption::And(vec![Box::from(BoolOption::Const(true)), Box::from(BoolOption::Const(false))]).get_result());

    assert_eq!(true, BoolOption::Or(vec![Box::from(BoolOption::Const(true)), Box::from(BoolOption::Const(false))]).get_result());
}

#[test]
fn recursive_expressions() {
    let expression = BoolOption::And(vec![Box::from(BoolOption::Const(true)), Box::from(BoolOption::Const(true))]);
    let expression = BoolOption::And(vec![Box::from(expression), Box::from(BoolOption::Const(true))]);

    assert_eq!(true, expression.get_result());
}

#[test]
fn ord_expression() {
    let expression = BoolOption::OrdExpr(Box::from(OrdOption::Greater(Value::Const(5), Value::Const(2))));
    assert_eq!(true, expression.get_result())
}

#[test]
fn basic_expression_to_string() {
    let and_expr = BoolOption::And(vec![Box::from(BoolOption::Const(true)), Box::from(BoolOption::Const(false))]);
    let or_expr = BoolOption::Or(vec![Box::from(BoolOption::Const(true)), Box::from(BoolOption::Const(false))]);
    let not_expr = BoolOption::Not(Box::from(BoolOption::Const(false)));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(Value::Const(5), Value::Const(2))));

    assert_eq!("true && false", and_expr.to_string());
    assert_eq!("true || false", or_expr.to_string());
    assert_eq!("!false", not_expr.to_string());
    assert_eq!("5 > 2", ord_expr.to_string())
}

#[test]
fn recursive_expression() {
    let five_greater_two = BoolOption::OrdExpr(Box::from(OrdOption::GreaterEqual(Value::Const(5), Value::Const(2))));
    let for_equals_two = BoolOption::OrdExpr(Box::from(OrdOption::Equal(Value::Const(4), Value::Const(2))));
    let true_or_for_equals_two = BoolOption::Or(vec![Box::from(BoolOption::Const(true)), Box::from(for_equals_two)]);
    let expression = BoolOption::Not(Box::from(BoolOption::And(vec![Box::from(five_greater_two), Box::from(true_or_for_equals_two), Box::from(BoolOption::Const(true))])));

    assert_eq!("!(5 >= 2 && (true || 4 == 2) && true)", expression.to_string())
}

#[test]
fn variable_print() {
    let variable = Rc::new(RefCell::new(Variable { name: String::from("a"), data: 2 }));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(Value::Const(5), Value::Compute(variable))));

    assert_eq!("5 > a", ord_expr.to_string());
}

#[test]
fn variable_expression() {
    let variable = Rc::new(RefCell::new(Variable { name: String::from("a"), data: 2 }));
    let ord_expr = BoolOption::OrdExpr(Box::from(OrdOption::Greater(Value::Const(5), Value::Compute(Rc::clone(&variable)))));

    assert_eq!(true, ord_expr.get_result());
    variable.borrow_mut().data = 10;
    assert_eq!(false, ord_expr.get_result());
}