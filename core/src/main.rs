use detection::predict_content_type;
use objc2_app_kit::{NSPasteboard, NSStringPboardType};
use ort::session::Session;
use tauri::{async_runtime::block_on, Manager};
use tauri_plugin_sql::DbPool;

mod db;
mod detection;
mod snippet;

const PASTEBOARD_POOL_MS: u64 = 500;

fn main() {
    // let (copy_tx, copy_rx) = mpsc::channel::<String>();
    // let (paste_tx, paste_rx) = mpsc::channel::<String>();

    // thread::spawn(move || {
    //     thread::scope(|s| {
    //         s.spawn(|| {
    //             let general = unsafe { NSPasteboard::generalPasteboard() };
    //             let mut cnt = unsafe { general.changeCount() };

    //             loop {
    //                 let new_cnt = unsafe { general.changeCount() };

    //                 if cnt != new_cnt {
    //                     cnt = new_cnt;

    //                     let content = unsafe { general.stringForType(NSStringPboardType) };

    //                     if let Some(content) = content {
    //                         copy_tx.send(content.to_string()).unwrap();
    //                     }
    //                 }
    //                 thread::sleep(Duration::from_millis(PASTEBOARD_POOL_MS));
    //             }
    //         });

    //         s.spawn(|| {
    //             println!("helloe");
    //         });
    //     });
    // });

    let session = Session::builder()
        .unwrap()
        .commit_from_memory(include_bytes!("model.onnx"))
        .unwrap();

    predict_content_type(
        &session,
        "&*handle.state::<tauri_plugin_sql::DbInstances>();",
    );
    let elapsed = std::time::Instant::now();
    for _ in 0..100 {
        predict_content_type(
            &session,
            "&*handle.state::<tauri_plugin_sql::DbInstances>();",
        );
    }

    println!("Elapsed time: {:?}", elapsed.elapsed());

    // tauri::Builder::default()
    //     .plugin(
    //         tauri_plugin_sql::Builder::default()
    //             .add_migrations(db::URL, db::list_migrations())
    //             .build(),
    //     )
    //     .setup(|app| {
    //         let handle = app.handle().clone();

    //         println!("1");
    //         tauri::async_runtime::spawn(async move {
    //             println!("2");
    //             let instances = &*handle.state::<tauri_plugin_sql::DbInstances>();
    //             let instances = instances.0.read().await;
    //             let DbPool::Sqlite(pool) = instances.get(db::URL).unwrap();

    //             let generator = Generator {
    //                 detector: &magika::Session::new().unwrap(),
    //                 syntax_set: &SyntaxSet::load_defaults_newlines(),
    //                 pool,
    //             };

    //             for content in copy_rx {
    //                 sqlx::query("INSERT INTO pb_item (content) VALUES (?)")
    //                     .bind(content)
    //                     .execute(pool)
    //                     .await
    //                     .unwrap();

    //                 let results = generator
    //                     .list_snippets(snippet::Filter {
    //                         cursor: None,
    //                         search: String::from("Vec"),
    //                     })
    //                     .await;

    //                 println!("{:?}", results);
    //             }
    //         });

    //         Ok(())
    //     })
    //     .run(tauri::generate_context!())
    //     .unwrap()
}
