use crate::errors::ApiError;
use crate::errors::error_codes::ErrorCodes;
use warp::Rejection;

enum ValidationRule<T> {
    NotNull,
    NotEmpty,
    MaxLength(usize),
    WithinRange(T, T),
    IsInteger,
    IsDecimal,
    IsNumber,
    HasDecimals,
}

pub struct Rule<'a, T> {
    value: Option<&'a T>,
    rules: Vec<RuleItem<T>>
}

pub struct RuleItem<T> {
    validation_rule: ValidationRule<T>,
    error_code: Option<ErrorCodes>
}

pub trait Validation {
    fn validate(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_not_null(value: &Option<&Self>, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_not_empty(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_max_length(&self, max: usize, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_within_range(&self, min: &Self, max: &Self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>
    where
        Self: PartialOrd;
    fn check_is_integer(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_is_decimal(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_is_number(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
    fn check_has_decimals(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>;
}

impl<'a, T> Rule<'a, T>
where
    T: Validation + PartialOrd,
{
    pub fn new(value: Option<&'a T>) -> Self {
        Rule {
            value,
            rules: Vec::new()
        }
    }

    pub fn with_error_code(mut self, error_code: ErrorCodes) -> Self {
        if let Some(rule ) = self.rules.last_mut() {
            rule.error_code = Some(error_code);
        }
        self
    }

    pub fn validate(&self) -> Result<Option<&'a T>, Rejection> {
        let mut errors: Option<Vec<ErrorCodes>> = Some(Vec::new());

        for rule in &self.rules {
            let error = match &rule.validation_rule {
                ValidationRule::NotNull => T::check_not_null(&self.value, rule.error_code.clone()),
                ValidationRule::NotEmpty => {
                    if let Some(value) = self.value {
                        value.check_not_empty(rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::MaxLength(max) => {
                    if let Some(value) = self.value {
                        value.check_max_length(*max, rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::WithinRange(min, max) => {
                    if let Some(value) = self.value {
                        value.check_within_range(min, max, rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::IsInteger => {
                    if let Some(value) = self.value {
                        value.check_is_integer(rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::IsDecimal => {
                    if let Some(value) = self.value {
                        value.check_is_decimal(rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::IsNumber => {
                    if let Some(value) = self.value {
                        value.check_is_number(rule.error_code.clone())
                    } else {
                        None
                    }
                },
                ValidationRule::HasDecimals => {
                    if let Some(value) = self.value {
                        value.check_has_decimals(rule.error_code.clone())
                    } else {
                        None
                    }
                },
            };

            if let Some(error) = error {
                if let Some(ref mut error_list) = errors {
                    error_list.push(error);
                }
            }
        }

        if let Some(error_list) = errors.clone() {
            if error_list.is_empty() {
                Ok(self.value)
            } else {
                Err(warp::reject::custom(ApiError::MultipleErrors(errors)))
            }
        } else {
            Err(warp::reject::custom(ApiError::ErrorCode(ErrorCodes::Nodeclared)))
        }
    }
}

impl Validation for String {
    fn validate(&self, _error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_not_null(value: &Option<&Self>, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if value.is_none() {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_not_empty(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if self.is_empty() {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_max_length(&self, max: usize, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if self.len() > max {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_within_range(&self, _min: &Self, _max: &Self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>
    where
        Self: PartialOrd,
    {
        None
    }

    fn check_is_integer(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_decimal(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_number(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_has_decimals(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }
}

impl Validation for u32 {
    fn validate(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_not_null(value: &Option<&Self>, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if value.is_none() {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_not_empty(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_max_length(&self, _max: usize, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_within_range(&self, min: &Self, max: &Self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>
    where
        Self: PartialOrd,
    {
        if self < min || self > max {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_is_integer(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_decimal(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_number(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_has_decimals(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }
}

impl Validation for bool {
    fn validate(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_not_null(value: &Option<&Self>, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if value.is_none() {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_not_empty(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_max_length(&self, _max: usize, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_within_range(&self, _min: &Self, _max: &Self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>
    where
        Self: PartialOrd,
    {
        None
    }

    fn check_is_integer(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_decimal(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_is_number(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_has_decimals(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }
}

impl Validation for f64 {
    fn validate(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_not_null(value: &Option<&Self>, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if value.is_none() {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_not_empty(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_max_length(&self, _max: usize, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_within_range(&self, min: &Self, max: &Self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes>
    where
        Self: PartialOrd,
    {
        if self < min || self > max {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_is_integer(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if self.fract() != 0.0 {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_is_decimal(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if self.fract() == 0.0 {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }

    fn check_is_number(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        None
    }

    fn check_has_decimals(&self, error_code: Option<ErrorCodes>) -> Option<ErrorCodes> {
        if self.fract() == 0.0 {
            Some(error_code.unwrap_or(ErrorCodes::Nodeclared))
        } else {
            None
        }
    }
}

impl<'a> Rule<'a, String> {
    pub fn not_null(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::NotNull,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn not_empty(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::NotEmpty,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::MaxLength(max),
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }
}

impl<'a> Rule<'a, u32> {
    pub fn within_range(mut self, min: u32, max: u32) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::WithinRange(min, max),
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }
}

impl<'a> Rule<'a, f64> {
    pub fn is_integer(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::IsInteger,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn is_decimal(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::IsDecimal,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn is_number(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::IsNumber,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn has_decimals(mut self) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::HasDecimals,
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }

    pub fn within_range(mut self, min: f64, max: f64) -> Self {
        let rule_item = RuleItem {
            validation_rule: ValidationRule::WithinRange(min, max),
            error_code: None
        };
        self.rules.push(rule_item);
        self
    }
}

