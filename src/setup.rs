use dotenv::var;
use sea_orm::*;

pub(super) async fn setup_db() -> Result<DatabaseConnection, DbErr> {
    let db_url = var("DB_URL").expect("DB_URL not set in .env file");
    let db_name = var("DB_NAME").expect("DB_NAME not set in .env file");

    let db = Database::connect(&db_url).await?;

    let db = match db.get_database_backend() {
        DatabaseBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
            ))
            .await?;

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }
        DatabaseBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("DROP DATABASE IF EXISTS \"{}\";", db_name),
            ))
            .await?;

            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", db_name),
            ))
            .await?;

            let url = format!("{}/{}", db_url, db_name);
            Database::connect(&url).await?
        }
        DatabaseBackend::Sqlite => db,
    };

    Ok(db)
}
