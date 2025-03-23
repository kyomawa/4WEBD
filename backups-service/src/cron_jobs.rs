use actix_rt::spawn;
use mongodb::Database;
use tokio_schedule::{Job, every};

// =============================================================================================================================

pub async fn cron_jobs(db: Database) {
    let every_10_seconds: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
        every(10).seconds().perform(move || {
            let local_db = db.clone();
            async move {
                println!("Running cron job: Backup Services.");
            }
        });
    // let every_10_seconds: std::pin::Pin<Box<dyn Future<Output = ()> + Send>> =
    //     every(10).seconds().perform(move || {
    //         let local_db = db.clone();
    //         async move {
    //             println!("Running cron job: Simulate payments");
    //             if let Err(e) = process_pending_payments(&local_db).await {
    //                 eprintln!("Erreur dans le cron job: {:?}", e);
    //             }
    //         }
    //     });
    spawn(every_10_seconds);
}

// =============================================================================================================================
