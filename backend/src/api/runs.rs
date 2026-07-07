use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{ColumnTrait, EntityTrait, IntoSimpleExpr, Order, QueryFilter, QueryOrder, Values};
use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        run_report::{self, Entity as RunReport},
        user,
    },
    error::{make_error, WIError},
    WIState,
};

// TODO: make not hardcoded
const DIFFICULTIES: [&str; 11] = [
    "DIFFICULTY_EASY_NAME",
    "DIFFICULTY_NORMAL_NAME",
    "DIFFICULTY_HARD_NAME",
    "ECLIPSE_1_NAME",
    "ECLIPSE_2_NAME",
    "ECLIPSE_3_NAME",
    "ECLIPSE_4_NAME",
    "ECLIPSE_5_NAME",
    "ECLIPSE_6_NAME",
    "ECLIPSE_7_NAME",
    "ECLIPSE_8_NAME",
];

const SURVIVORS: [&str; 18] = [
    "CommandoBody",
    "HuntressBody",
    "BanditBody",
    "ToolbotBody",
    "EngineerBody",
    "ArtificerBody",
    "MercBody",
    "TreebotBody",
    "LoaderBody",
    "CrocoBody",
    "CaptainBody",
    "RailgunnerBody",
    "VoidSurvivorBody",
    "SeekerBody",
    "FalseSonBody",
    "ChefBody",
    "DroneTechBody",
    "DrifterBody",
];

const ENDINGS: [&str; 9] = [
    "StandardLoss",
    "EscapeSequenceFailed",
    "PrismaticTrialEnding",
    "VoidEnding",
    "DecompileEnding",
    "RebirthEndingDef",
    "ObliterationEnding",
    "LimboEnding",
    "MainEnding",
];

#[derive(Serialize)]
pub struct RunReportWithUser {
    #[serde(flatten)]
    report: run_report::Model,
    user_image: Option<String>,
    user_username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListParams {
    #[serde(default = "default_sort_by")]
    by: String,
    #[serde(default = "default_sort")]
    sort: String,
}

fn default_sort_by() -> String {
    "id".to_string()
}
fn default_sort() -> String {
    "DESC".to_string()
}

fn to_order(sort: &str, list: &[&str]) -> Order {
    let mut list: Vec<sea_orm::Value> = list.iter().map(|s| (*s).into()).collect();
    if sort == "DESC" {
        list.reverse();
    }
    Order::Field(Values(list))
}

pub async fn list(
    State(state): State<Arc<WIState>>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<RunReportWithUser>>, WIError> {
    println!("{params:?}");
    let order = match params.sort.as_ref() {
        "DESC" => Order::Desc,
        "ASC" => Order::Asc,
        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            "field `order` should be either `DESC` or `ASC`".into(),
        ))?,
    };
    let (sort_by, order) = match params.by.as_ref() {
        "id" => (run_report::Column::Id, order),
        // "username" => user::Column::Username,
        "uploadTime" => (run_report::Column::UploadTime, order),

        // run info
        "survivor" => (
            run_report::Column::Survivor,
            to_order(&params.sort, &SURVIVORS),
        ),
        "startTime" => (run_report::Column::StartTime, order),
        "ending" => (run_report::Column::Ending, to_order(&params.sort, &ENDINGS)),
        "difficulty" => (
            run_report::Column::Difficulty,
            to_order(&params.sort, &DIFFICULTIES),
        ),
        "timeAliveSeconds" => (run_report::Column::TimeAliveSeconds, order),
        "artifacts" => (run_report::Column::Artifacts, order),
        "stagesCompleted" => (run_report::Column::StagesCompleted, order),
        "score" => (run_report::Column::Score, order),

        // items
        "itemsCollected" => (run_report::Column::ItemsCollected, order),

        // drones
        "dronesPurchased" => (run_report::Column::DronesPurchased, order),
        "turretsPurchased" => (run_report::Column::TurretsPurchased, order),

        // combat
        "kills" => (run_report::Column::Kills, order),
        "eliteKills" => (run_report::Column::EliteKills, order),
        "minionKills" => (run_report::Column::MinionKills, order),
        "deaths" => (run_report::Column::Deaths, order),

        // damage
        "damageDealt" => (run_report::Column::DamageDealt, order),
        "minionDamageDealt" => (run_report::Column::MinionDamageDealt, order),
        "damageTaken" => (run_report::Column::DamageTaken, order),
        "highestDamageDealt" => (run_report::Column::HighestDamageDealt, order),

        // healing
        "healingRecieved" => (run_report::Column::HealingRecieved, order),

        // progression
        "highestLevel" => (run_report::Column::HighestLevel, order),
        "goldCollected" => (run_report::Column::GoldCollected, order),
        "purchases" => (run_report::Column::Purchases, order),
        "goldPurchases" => (run_report::Column::GoldPurchases, order),
        "bloodPurchases" => (run_report::Column::BloodPurchases, order),
        "lunarPurchases" => (run_report::Column::LunarPurchases, order),

        // movement
        "distanceTraveled" => (run_report::Column::DistanceTraveledMetres, order),

        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            format!("{} is an invalid column name", params.by),
            // TODO: show actual column names to sort by
        ))?,
    };

    let reports = RunReport::find()
        .order_by(sort_by, order)
        .find_also_related(user::Entity)
        .all(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query runs: {e}"),
            )
        })?;

    let results = reports
        .into_iter()
        .map(|(report, user)| RunReportWithUser {
            report,
            user_username: user.as_ref().and_then(|u| u.username.clone()),
            user_image: user.as_ref().and_then(|u| u.image.clone()),
        })
        .collect();
    Ok(Json(results))
}

pub async fn get(
    Path(id): Path<i32>,
    State(state): State<Arc<WIState>>,
) -> Result<Json<RunReportWithUser>, WIError> {
    let (report, user) = RunReport::find()
        .filter(run_report::Column::Id.eq(id))
        .find_also_related(user::Entity)
        .one(&state.db)
        .await
        .map_err(|e| {
            make_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to query runs: {e}"),
            )
        })?
        .ok_or_else(|| make_error(StatusCode::NOT_FOUND, format!("Run {id} not found")))?;

    let result = RunReportWithUser {
        report,
        user_username: user.as_ref().and_then(|u| u.username.clone()),
        user_image: user.as_ref().and_then(|u| u.image.clone()),
    };
    Ok(Json(result))
}
