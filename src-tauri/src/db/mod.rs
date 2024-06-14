use std::sync::Arc;
use surrealdb::{
    engine::local::{Db, Mem},
    Surreal,
};
use tauri::{async_runtime::Mutex, Manager};

pub mod event_register;
pub mod match_register;

pub struct Database {
    pub db: Arc<Mutex<Surreal<Db>>>,
}

impl Database {
    pub fn init(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
        let app_handle = app.app_handle();
        tauri::async_runtime::spawn(async move {
            let app_data_dir = app_handle
                .path_resolver()
                .app_data_dir()
                .unwrap_or_default();
            log::info!("App data dir: {}", app_data_dir.display());

            let db = Surreal::new::<Mem>(()).await?;

            db.use_ns("soccer_event_tagger")
                .use_db("match_data")
                .await?;

            app_handle.manage(Database {
                db: Arc::new(Mutex::new(db)),
            });

            Ok::<(), surrealdb::Error>(())
        });

        Ok(())
    }
}
