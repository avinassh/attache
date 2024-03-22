use std::env;
use libsql::Builder;


#[tokio::main]
async fn main() {
    let url = env::var("LIBSQL_URL").expect("LIBSQL_URL must be set");
    let token = env::var("LIBSQL_AUTH_TOKEN").unwrap_or_default();

    let db = Builder::new_remote(url, token).build().await.unwrap();
    let conn = db.connect().unwrap();
//    let tx = conn.transaction().await.unwrap();
    // tx.execute("CREATE TABLE test (x INTEGER)", ()).await.unwrap();
    // tx.execute("INSERT INTO test VALUES (10)", ()).await.unwrap();
    // tx.rollback().await.unwrap();

    let tx = conn.transaction().await.unwrap();
    tx.execute("ATTACH \"9c65ae5b-9132-4004-ac71-8ac2cfeba73f\" as s", ())
        .await
        .unwrap();
    let mut rows = tx.query("SELECT * FROM s.t", ()).await.unwrap();
    println!("{:?}", rows.next().await.unwrap());
}
