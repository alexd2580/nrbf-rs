use std::collections::HashMap;
use std::fmt::Display;

#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Null,
    Byte(u8),
    Bool(bool),
    U8(u8),
    U32(u32),
    U64(u64),
    I8(i8),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(String),
    Array(Vec<usize>, Vec<usize>, Vec<Value>),
    Object(String, HashMap<String, Value>),
    Reference(i32),
    Bottom,
}

fn fmt_indent(v: &Value, f: &mut std::fmt::Formatter<'_>, indent: usize) -> std::fmt::Result {
    match v {
        Value::Null => write!(f, "Null"),
        Value::Byte(v) => write!(f, "{v}u8"),
        Value::Bool(v) => write!(f, "{v}"),
        Value::U8(v) => write!(f, "{v}u8"),
        Value::U32(v) => write!(f, "{v}u32"),
        Value::U64(v) => write!(f, "{v}u64"),
        Value::I8(v) => write!(f, "{v}i8"),
        Value::I32(v) => write!(f, "{v}i32"),
        Value::I64(v) => write!(f, "{v}i64"),
        Value::F32(v) => write!(f, "{v}uf32"),
        Value::F64(v) => write!(f, "{v}f64"),
        Value::String(v) => write!(f, "{v}"),
        Value::Array(_, _, vs) => {
            writeln!(f, "[").unwrap();
            for v in vs {
                write!(f, "{:>1$}", "", indent + 2).unwrap();
                fmt_indent(v, f, indent + 2).unwrap();
                writeln!(f, ",").unwrap();
            }
            write!(f, "{:>1$}]", "", indent)
        }
        Value::Object(class_name, members) => {
            writeln!(f, "{class_name} {{").unwrap();
            for (member, v) in members {
                write!(f, "{:>1$}{member}: ", "", indent + 2).unwrap();
                fmt_indent(v, f, indent + 2).unwrap();
                writeln!(f, ",").unwrap();
            }
            write!(f, "{:>1$}}}", "", indent)
        }
        Value::Reference(v) => write!(f, "#{v}"),
        Value::Bottom => write!(f, "ERROR"),
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt_indent(self, f, 0)
    }
}

fn expected_got<T>(expected: &str, got: &str) -> Result<T, String> {
    Err(format!(
        "Expected {expected}; Got {}",
        &got[..100.min(got.len())]
    ))
}

impl TryFrom<&Value> for bool {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::Bool(v) => Ok(*v),
            _ => expected_got("Bool", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for u8 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::U8(v) => Ok(*v),
            _ => expected_got("U8", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for u32 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::U32(v) => Ok(*v),
            _ => expected_got("U32", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for u64 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::U64(v) => Ok(*v),
            _ => expected_got("U64", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for i8 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::I8(v) => Ok(*v),
            _ => expected_got("I8", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for i32 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::I32(v) => Ok(*v),
            _ => expected_got("I32", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for i64 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::I64(v) => Ok(*v),
            _ => expected_got("I64", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for f32 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::F32(v) => Ok(*v),
            _ => expected_got("F32", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for f64 {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::F64(v) => Ok(*v),
            _ => expected_got("F64", &value.to_string()),
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = String;

    fn try_from(value: &Value) -> Result<Self, String> {
        match value {
            Value::String(v) => Ok(v.clone()),
            _ => expected_got("String", &value.to_string()),
        }
    }
}

impl<'a, T: TryFrom<&'a Value, Error = String>> TryFrom<&'a Value> for Vec<T> {
    type Error = String;

    fn try_from(value: &'a Value) -> Result<Self, String> {
        match value {
            Value::Array(_, _, v) => v.iter().map(T::try_from).collect::<Result<_, _>>(),
            _ => expected_got("Array", &value.to_string()),
        }
    }
}

// Null,
// Byte(u8),
// Object(String, HashMap<String, Value>),
