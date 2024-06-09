use super::Database;
use tauri::{Runtime, State, Window};
use types::{AppError, TaggedEvent};

#[tauri::command]
pub async fn insert_data<R: Runtime>(
    mut payload: TaggedEvent,
    state: State<'_, Database>,
    _window: Window<R>,
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
            log::info!("Inserted: {:?}", inserted);
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
        Ok(_) => {
            log::info!("Deleted record: {:?}", payload);
            Ok(())
        }
        Err(err) => {
            log::error!("Failed to delete record: {}", err);
            Err(err)
        }
    }
}