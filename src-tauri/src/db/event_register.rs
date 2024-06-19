use super::Database;
use csv::Writer;
use tauri::State;
use types::{AppError, TaggedEvent};

#[tauri::command]
pub async fn insert_data(
    mut payload: TaggedEvent,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    payload.assign_uuid();
    let db = state.db.lock().await;
    match db
        .create::<Option<TaggedEvent>>(("tagged_events", &payload.uuid))
        .content(payload)
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(inserted) => {
            if let Some(data) = inserted {
                log::info!(
                    "[INS]: event_id: {}, team_name: {}, player_name: {}",
                    data.event_id,
                    data.team_name,
                    data.player_name
                );
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
    payload: String,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let maybe_file = rfd::AsyncFileDialog::new().save_file().await;

    if let Some(file) = maybe_file {
        let path = file.path();
        let db = state.db.lock().await;
        let data = db
            .query("SELECT * FROM tagged_events WHERE match_id = $match_id")
            .bind(("match_id", payload))
            .await
            .map_err(|err| AppError::DatabaseError(err.to_string()))?
            .take::<Vec<TaggedEvent>>(0)
            .map_err(|err| AppError::DatabaseError(err.to_string()))?;

        let mut writer =
            Writer::from_path(path).map_err(|err| AppError::CsvWriteError(err.to_string()))?;

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
pub async fn get_match_events_from_match_id(
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
pub async fn delete_by_id(payload: i32, state: State<'_, Database>) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Option<TaggedEvent>>(("tagged_events", payload))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(deleted) => {
            if let Some(data) = deleted {
                log::info!("[DEL]: {:?}", data.event_id);
            }
            Ok(())
        }
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
        Ok(_) => {
            log::info!("[CLR]: {}", payload);
            Ok(())
        }
        Err(err) => {
            log::error!("[CLR]: {}", err);
            Err(err)
        }
    }
}
