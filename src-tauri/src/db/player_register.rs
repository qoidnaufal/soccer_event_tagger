use super::Database;
use tauri::State;
use types::{AppError, MatchInfo, Payload, PlayerInfo, PlayerQuery, TeamInfo};

#[tauri::command]
pub async fn register_match_info(
    mut payload: MatchInfo,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    payload.assign_id();
    let db = state.db.lock().await;

    match db
        .create::<Option<MatchInfo>>(("match_info", &payload.match_id))
        .content(payload)
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(created) => {
            if let Some(match_info) = created {
                log::info!("Registered: {:?}", match_info);
            }
            Ok(())
        }
        Err(err) => {
            log::error!("Failed to register player: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn get_all_player_by_team_name(
    payload: String,
    state: State<'_, Database>,
) -> Result<Option<TeamInfo>, AppError> {
    let db = state.db.lock().await;

    let team_state = match payload.as_str() {
        "Home" => "team_home",
        "Away" => "team_away",
        _ => "*",
    };

    match db
        .query("SELECT * FROM match_info")
        // .bind(("team_state", team_state))
        // .bind(("team_name", &payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(mut n) => {
            let team_info = n
                .take::<Option<TeamInfo>>(team_state)
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

    db.select::<Option<PlayerInfo>>((&payload.team_name, payload.number))
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
