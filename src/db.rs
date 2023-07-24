
use surrealdb::{
    engine::remote::http::{Client, Https},
    opt::auth::Database,
    Surreal
};

/// A generic Record for testing purposes
/// 
/// allows for returning of serialized data with a generic struct
#[cfg(not(profile="release"))]
#[derive(Debug, Deserialize)]
pub(crate) struct Record {
    #[allow(dead_code)]
    id: surrealdb::sql::Thing,
}

pub async fn init(server: String, auth: Database<'_>, namespace: String, database: String) -> Result<Surreal<Client>, Box<dyn std::error::Error>> {
    debug!("Connecting to Surreal database {} <{}/{}>", server, namespace, database);

    let db = Surreal::new::<Https>(server).await?;
    db.signin( auth ).await?;
    db
        .use_ns(namespace)
        .use_db(database)
        .await?;

    info!("Surrealdb connection successful");

    Ok(db)
}
