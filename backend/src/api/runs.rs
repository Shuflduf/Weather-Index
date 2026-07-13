use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, Query, State},
    Json,
};
use reqwest::StatusCode;
use sea_orm::{
    prelude::Expr, ColumnTrait, Condition, EntityTrait, IntoSimpleExpr, Order, PaginatorTrait,
    QueryFilter, QueryOrder, Values,
};
use serde::{Deserialize, Serialize};

use crate::{
    entity::{
        run_report::{self, Entity as RunReport},
        user,
    },
    error::{db_error, make_error, WIError},
    WIState,
};

// TODO: make not hardcoded
pub const ORDERED_DIFFICULTIES: [&str; 11] = [
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

pub const ORDERED_SURVIVORS: [&str; 18] = [
    "CommandoBody",
    "HuntressBody",
    "BanditBody",
    "ToolbotBody",
    "EngiBody",
    "MageBody",
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

const ORDERED_ENDINGS: [&str; 9] = [
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
#[serde(rename_all = "camelCase")]
pub struct RunReportWithUser {
    #[serde(flatten)]
    report: run_report::Model,
    user_image: Option<String>,
    user_username: Option<String>,
    user_display_username: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ListParams {
    #[serde(default = "default_sort_by")]
    by: String,
    #[serde(default = "default_sort")]
    sort: String,
    #[serde(default)]
    page: u64,
    filters: Option<String>,
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
    Order::Field(Values(list))
}

pub async fn list(
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
    // println!("{:?}", params.page);
    let order = match params.sort.as_ref() {
        "DESC" => Order::Desc,
        "ASC" => Order::Asc,
        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            "field `order` should be either `DESC` or `ASC`".into(),
        ))?,
    };
    let (sort_by, order) = match params.by.as_ref() {
        "id" => (run_report::Column::Id.into_simple_expr(), order),
        "player" => (Expr::col((user::Entity, user::Column::Username)), order),
        "uploadTime" => (run_report::Column::UploadTime.into_simple_expr(), order),

        // run info
        "survivor" => (
            run_report::Column::Survivor.into_simple_expr(),
            to_order(&params.sort, &ORDERED_SURVIVORS),
        ),
        "startTime" => (run_report::Column::StartTime.into_simple_expr(), order),
        "ending" => (
            run_report::Column::Ending.into_simple_expr(),
            to_order(&params.sort, &ORDERED_ENDINGS),
        ),
        "difficulty" => (
            run_report::Column::Difficulty.into_simple_expr(),
            to_order(&params.sort, &ORDERED_DIFFICULTIES),
        ),
        "timeAlive" => (
            run_report::Column::TimeAliveSeconds.into_simple_expr(),
            order,
        ),
        "artifacts" => (run_report::Column::Artifacts.into_simple_expr(), order),
        "stagesCompleted" => (
            run_report::Column::StagesCompleted.into_simple_expr(),
            order,
        ),
        "score" => (run_report::Column::Score.into_simple_expr(), order),

        // items
        "itemsCollected" => (run_report::Column::ItemsCollected.into_simple_expr(), order),

        // drones
        "dronesPurchased" => (
            run_report::Column::DronesPurchased.into_simple_expr(),
            order,
        ),
        "turretsPurchased" => (
            run_report::Column::TurretsPurchased.into_simple_expr(),
            order,
        ),

        // combat
        "kills" => (run_report::Column::Kills.into_simple_expr(), order),
        "eliteKills" => (run_report::Column::EliteKills.into_simple_expr(), order),
        "minionKills" => (run_report::Column::MinionKills.into_simple_expr(), order),
        "deaths" => (run_report::Column::Deaths.into_simple_expr(), order),

        // damage
        "damageDealt" => (run_report::Column::DamageDealt.into_simple_expr(), order),
        "minionDamageDealt" => (
            run_report::Column::MinionDamageDealt.into_simple_expr(),
            order,
        ),
        "damageTaken" => (run_report::Column::DamageTaken.into_simple_expr(), order),
        "highestDamageDealt" => (
            run_report::Column::HighestDamageDealt.into_simple_expr(),
            order,
        ),

        // healing
        "healingRecieved" => (
            run_report::Column::HealingRecieved.into_simple_expr(),
            order,
        ),

        // progression
        "highestLevel" => (run_report::Column::HighestLevel.into_simple_expr(), order),
        "goldCollected" => (run_report::Column::GoldCollected.into_simple_expr(), order),
        "purchases" => (run_report::Column::Purchases.into_simple_expr(), order),
        "goldPurchases" => (run_report::Column::GoldPurchases.into_simple_expr(), order),
        "bloodPurchases" => (run_report::Column::BloodPurchases.into_simple_expr(), order),
        "lunarPurchases" => (run_report::Column::LunarPurchases.into_simple_expr(), order),

        // movement
        "distanceTraveled" => (
            run_report::Column::DistanceTraveledMetres.into_simple_expr(),
            order,
        ),

        _ => Err(make_error(
            StatusCode::BAD_REQUEST,
            format!("{} is an invalid column name", params.by),
            // TODO: show actual column names to sort by
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
        (
            "distanceTraveled",
            run_report::Column::DistanceTraveledMetres,
        ),
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
    let report_pages = RunReport::find()
        .order_by(sort_by, order)
        .filter(condition)
        .find_also_related(user::Entity)
        .paginate(&state.db, 10);
    let reports = report_pages.fetch_page(params.page).await.map_err(|e| {
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
            user_display_username: user.as_ref().and_then(|u| u.display_username.clone()),
            user_image: user.as_ref().and_then(|u| u.image.clone()),
        })
        .collect();
    Ok(Json(ListReturn {
        total: report_pages.num_items().await.map_err(db_error)?,
        runs: results,
    }))
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
        user_display_username: user.as_ref().and_then(|u| u.display_username.clone()),
        user_image: user.as_ref().and_then(|u| u.image.clone()),
    };
    Ok(Json(result))
}
