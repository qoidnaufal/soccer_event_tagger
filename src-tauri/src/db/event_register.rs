use super::Database;
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
                log::info!("Inserted: {:?}", data.uuid);
            }
            Ok(())
        }
        Err(err) => {
            log::error!("Insert error: {}", err);
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
pub async fn delete_by_id(
    payload: TaggedEvent,
    state: State<'_, Database>,
) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Option<TaggedEvent>>(("tagged_events", &payload.uuid))
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(deleted) => {
            if let Some(data) = deleted {
                log::info!("Deleted record: {:?}", data.uuid);
            }
            Ok(())
        }
        Err(err) => {
            log::error!("Failed to delete record: {}", err);
            Err(err)
        }
    }
}

#[tauri::command]
pub async fn delete_all_records(state: State<'_, Database>) -> Result<(), AppError> {
    let db = state.db.lock().await;

    match db
        .delete::<Vec<TaggedEvent>>("tagged_events")
        .await
        .map_err(|err| AppError::DatabaseError(err.to_string()))
    {
        Ok(deleted) => {
            log::info!("Cleaned {} database records", deleted.len());
            Ok(())
        }
        Err(err) => {
            log::error!("Failed to clear the database: {}", err);
            Err(err)
        }
    }
}
