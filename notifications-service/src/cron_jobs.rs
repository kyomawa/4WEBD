use actix_rt::spawn;
use mongodb::Database;
use tokio_schedule::{Job, every};

use crate::service::check_notifications_and_try_to_send_mail;

// =============================================================================================================================

pub async fn cron_jobs(db: Database) {
    let every_15_seconds: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
        every(30).seconds().perform(move || {
            let local_db = db.clone();
            async move {
                println!("Running cron job: send email");
                if let Err(e) = check_notifications_and_try_to_send_mail(&local_db).await {
                    eprintln!("Erreur dans le cron job: {:?}", e);
                }
            }
        });
    spawn(every_15_seconds);
}

// =============================================================================================================================
