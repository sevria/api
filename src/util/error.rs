#[derive(Debug, thiserror::Error)]
#[error("{} {}", .code, .message)]
pub struct Error {
    pub code: u16,
    pub message: String,
}

pub fn already_present() -> Error {
    Error {
        code: 409,
        message: String::from("already present"),
    }
}

pub fn already_present_with_message(msg: &str) -> Error {
    Error {
        code: 409,
        message: String::from(msg),
    }
}

pub fn is_already_present(err: &Error) -> bool {
    err.code == 409
}

pub fn internal() -> Error {
    Error {
        code: 500,
        message: String::from("internal error"),
    }
}

pub fn internal_with_message(msg: &str) -> Error {
    Error {
        code: 500,
        message: String::from(msg),
    }
}

pub fn is_internal(err: &Error) -> bool {
    err.code == 500
}

pub fn invalid_argument() -> Error {
    Error {
        code: 400,
        message: String::from("invalid argument"),
    }
}

pub fn invalid_argument_with_message(msg: &str) -> Error {
    Error {
        code: 400,
        message: String::from(msg),
    }
}

pub fn is_invalid_argument(err: &Error) -> bool {
    err.code == 400
}

pub fn not_found() -> Error {
    Error {
        code: 404,
        message: String::from("not found"),
    }
}

pub fn not_found_with_message(msg: &str) -> Error {
    Error {
        code: 404,
        message: String::from(msg),
    }
}

pub fn is_not_found(err: &Error) -> bool {
    err.code == 404
}

pub fn permission_denied() -> Error {
    Error {
        code: 403,
        message: String::from("permission denied"),
    }
}

pub fn permission_denied_with_message(msg: &str) -> Error {
    Error {
        code: 403,
        message: String::from(msg),
    }
}

pub fn is_permission_denied(err: &Error) -> bool {
    err.code == 403
}

pub fn unauthenticated() -> Error {
    Error {
        code: 401,
        message: String::from("unauthenticated"),
    }
}

pub fn unauthenticated_with_message(msg: &str) -> Error {
    Error {
        code: 401,
        message: String::from(msg),
    }
}

pub fn is_unauthenticated(err: &Error) -> bool {
    err.code == 401
}
