// Copyright (C) Call of Nil contributors
// SPDX-License-Identifier: AGPL-3.0-only

use crate::error::{Error, Result};
use crate::model::user::{NewUser, User};
use crate::sql_types::id::UserId;
use crate::sql_types::player_id::db_PlayerId;
use crate::{Database, conn};
use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel_async::RunQueryDsl;
use either::Either;

impl Database {
  pub async fn count_games_by_user(&self, id: db_PlayerId) -> Result<i64> {
    let user_id = self.get_user_id(id).await?;
    self.count_games_by_user_id(user_id).await
  }

  pub async fn count_games_by_user_id(&self, id: UserId) -> Result<i64> {
    use crate::schema::game;

    game::table
      .filter(game::created_by.eq(id))
      .count()
      .get_result(conn!(self))
      .await
      .map_err(Into::into)
  }

  pub async fn create_user(&self, new: &NewUser) -> Result<usize> {
    use crate::schema::user;

    let result = diesel::insert_into(user::table)
      .values(new)
      .execute(conn!(self))
      .await;

    if let Err(DieselError::DatabaseError(kind, _)) = &result
      && let DatabaseErrorKind::UniqueViolation = kind
    {
      Err(Error::UserAlreadyExists(new.player_id()))
    } else {
      Ok(result?)
    }
  }

  pub async fn get_user(&self, id: impl Into<db_PlayerId>) -> Result<User> {
    use crate::schema::user;

    let id: db_PlayerId = id.into();

    let result = user::table
      .filter(user::player_id.eq(&id))
      .select(User::as_select())
      .first(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(id)))
    } else {
      Ok(result?)
    }
  }

  pub async fn get_user_by_id(&self, id: UserId) -> Result<User> {
    use crate::schema::user;

    let result = user::table
      .find(id)
      .select(User::as_select())
      .first(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub async fn get_user_id(&self, id: db_PlayerId) -> Result<UserId> {
    use crate::schema::user;

    let result = user::table
      .filter(user::player_id.eq(&id))
      .select(user::id)
      .first(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Left(id)))
    } else {
      Ok(result?)
    }
  }

  pub async fn get_user_player_id(&self, id: UserId) -> Result<db_PlayerId> {
    use crate::schema::user;

    let result = user::table
      .find(id)
      .select(user::player_id)
      .first::<db_PlayerId>(conn!(self))
      .await;

    if let Err(DieselError::NotFound) = &result {
      Err(Error::UserNotFound(Either::Right(id)))
    } else {
      Ok(result?)
    }
  }

  pub async fn user_exists(&self, id: impl Into<db_PlayerId>) -> Result<bool> {
    use crate::schema::user;
    use diesel::dsl::{exists, select};

    let id: db_PlayerId = id.into();

    select(exists(user::table.filter(user::player_id.eq(id))))
      .get_result(conn!(self))
      .await
      .map_err(Into::into)
  }
}
