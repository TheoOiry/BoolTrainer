use dotenv::var;
use std::fmt::{Display, Formatter};
use std::cell::RefCell;
use std::rc::Rc;

pub trait OrdValue: Display + Clone + PartialOrd { }
impl<T: Display + Clone + PartialOrd> OrdValue for T { }

pub type VariableCell<T> = Rc<RefCell<Variable<T>>>;

pub enum BoolOption {
    Const(bool),
    OrdExpr(Box<OrdOption<u32>>),
    Not(Box<BoolOption>),
    And(Vec<Box<BoolOption>>),
    Or(Vec<Box<BoolOption>>),
}

pub enum OrdOption<T: OrdValue> {
    Equal(Value<T>, Value<T>),
    NotEqual(Value<T>, Value<T>),
    Greater(Value<T>, Value<T>),
    GreaterEqual(Value<T>, Value<T>),
    Less(Value<T>, Value<T>),
    LessEqual(Value<T>, Value<T>),
}

pub enum Value<T: OrdValue> {
    Const(T),
    Compute(VariableCell<T>)
}

pub struct Variable<T: OrdValue> {
    name: String,
    data: T,
}

impl BoolOption {
    pub fn get_result(&self) -> bool {
        match self {
            BoolOption::Const(bool) => *bool,
            BoolOption::OrdExpr(ord_expr) => ord_expr.get_result(),
            BoolOption::Not(expr) => !expr.get_result(),
            BoolOption::And(col_expr) => col_expr.iter()
                .fold(true, |acc, expr| acc && expr.get_result()),
            BoolOption::Or(col_expr) => col_expr.iter()
                .fold(false, |acc, expr| acc || expr.get_result())
        }
    }

    fn build_expr_col(col: &[Box<BoolOption>], join_token: &str) -> String {
        col.iter()
            .fold(String::new(), |acc, expr| {
                match **expr {
                    BoolOption::Or(_) | BoolOption::And(_) => format!("{} {} ({})", acc, join_token, expr.to_string()),
                    _ => format!("{} {} {}", acc, join_token, expr.to_string())
                }
            }).trim_start_matches(&format!(" {} ", join_token)).to_owned()
    }
}

impl<T: OrdValue> OrdOption<T> {
    pub fn get_result(&self) -> bool {
        match self {
            OrdOption::Equal(v1, v2) => v1.get_value() == v2.get_value(),
            OrdOption::NotEqual(v1, v2) => v1.get_value() != v2.get_value(),
            OrdOption::Greater(v1, v2) => v1.get_value() > v2.get_value(),
            OrdOption::GreaterEqual(v1, v2) => v1.get_value() >= v2.get_value(),
            OrdOption::Less(v1, v2) => v1.get_value() < v2.get_value(),
            OrdOption::LessEqual(v1, v2) => v1.get_value() <= v2.get_value(),
        }
    }
}

impl<T: OrdValue> Value<T> {
    fn get_value(&self) -> T {
        match self {
            Value::Const(v) => v.clone(),
            Value::Compute(variable) => variable.borrow().get_value().clone()
        }
    }
}

impl<T: OrdValue> Variable<T> {
    pub fn new(name: String, data: T) -> Self<T> {
        Variable { name, data }
    }

    fn get_value(&self) -> &T {
        &self.data
    }
}

impl ToString for BoolOption {
    fn to_string(&self) -> String {
        match self {
            BoolOption::Const(cst) => cst.to_string(),
            BoolOption::OrdExpr(ord_expr) => ord_expr.to_string(),
            BoolOption::Not(expr) => {
                match **expr {
                    BoolOption::OrdExpr(_) | BoolOption::Or(_) | BoolOption::And(_) => format!("!({})", expr.to_string()),
                    _ => format!("!{}", expr.to_string())
                }
            }
            BoolOption::And(col_expr) => BoolOption::build_expr_vec(col_expr, "&&"),
            BoolOption::Or(col_expr) => BoolOption::build_expr_vec(col_expr, "||"),
        }
    }
}

impl<T: OrdValue> ToString for OrdOption<T> {
    fn to_string(&self) -> String {
        match self {
            OrdOption::Equal(v1, v2) => format!("{} == {}", v1.to_string(), v2.to_string()),
            OrdOption::NotEqual(v1, v2) => format!("{} != {}", v1.to_string(), v2.to_string()),
            OrdOption::Greater(v1, v2) => format!("{} > {}", v1.to_string(), v2.to_string()),
            OrdOption::GreaterEqual(v1, v2) => format!("{} >= {}", v1.to_string(), v2.to_string()),
            OrdOption::Less(v1, v2) => format!("{} < {}", v1.to_string(), v2.to_string()),
            OrdOption::LessEqual(v1, v2) => format!("{} <= {}", v1.to_string(), v2.to_string())
        }
    }
}

impl<T: OrdValue> ToString for Value<T> {
    fn to_string(&self) -> String {
        match self {
            Value::Const(value) => value.to_string(),
            Value::Compute(variable) => variable.borrow().name.to_owned()
        }
    }
}
