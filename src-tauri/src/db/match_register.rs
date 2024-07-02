use super::Database;
use tauri::State;
use types::{AppError, MatchInfo, PlayerInfo, TeamInfoQuery};

#[tauri::command]
pub async fn register_match_info(
    payload: MatchInfo,
    state: State<'_, Database>,
) -> Result<MatchInfo, AppError> {
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
                Ok(match_info)
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
pub async fn register_player_info(
    payload: PlayerInfo,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .create::<Option<PlayerInfo>>((&payload.match_id, &payload.player_id))
        .content(payload.clone())
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(created) => {
            if let Some(player_info) = created {
                log::info!(
                    "Registered player: {} for match id: {}",
                    player_info.player_id,
                    player_info.match_id
                );
                Ok(())
            } else {
                let response = format!(
                    "Failed to register player: {} for match id: {}",
                    payload.player_id, payload.match_id
                );
                Err(AppError::DatabaseError(response))
            }
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn get_all_match_info(state: State<'_, Database>) -> Result<Vec<MatchInfo>, AppError> {
    let db = state.db.lock().await;

    db.select::<Vec<MatchInfo>>("match_info")
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

#[tauri::command]
pub async fn get_all_players_from_match_id(
    payload: String,
    state: State<'_, Database>,
) -> Result<Vec<PlayerInfo>, AppError> {
    let db = state.db.lock().await;

    db.select::<Vec<PlayerInfo>>(payload)
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

#[tauri::command]
pub async fn query_team_info(
    payload: TeamInfoQuery,
    state: State<'_, Database>,
) -> Result<Vec<PlayerInfo>, AppError> {
    let db = state.db.lock().await;

    match db
        .query("SELECT * FROM type::table($match_id) WHERE team_state = $team_state")
        .bind(("match_id", &payload.match_id))
        .bind(("team_state", &payload.team_state))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(mut n) => {
            let team_info = n
                .take::<Vec<PlayerInfo>>(0)
                .map_err(|err| AppError::DatabaseError(err.to_string()));
            team_info
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn delete_match_info_by_id(
    payload: String,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let db = state.db.lock().await;

    let to_delete = db
        .delete::<Option<MatchInfo>>(("match_info", payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

    if let Some(match_info) = to_delete {
        db.delete::<Vec<PlayerInfo>>(match_info.match_id)
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_all_match_info(state: State<'_, Database>) -> Result<(), AppError> {
    let db = state.db.lock().await;

    let all_match_info = db
        .delete::<Vec<MatchInfo>>("match_info")
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))?;

    let mut all_match_info = all_match_info.iter();

    while let Some(m) = all_match_info.next() {
        db.delete::<Vec<PlayerInfo>>(m.match_id.clone())
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;
    }

    Ok(())
}
