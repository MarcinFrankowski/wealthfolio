use diesel::prelude::*;
use diesel::r2d2::{self, Pool};
use diesel::sqlite::SqliteConnection;
use std::sync::Arc;

use crate::errors::Result;
use crate::schema::accounts;
use crate::schema::accounts::dsl::*;
use crate::db::get_connection;

use super::accounts_model::{Account, AccountDB, AccountUpdate, NewAccount};
use super::accounts_traits::AccountRepositoryTrait;

/// Repository for managing account data in the database
pub struct AccountRepository {
    pool: Arc<Pool<r2d2::ConnectionManager<SqliteConnection>>>,
}

impl AccountRepository {
    /// Creates a new AccountRepository instance
    pub fn new(pool: Arc<Pool<r2d2::ConnectionManager<SqliteConnection>>>) -> Self {
        Self { pool }
    }
}

// Implement the trait
impl AccountRepositoryTrait for AccountRepository {
    /// Creates a new account within a given database transaction
    fn create_in_transaction(
        &self,
        new_account: NewAccount,
        conn: &mut SqliteConnection
    ) -> Result<Account> {
        new_account.validate()?;

        let mut account_db: AccountDB = new_account.into();
        account_db.id = uuid::Uuid::new_v4().to_string();
        
        diesel::insert_into(accounts::table)
            .values(&account_db)
            .execute(conn)?;

        Ok(account_db.into())
    }

    /// Updates an existing account in the database
    fn update(&self, account_update: AccountUpdate) -> Result<Account> {
        account_update.validate()?;

        let mut conn = get_connection(&self.pool)?;

        let mut account_db: AccountDB = account_update.into();
        let existing = accounts
            .find(&account_db.id)
            .first::<AccountDB>(&mut conn)?;

        account_db.currency = existing.currency;
        account_db.created_at = existing.created_at;
        account_db.updated_at = chrono::Utc::now().naive_utc();

        diesel::update(accounts.find(&account_db.id))
            .set(&account_db)
            .execute(&mut conn)?;

        Ok(account_db.into())
    }

    /// Retrieves an account by its ID
    fn get_by_id(&self, account_id: &str) -> Result<Account> {
        let mut conn = get_connection(&self.pool)?;

        let account = accounts
            .find(account_id)
            .first::<AccountDB>(&mut conn)?;

        Ok(account.into())
    }

    /// Lists accounts in the database, optionally filtering by active status and account IDs
    fn list(&self, is_active_filter: Option<bool>, account_ids: Option<&[String]>) -> Result<Vec<Account>> {
        let mut conn = get_connection(&self.pool)?;

        let mut query = accounts::table.into_boxed();
        
        if let Some(active) = is_active_filter {
            query = query.filter(is_active.eq(active));
        }

        if let Some(ids) = account_ids {
            query = query.filter(id.eq_any(ids));
        }

        let results = query
            .order((is_active.desc(), name.asc()))
            .load::<AccountDB>(&mut conn)?;

        let accounts_list: Vec<Account> = results.into_iter().map(Account::from).collect();
        Ok(accounts_list)
    }

    /// Deletes an account by its ID and returns the number of deleted records
    fn delete(&self, account_id: &str) -> Result<usize> {
        let mut conn = get_connection(&self.pool)?;

        let affected = diesel::delete(accounts.find(account_id))
            .execute(&mut conn)?;

        Ok(affected)
    }
} 