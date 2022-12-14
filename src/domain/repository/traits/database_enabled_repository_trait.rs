use crate::core::database::{DbPoolState, DB};

pub trait DatabaseEnabledRepositoryTrait {
    fn set_db(&mut self, db_conn: DbPoolState) -> &mut Self;
    fn get_db(&self) -> DB;
}
