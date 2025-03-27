use actix_rt::spawn;
use mongodb::Database;
use tokio_schedule::{Job, every};

use crate::service::{process_pending_backups, trigger_backups_for_all_services};

// =============================================================================================================================

pub async fn cron_jobs(db: Database) {
    let db_for_backup = db.clone();
    let db_for_trigger = db.clone();

    let every_90_seconds: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
        every(20).seconds().perform(move || {
            let local_db = db_for_backup.clone();
            async move {
                println!("Running cron job: Backup all Services");
                if let Err(e) = process_pending_backups(&local_db).await {
                    eprintln!("Erreur dans le cron job: {:?}", e);
                }
            }
        });
    spawn(every_90_seconds);

    let every_10_minutes: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
        every(10).seconds().perform(move || {
            let local_db = db_for_trigger.clone();
            async move {
                println!("Running cron job: Trigger Backup for all Services");
                if let Err(e) = trigger_backups_for_all_services(&local_db).await {
                    eprintln!("Erreur dans le cron job: {:?}", e);
                }
            }
        });
    spawn(every_10_minutes);
}

// =============================================================================================================================
