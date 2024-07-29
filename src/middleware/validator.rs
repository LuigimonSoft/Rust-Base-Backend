use crate::errors::ApiError;
use crate::errors::error_codes::ErrorCodes;

pub struct Rule<'a, T>{
    value: Option<&'a T>,
    error: Option<ApiError>,
    error_code: Option<ErrorCodes>
}

impl<'a, T> Rule<'a, T> {
    pub fn new(value: Option<&'a T>) -> Self {
        Rule { value, error: None, error_code: None }
    }

    pub fn with_error_code(mut self, error_code: ErrorCodes) -> Self {
        self.error_code = Some(error_code);
        self
    }

    pub fn validate(self) -> Result<(), ApiError> {
        if let Some(error) = self.error {
            Err(error)
        } else {
            Ok(())
        }
    }

    pub fn not_null(mut self) -> Self {
        if self.value.is_none() {
            self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
        }
        self
    }
}
// string
impl<'a> Rule<'a, String> {
    pub fn not_empty(mut self) -> Self {
        if let Some(s) = self.value {
            if s.is_empty() {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }

    pub fn max_length(mut self, max: usize) -> Self {
        if let Some(s) = self.value {
            if s.len() > max {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }
}
//u32
impl<'a> Rule<'a, u32> {
    pub fn within_range(mut self, min: u32, max: u32) -> Self {
        if let Some(n) = self.value {
            if *n < min || *n > max {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }
}
//bool
impl<'a> Rule<'a, bool> {
    pub fn is_true(mut self) -> Self {
        if let Some(b) = self.value {
            if !*b {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }
}
//f64
impl<'a> Rule<'a, f64> {
    pub fn is_integer(mut self) -> Self {
        if let Some(n) = self.value {
            if n.fract() != 0.0 {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }

    pub fn is_decimal(mut self) -> Self {
        if let Some(n) = self.value {
            if n.fract() == 0.0 {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }

    pub fn is_number(mut self) -> Self {
        if self.value.is_none() {
            self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
        }
        self
    }

    pub fn has_decimals(mut self) -> Self {
        if let Some(n) = self.value {
            if n.fract() == 0.0 {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }

    pub fn within_range(mut self, min: f64, max: f64) -> Self {
        if let Some(n) = self.value {
            if *n < min || *n > max {
                self.error = Some(ApiError::ErrorCode(self.error_code.clone().unwrap_or(ErrorCodes::Nodeclared)));
            }
        }
        self
    }
}