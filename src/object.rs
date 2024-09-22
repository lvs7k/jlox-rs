use crate::lox_callable::{CallableKind, LoxInstance};

#[derive(Debug, Clone)]
pub enum Object {
    Bool(bool),
    Num(f64),
    Str(String),
    Null,
    Callable(CallableKind),
    Instance(LoxInstance),
}

impl Object {
    pub fn is_bool(&self) -> bool {
        matches!(self, Object::Bool(_))
    }

    pub fn is_num(&self) -> bool {
        matches!(self, Object::Num(_))
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Object::Str(_))
    }

    pub fn is_null(&self) -> bool {
        matches!(self, Object::Null)
    }

    pub fn is_callable(&self) -> bool {
        matches!(self, Self::Callable(_))
    }

    pub fn is_truthy(&self) -> bool {
        if self.is_null() {
            return false;
        }
        if let Object::Bool(bool) = self {
            return *bool;
        }
        true
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bool(v) => write!(f, "{}", v),
            Self::Num(v) => write!(f, "{}", v),
            Self::Str(v) => write!(f, "{}", v),
            Self::Null => write!(f, "nil"),
            Self::Callable(v) => write!(f, "{}", v),
            Self::Instance(v) => write!(f, "{}", v),
        }
    }
}

impl std::ops::Neg for Object {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Object::Num(num) = self {
            return Object::Num(-num);
        }
        panic!("Failed to negate Object {}.", self);
    }
}

impl std::ops::Not for Object {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self.is_truthy() {
            Self::Bool(false)
        } else {
            Self::Bool(true)
        }
    }
}

impl std::ops::Add for Object {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(a), Object::Num(b)) => Object::Num(a + b),
            (Object::Str(a), Object::Str(b)) => {
                let mut str = a.clone();
                str.push_str(b);
                Object::Str(str)
            }
            _ => panic!("Failed to add Objects {} and {}.", &self, &rhs),
        }
    }
}

impl std::ops::Sub for Object {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(a), Object::Num(b)) => Object::Num(a - b),
            _ => panic!("Failed to subtract Objects {} and {}.", &self, &rhs),
        }
    }
}

impl std::ops::Mul for Object {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(a), Object::Num(b)) => Object::Num(a * b),
            _ => panic!("Failed to multiple Objects {} and {}.", &self, &rhs),
        }
    }
}

impl std::ops::Div for Object {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (&self, &rhs) {
            (Object::Num(a), Object::Num(b)) => Object::Num(a / b),
            _ => panic!("Failed to divide Objects {} and {}.", &self, &rhs),
        }
    }
}

impl std::cmp::PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Num(a), Object::Num(b)) if a.is_nan() && b.is_nan() => true,
            (Object::Num(a), Object::Num(b)) => a.eq(b),
            (Object::Bool(a), Object::Bool(b)) => a.eq(b),
            (Object::Str(a), Object::Str(b)) => a.eq(b),
            (Object::Null, Object::Null) => true,
            _ => false,
        }
    }
}

impl std::cmp::Eq for Object {}

impl std::cmp::PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Num(a), Object::Num(b)) => a.partial_cmp(b),
            _ => panic!("Failed to compare Objects {} and {}.", self, other),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn object_nan_eq_nan_is_true() {
        assert_eq!(Object::Num(f64::NAN), Object::Num(f64::NAN))
    }

    #[test]
    fn object_different_variants_are_not_equal() {
        assert_ne!(Object::Bool(true), Object::Num(1.23))
    }
}
