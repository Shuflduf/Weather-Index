use fake::{Fake, Faker};
use weather_index::run_report_dto::RunReportDTO;

#[tokio::main]
async fn main() {
    // let db = db::init_db().await.unwrap();
    let run: RunReportDTO = Faker.fake();
    println!("{run:?}");
}
