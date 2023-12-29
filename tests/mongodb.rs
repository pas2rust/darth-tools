use darth_tools::{DarthTools, MongodbTrait};
use dotenv::dotenv;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::env;
use tokio;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

#[tokio::test]
async fn mongodb_connect_is_impossible() {
    let connect =
        DarthTools::mongodb_connect::<User>("uri", "dbname", "table")
            .await;
    match connect {
        Ok(table) => assert!(false, "{:#?}", table.collection),
        Err(_) => assert!(true),
    }
}

#[tokio::test]
async fn mongodb_connect_is_possible() {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("env MONGODB_URI error");
    let connect = DarthTools::mongodb_connect::<User>(
        uri.as_str(),
        "test",
        "table",
    )
    .await;
    match connect {
        Ok(table) => {
            let insert = table
                .insert_one(
                    5,
                    User {
                        email: "email".to_string(),
                        id: "1".to_string(),
                        name: "name".to_string(),
                    },
                    None,
                )
                .await;
            match insert {
                Ok(_) => assert!(true),
                Err(err) => assert!(false, "{:#?}", err),
            };
            let doc = doc! { "id": "1" };
            let del = table.delete_one(5, doc, None).await;
            match del {
                Ok(_) => assert!(true),
                Err(err) => assert!(false, "{:#?}", err),
            };
        }
        Err(err) => assert!(false, "{:#?}", err),
    }
}
