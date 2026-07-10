use fake::{Fake, Faker};
use sea_orm::{ActiveModelTrait, ActiveValue::Set};
use weather_index::{db, entity::run_report, run_report_dto::RunReportDTO};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();
    let db = db::init_db().await.unwrap();
    let run: RunReportDTO = Faker.fake();
    println!("{run:?}");

    let mut run: run_report::ActiveModel = run.try_into().unwrap();
    run.user_id = Set("1".to_string());
    println!("{run:#?}");
    run.insert(&db).await.unwrap();
}
