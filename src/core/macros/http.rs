#[macro_export]
macro_rules! http_exception {
    ($status:expr) => {{
        return Err(ApiResponse::from_status($status));
    }};
    ($status:expr, $reason:expr) => {{
        return Err(ApiResponse::from_status_with_reason($status, $reason));
    }};
}

#[macro_export]
macro_rules! deny_access_unless_granted {
    ($security:expr, $user:expr, $subject:expr, $right:expr) => {{
        use $crate::http_exception;

        if !$security.has_access($subject, $right, $user) {
            http_exception!(Status::Unauthorized);
        }
    }};
}

#[macro_export]
macro_rules! http_ok {
    ($output:expr) => {{
        return Ok(ApiResponse::ok(Json($output)));
    }};
}
