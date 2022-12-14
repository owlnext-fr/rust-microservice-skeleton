use crate::{
    core::database::DbPoolState,
    domain::repository::traits::database_enabled_repository_trait::DatabaseEnabledRepositoryTrait,
};

pub trait DbEnabledMiddlewareTrait<T>
where
    T: DatabaseEnabledRepositoryTrait,
{
    fn setup_db(&mut self, db_conn: DbPoolState) -> &mut Self;
    fn get_repository(&self) -> T;
}
