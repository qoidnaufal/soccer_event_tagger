use super::Database;
use surrealdb::opt::PatchOp;
use tauri::State;
use types::{AppError, MatchInfo, PlayerInfo, TaggedEvent};

#[tauri::command]
pub async fn insert_data(
    mut payload: TaggedEvent,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    payload.assign_uuid();

    let db = state.db.lock().await;

    match db
        .create::<Option<TaggedEvent>>(("tagged_events", &payload.uuid))
        .content(payload.clone())
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(created) => {
            if let Some(tagged_event) = created {
                if tagged_event.event_name == "Subs Out" {
                    db.update::<Option<PlayerInfo>>((
                        &tagged_event.match_id,
                        &tagged_event.player_id,
                    ))
                    .patch(PatchOp::replace("/play", false))
                    .await
                    .map_err(|err| AppError::DatabaseError(err.to_string()))?;

                    let player_in = db.query("SELECT * FROM type::table($match_id) WHERE team_name = $team_name AND player_name = $player_name")
                        .bind(("match_id", &tagged_event.match_id))
                        .bind(("team_name", &tagged_event.team_end))
                        .bind(("player_name", &tagged_event.player_end))
                        .await
                        .map_err(|err| AppError::DatabaseError(err.to_string()))?
                        .take::<Option<PlayerInfo>>(0)
                        .map_err(|err| AppError::DatabaseError(err.to_string()))?
                        .unwrap_or_default();

                    db.update::<Option<PlayerInfo>>((&tagged_event.match_id, &player_in.player_id))
                        .patch(PatchOp::replace("/play", true))
                        .patch(PatchOp::add(
                            "/play_position",
                            payload.play_position.as_ref().unwrap(),
                        ))
                        .await
                        .map_err(|err| AppError::DatabaseError(err.to_string()))?;
                }

                if tagged_event.event_name == "Change Position" {
                    db.update::<Option<PlayerInfo>>((
                        &tagged_event.match_id,
                        &tagged_event.player_id,
                    ))
                    .patch(PatchOp::add(
                        "/play_position",
                        payload.play_position.as_ref().unwrap(),
                    ))
                    .await
                    .map_err(|err| AppError::DatabaseError(err.to_string()))?;
                }
            }

            Ok(())
        }
        Err(err) => {
            log::error!("[INS]: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn get_all_data(state: State<'_, Database>) -> Result<Vec<TaggedEvent>, AppError> {
    let db = state.db.lock().await;

    db.select::<Vec<TaggedEvent>>("tagged_events")
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
}

#[tauri::command]
pub async fn export_tagged_events_from_match_id(
    payload: MatchInfo,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let date = payload.match_date.split('T').next().unwrap_or_default();
    let file_name = format!(
        "[{}] {} vs {}.csv",
        date, payload.team_home, payload.team_away
    );
    let maybe_file = rfd::AsyncFileDialog::new()
        .set_file_name(file_name)
        .save_file()
        .await;

    if let Some(file) = maybe_file {
        let path = file.path();
        log::info!("path: {:?}", path);

        let db = state.db.lock().await;
        let data = db
            .query("SELECT * FROM tagged_events WHERE match_id = $match_id")
            .bind(("match_id", payload.match_id))
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?
            .take::<Vec<TaggedEvent>>(0)
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        let mut writer = csv::Writer::from_path(path).map_err(|err| {
            log::error!("csv error: {}", err);
            AppError::CsvWriteError(err.to_string())
        })?;

        for i in data.iter() {
            writer
                .serialize(i)
                .map_err(|err| AppError::CsvWriteError(err.to_string()))?;
        }
        writer
            .flush()
            .map_err(|err| AppError::CsvWriteError(err.to_string()))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn export_all_tagged_events(state: State<'_, Database>) -> Result<(), AppError> {
    let maybe_file = rfd::AsyncFileDialog::new()
        .set_file_name("ALL_DATA.csv")
        .save_file()
        .await;

    if let Some(file) = maybe_file {
        let path = file.path();
        log::info!("path: {:?}", path);

        let db = state.db.lock().await;

        let data = db
            .select::<Vec<TaggedEvent>>("tagged_events")
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        let mut writer = csv::Writer::from_path(path).map_err(|err| {
            log::error!("csv error: {}", err);
            AppError::CsvWriteError(err.to_string())
        })?;

        for d in data.iter() {
            writer
                .serialize(d)
                .map_err(|err| AppError::CsvWriteError(err.to_string()))?;
        }
        writer
            .flush()
            .map_err(|err| AppError::CsvWriteError(err.to_string()))?;
    }

    Ok(())
}

#[tauri::command]
pub async fn get_events_by_match_id(
    payload: String,
    state: State<'_, Database>,
) -> Result<Vec<TaggedEvent>, AppError> {
    let db = state.db.lock().await;

    match db
        .query("SELECT * FROM tagged_events WHERE match_id = $match_id")
        .bind(("match_id", &payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(mut res) => {
            let tagged_event = res
                .take(0)
                .map_err(|err| AppError::DatabaseError(err.to_string()));
            tagged_event
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
pub async fn delete_by_id(payload: String, state: State<'_, Database>) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Option<TaggedEvent>>(("tagged_events", payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("[DEL]: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn delete_all_records_by_match_id(
    payload: String,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .query("DELETE tagged_events WHERE match_id = $match_id")
        .bind(("match_id", &payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(_) => Ok(()),
        Err(err) => {
            log::error!("[CLR]: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn delete_all_data(state: State<'_, Database>) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Vec<TaggedEvent>>("tagged_events")
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
