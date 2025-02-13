use axum::response::Response;
use nil_core::Error;

#[macro_export]
macro_rules! res {
  ($status:ident) => {{
    use axum::body::Body;
    use axum::http::StatusCode;
    use axum::response::Response;

    Response::builder()
      .status(StatusCode::$status)
      .body(Body::empty())
      .unwrap()
  }};
  ($status:ident, $data:expr) => {{
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    (StatusCode::$status, $data).into_response()
  }};
}

#[allow(clippy::needless_pass_by_value)]
pub fn from_err(err: Error) -> Response {
  let text = err.to_string();
  match err {
    Error::NoPlayerToSchedule => res!(BAD_REQUEST, text),
    Error::NotAVillage(_) => res!(BAD_REQUEST, text),
    Error::NotTurnOf(_) => res!(FORBIDDEN, text),
    Error::PlayerNotFound(_) => res!(NOT_FOUND, text),
    Error::UnitNotFound(_) => res!(NOT_FOUND, text),
    _ => res!(INTERNAL_SERVER_ERROR, text),
  }
}
