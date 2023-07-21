
use surrealdb::{
    engine::remote::http::{Client, Https},
    opt::auth::Database,
    sql::Thing,
    Surreal
};

#[derive(Debug, Deserialize)]
struct Record {
    #[allow(dead_code)]
    id: Thing,
}

pub async fn init(server: String, auth: Database<'_>, namespace: String, database: String) -> Result<Surreal<Client>, Box<dyn std::error::Error>> {
    info!("Connecting to Surreal database {} <{}/{}>", server, namespace, database);

    let db = Surreal::new::<Https>(server).await?;
    db.signin( auth ).await?;
    db
        .use_ns(namespace)
        .use_db(database)
        .await?;

    info!("Surrealdb connection successful");

    Ok(db)
}
