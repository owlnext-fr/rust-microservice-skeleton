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

        let vote = $security.has_access($subject, $right, $user, None);

        if vote.is_err() {
            let root_cause_message = format!("{}", vote.err().unwrap().root_cause());

            http_exception!(Status::Unauthorized, &root_cause_message);
        } else {
            if let Ok(voted) = vote {
                if !voted {
                    http_exception!(Status::Unauthorized, "Unauthorized");
                }
            }
        }
    }};
    ($security:expr, $user:expr, $subject:expr, $right:expr, $context:expr) => {{
        use $crate::http_exception;

        let vote = $security.has_access($subject, $right, $user, Some($context));

        if vote.is_err() {
            let root_cause_message = format!("{}", vote.err().unwrap().root_cause());

            http_exception!(Status::Unauthorized, &root_cause_message);
        } else {
            if let Ok(voted) = vote {
                if !voted {
                    http_exception!(Status::Unauthorized, "Unauthorized");
                }
            }
        }
    }};
}

#[macro_export]
macro_rules! http_ok {
    ($output:expr) => {{
        return Ok(ApiResponse::ok(Json($output)));
    }};
}

#[macro_export]
macro_rules! http_no_content {
    () => {{
        return Ok(ApiResponse::no_content());
    }};
}
