use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{
    ColumnTrait, Condition, EntityTrait, IntoSimpleExpr, Order, PaginatorTrait, QueryFilter,
    QueryOrder,
};
use serde::{Deserialize, Serialize};

use crate::{
    api::runs::RunReportWithUser,
    data::{ORDERED_DIFFICULTIES, ORDERED_ENDINGS, ORDERED_SURVIVORS},
    entity::{run_report, user},
    error::{db_error, make_error, WIError},
    WIState,
};

#[derive(Deserialize, Debug)]
pub struct ListParams {
    // prop name, ex. "id"
    #[serde(default = "default_sort_by")]
    by: String,

    // ASC or DESC
    #[serde(default = "default_sort")]
    sort: String,

    // ASC or DESC
    #[serde(default = "default_sort")]
    fallback_sort: String,

    #[serde(default)]
    // numbmer, ex. 4
    page: u64,

    // json object of props to array of filters. some props only have one filter, ex. {"id":[">10"],"survivor":["CommandoBody","BanditBody"]}
    filters: Option<String>,

    // json array of props, ex. ["id", "score"]
    only: Option<String>,
}

#[derive(Serialize)]
pub struct ListReturn {
    total: u64,
    runs: Vec<RunReportWithUser>,
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
    Order::Field(sea_orm::Values(list))
}

pub async fn get(
    State(state): State<Arc<WIState>>,
    Query(params): Query<ListParams>,
) -> Result<Json<ListReturn>, WIError> {
    let filters: HashMap<String, Vec<String>> = params
        .filters
        .as_deref()
        .map(serde_json::from_str)
        .transpose()
        .map_err(|e| make_error(StatusCode::BAD_REQUEST, format!("Invalid filter json: {e}")))?
        .unwrap_or_default();
    let columns: HashMap<&str, run_report::Column> = HashMap::from([
        ("id", run_report::Column::Id),
        ("uploadTime", run_report::Column::UploadTime),
        // run info
        ("survivor", run_report::Column::Survivor),
        ("startTime", run_report::Column::StartTime),
        ("ending", run_report::Column::Ending),
        ("difficulty", run_report::Column::Difficulty),
        ("timeAlive", run_report::Column::TimeAliveSeconds),
        ("artifacts", run_report::Column::Artifacts),
        ("stagesCompleted", run_report::Column::StagesCompleted),
        ("score", run_report::Column::Score),
        // pickups
        ("itemsCollected", run_report::Column::ItemsCollected),
        ("dronesPurchased", run_report::Column::DronesPurchased),
        ("turretsPurchased", run_report::Column::TurretsPurchased),
        // combat
        ("kills", run_report::Column::Kills),
        ("eliteKills", run_report::Column::EliteKills),
        ("minionKills", run_report::Column::MinionKills),
        ("deaths", run_report::Column::Deaths),
        ("damageDealt", run_report::Column::DamageDealt),
        ("minionDamageDealt", run_report::Column::MinionDamageDealt),
        ("damageTaken", run_report::Column::DamageTaken),
        ("highestDamageDealt", run_report::Column::HighestDamageDealt),
        // healing
        ("healingRecieved", run_report::Column::HealingRecieved),
        // progression
        ("highestLevel", run_report::Column::HighestLevel),
        ("goldCollected", run_report::Column::GoldCollected),
        ("purchases", run_report::Column::Purchases),
        ("goldPurchases", run_report::Column::GoldPurchases),
        ("bloodPurchases", run_report::Column::BloodPurchases),
        ("lunarPurchases", run_report::Column::LunarPurchases),
        // movement
        ("distanceTraveled", run_report::Column::DistanceTraveled),
    ]);
    let sort_by = if params.by == "player" {
        user::Column::Username.into_simple_expr()
    } else {
        columns
            .get::<str>(params.by.as_ref())
            .ok_or(make_error(
                StatusCode::BAD_REQUEST,
                format!("{} is an invalid column name", params.by),
            ))?
            .into_simple_expr()
    };
    let order = match params.by.as_ref() {
        // run info
        "survivor" => to_order(&params.sort, &ORDERED_SURVIVORS),
        "ending" => to_order(&params.sort, &ORDERED_ENDINGS),
        "difficulty" => to_order(&params.sort, &ORDERED_DIFFICULTIES),
        _ => match params.sort.as_ref() {
            "DESC" => Order::Desc,
            "ASC" => Order::Asc,
            _ => Err(make_error(
                StatusCode::BAD_REQUEST,
                "field `order` should be either `DESC` or `ASC`".into(),
            ))?,
        },
    };
    let fallback_order = match params.fallback_sort.as_ref() {
        "DESC" => Order::Desc,
        "ASC" => Order::Asc,
        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            "field `fallback_order` should be either `DESC` or `ASC`".into(),
        ))?,
    };

    let mut condition = Condition::all();
    let numerical_conditions = [
        ("id", run_report::Column::Id),
        //
        ("timeAlive", run_report::Column::TimeAliveSeconds),
        ("stagesCompleted", run_report::Column::StagesCompleted),
        ("score", run_report::Column::Score),
        //
        ("itemsCollected", run_report::Column::ItemsCollected),
        //
        ("dronesPurchased", run_report::Column::DronesPurchased),
        ("turretsPurchased", run_report::Column::TurretsPurchased),
        //
        ("kills", run_report::Column::Kills),
        ("eliteKills", run_report::Column::EliteKills),
        ("minionKills", run_report::Column::MinionKills),
        ("deaths", run_report::Column::Deaths),
        //
        ("damageDealt", run_report::Column::DamageDealt),
        ("minionDamageDealt", run_report::Column::MinionDamageDealt),
        ("damageTaken", run_report::Column::DamageTaken),
        ("highestDamageDealt", run_report::Column::HighestDamageDealt),
        //
        ("healingRecieved", run_report::Column::HealingRecieved),
        //
        ("highestLevel", run_report::Column::HighestLevel),
        ("goldCollected", run_report::Column::GoldCollected),
        ("purchases", run_report::Column::Purchases),
        ("goldPurchases", run_report::Column::GoldPurchases),
        ("bloodPurchases", run_report::Column::BloodPurchases),
        ("lunarPurchases", run_report::Column::LunarPurchases),
        //
        ("distanceTraveled", run_report::Column::DistanceTraveled),
    ]
    .map(|(name, col)| {
        filters.get(name).and_then(|f| f.first()).and_then(|f| {
            let (sign, rest) = f.split_at(1);
            let value = rest.parse::<i64>().ok()?;
            Some(match sign {
                ">" => col.gt(value),
                "<" => col.lt(value),
                _ => return None,
            })
        })
    });
    let time_conditions = [
        ("startTime", run_report::Column::StartTime),
        ("uploadTime", run_report::Column::UploadTime),
    ]
    .map(|(name, col)| {
        filters.get(name).and_then(|f| f.first()).and_then(|f| {
            let (sign, rest) = f.split_at(1);
            let value = chrono::NaiveDateTime::parse_from_str(rest, "%Y-%m-%dT%H:%M").ok()?;
            Some(match sign {
                ">" => col.gt(value),
                "<" => col.lt(value),
                _ => return None,
            })
        })
    });
    let player_filter = filters.get("player").map(|f| {
        if f[0].starts_with("@") {
            user::Column::Username.eq(&f[0][1..])
        } else {
            user::Column::Username.contains(&f[0])
        }
    });

    for filter in [
        player_filter,
        filters
            .get("difficulty")
            .map(|f| run_report::Column::Difficulty.is_in(f)),
        filters
            .get("survivor")
            .map(|f| run_report::Column::Survivor.is_in(f)),
        filters
            .get("ending")
            .map(|f| run_report::Column::Ending.is_in(f)),
    ]
    .into_iter()
    .chain(numerical_conditions)
    .chain(time_conditions)
    {
        condition = condition.add_option(filter);
    }
    let report_pages = run_report::Entity::find()
        .order_by(sort_by, order)
        .order_by(run_report::Column::Id, fallback_order)
        .filter(condition)
        .find_also_related(user::Entity)
        .paginate(&state.db, 10);
    let reports = report_pages.fetch_page(params.page).await.map_err(|e| {
        make_error(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to query runs: {e}"),
        )
    })?;

    let results: Vec<RunReportWithUser> = reports
        .into_iter()
        .map(|(report, user)| {
            let mut report = serde_json::to_value(report).unwrap();
            if let Some(ref fields) = params.only {
                if let serde_json::Value::Object(ref mut map) = report {
                    map.retain(|k, _| fields.contains(k));
                };
            };
            RunReportWithUser {
                report,
                user_username: user.as_ref().and_then(|u| u.username.clone()),
                user_display_username: user.as_ref().and_then(|u| u.display_username.clone()),
                user_image: user.as_ref().and_then(|u| u.image.clone()),
            }
        })
        .collect();
    Ok(Json(ListReturn {
        total: report_pages.num_items().await.map_err(db_error)?,
        runs: results,
    }))
}
