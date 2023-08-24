use crate::model::message::Message;
use scylla::IntoTypedRows;
use scylla::{Session};

// https://github.com/scylladb/scylla-rust-driver/blob/e1fa6ff346cd14663fc1a0e483b68045885ec306/examples/basic.rs
pub async fn scylla_db_queries(session: Session) {

    session.query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class' : 'NetworkTopologyStrategy', 'replication_factor' : 1}", &[]).await.unwrap();

    // session
    // .query(
    //     "DROP TABLE IF EXISTS ks.messages",
    //     &[],
    // )
    // .await.unwrap();

    session
    .query(
        "CREATE TABLE IF NOT EXISTS ks.messages (message_id bigint, message_text text, primary key (message_id))",
        &[],
    )
    .await.unwrap();

    session
    .query(
        "CREATE TYPE IF NOT EXISTS ks.message (message_id bigint, message_text text)",
        &[],
    )
    .await.unwrap();

    // одно значение
    // "DELETE FROM ks.messages WHERE message_id = ?"


    // session
    // .query("INSERT INTO ks.messages (message_id, message_text) VALUES (0, 'самое первое')", &[])
    // .await.unwrap();

    // session
    // .query("INSERT INTO ks.messages (message_id, message_text) VALUES (1, 'это вообще работает ?')", &[])
    // .await.unwrap();

    // session
    // .query("INSERT INTO ks.messages (message_id, message_text) VALUES (2, 'правда работает !')", &[])
    // .await.unwrap();


    // let prepared = session
    //     .prepare("INSERT INTO ks.messages (message_id, message_text) VALUES (?, ?)")
    //     .await
    //     .unwrap();
    // session
    //     .execute(&prepared, (1, "это вообще работает ?"))
    //     .await
    //     .unwrap();
    // session
    //     .execute(&prepared, (2, "правда работает !"))
    //     .await
    //     .unwrap();

    // typed rows работает !
    if let Some(rows) = session.query("SELECT * FROM ks.messages", &[]).await.unwrap().rows {
        for row_data in rows.into_typed::<Message>() {
            let row_data = row_data.unwrap();
            println!("row_data: {:?}", row_data);
        }
    }

}

// работает !
// if let Some(rows) = session.query("SELECT message_id, message_text FROM ks.messages", &[]).await.unwrap().rows {
//     for row in rows {
//         let a = row.columns[0].as_ref().unwrap().as_int().unwrap();
//         let b = row.columns[1].as_ref().unwrap().as_text().unwrap();
//         println!("a, b: {}, {}", a, b);
//     }
// }