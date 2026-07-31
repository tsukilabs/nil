// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use axum::RequestExt;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::TypedHeader;
use headers::UserAgent;

pub async fn check_user_agent(mut request: Request, next: Next) -> Response {
  if request
    .extract_parts::<TypedHeader<UserAgent>>()
    .await
    .is_err()
  {
    return StatusCode::BAD_REQUEST.into_response();
  }

  next.run(request).await
}
