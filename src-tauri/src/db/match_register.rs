use super::Database;
use tauri::State;
use types::{AppError, MatchInfo, PlayerInfo, PlayerQuery, TeamInfo};

#[tauri::command]
pub async fn register_match_info(
    mut payload: MatchInfo,
    state: State<'_, Database>,
) -> Result<String, AppError> {
    payload.assign_id();
    let db = state.db.lock().await;

    match db
        .create::<Option<MatchInfo>>(("match_info", &payload.match_id))
        .content(payload.clone())
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(created) => {
            if let Some(match_info) = created {
                log::info!("Registered match: {:?}", match_info.match_id);
                Ok(match_info.match_id)
            } else {
                let response = format!(
                    "Failed to register match info: match id {} already exist",
                    payload.match_id
                );
                Err(AppError::DatabaseError(response))
            }
        }
        Err(err) => {
            log::error!("Failed to register match info: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn get_match_info(
    payload: String,
    state: State<'_, Database>,
) -> Result<Option<MatchInfo>, AppError> {
    let db = state.db.lock().await;

    match db
        .query("SELECT * FROM match_info WHERE match_id = $match_id")
        .bind(("match_id", &payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(mut res) => {
            let match_info = res
                .take(0)
                .map_err(|err| AppError::DatabaseError(err.to_string()));
            match_info
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn get_all_player_by_team_name(
    payload: String,
    state: State<'_, Database>,
) -> Result<Option<TeamInfo>, AppError> {
    let db = state.db.lock().await;

    match db
        .query("SELECT * FROM match_info WHERE team_name = $team_name")
        .bind(("team_name", &payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(mut n) => {
            let team_info = n
                .take::<Option<TeamInfo>>(0)
                .map_err(|err| AppError::DatabaseError(err.to_string()));
            team_info
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn get_player_by_query(
    payload: PlayerQuery,
    state: State<'_, Database>,
) -> Result<Option<PlayerInfo>, AppError> {
    let db = state.db.lock().await;

    db.select::<Option<PlayerInfo>>(("match_info", payload.number))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

#[tauri::command]
pub async fn delete_player_by_info(
    payload: PlayerInfo,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Option<PlayerInfo>>((&payload.team_name, payload.number))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(deleted) => {
            if let Some(player) = deleted {
                log::info!("Deleted player: {:?}", player);
            }

            Ok(())
        }
        Err(err) => {
            log::error!("Failed to delete player: {}", err);
            Err(err)
        }
    }
}
