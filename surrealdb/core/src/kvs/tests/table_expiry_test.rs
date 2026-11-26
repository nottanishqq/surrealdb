//! The `EXPIRE ON` record expiry job.

use std::sync::Arc;

use tokio_util::sync::CancellationToken;
use web_time::Duration;

use crate::dbs::{Capabilities, Session};
use crate::kvs::Datastore;
use crate::kvs::tasks::Progress;
use crate::types::PublicValue;

async fn ds() -> Arc<Datastore> {
	let ds = Datastore::builder()
		.with_capabilities(Capabilities::all())
		.without_maintenance_tasks()
		.build_with_path("memory")
		.await
		.unwrap();
	ds.execute(
		"DEFINE NAMESPACE test; USE NS test; DEFINE DATABASE test;",
		&Session::owner(),
		None,
	)
	.await
	.unwrap();
	ds
}

async fn run(ds: &Datastore, sql: &str) -> Vec<PublicValue> {
	let ses = Session::owner().with_ns("test").with_db("test");
	ds.execute(sql, &ses, None)
		.await
		.unwrap()
		.into_iter()
		.map(|r| r.result.unwrap_or_else(|e| panic!("`{sql}` failed: {e}")))
		.collect()
}

async fn ids(ds: &Datastore, table: &str) -> String {
	let out = run(ds, &format!("SELECT VALUE id FROM {table} ORDER BY id")).await;
	format!("{:?}", out.last().unwrap())
}

/// A pass deletes exactly the records whose expiry field holds a datetime that
/// has passed, runs the table's events for each, and leaves other tables alone.
#[tokio::test]
async fn a_pass_deletes_only_records_whose_expiry_has_passed() {
	let ds = ds().await;
	run(
		&ds,
		"DEFINE TABLE session EXPIRE ON expires_at;
		DEFINE EVENT gone ON session WHEN $event = 'DELETE' THEN (CREATE log SET of = $before.id);
		CREATE session:past SET expires_at = time::now() - 1h;
		CREATE session:edge SET expires_at = time::now() - 1ms;
		CREATE session:future SET expires_at = time::now() + 1h;
		CREATE session:never SET expires_at = NONE;
		CREATE session:unset;
		CREATE session:text SET expires_at = '2000-01-01';
		CREATE other:past SET expires_at = time::now() - 1h;",
	)
	.await;

	let pass =
		ds.table_expiry_process(&Duration::from_secs(10), &CancellationToken::new()).await.unwrap();
	assert_eq!(pass.items, 2);
	assert_eq!(pass.progress, Progress::Done);

	let left = ids(&ds, "session").await;
	for kept in ["future", "never", "unset", "text"] {
		assert!(left.contains(kept), "session:{kept} was deleted: {left}");
	}
	for gone in ["past", "edge"] {
		assert!(!left.contains(&format!("\"{gone}\"")), "session:{gone} survived: {left}");
	}
	assert!(ids(&ds, "other").await.contains("past"), "a table without EXPIRE ON was swept");
	let logged = run(&ds, "RETURN count(SELECT * FROM log)").await;
	assert_eq!(
		format!("{:?}", logged.last().unwrap()),
		"Number(Int(2))",
		"the DELETE events did not run"
	);

	// Extending a record's lifetime keeps it; a second pass finds nothing.
	run(&ds, "UPDATE session:future SET expires_at = time::now() + 2h").await;
	let pass =
		ds.table_expiry_process(&Duration::from_secs(10), &CancellationToken::new()).await.unwrap();
	assert_eq!(pass.items, 0);
}

/// A backlog larger than one batch is reported as more work.
#[tokio::test]
async fn a_backlog_beyond_one_batch_reports_more() {
	let ds = ds().await;
	run(
		&ds,
		"DEFINE TABLE cache EXPIRE ON at;
		FOR $i IN 0..1001 { CREATE cache SET at = time::now() - 1s };",
	)
	.await;
	let token = CancellationToken::new();
	let pass = ds.table_expiry_process(&Duration::from_secs(10), &token).await.unwrap();
	assert_eq!(pass.items, 1000);
	assert_eq!(pass.progress, Progress::More);
	let pass = ds.table_expiry_process(&Duration::from_secs(10), &token).await.unwrap();
	assert_eq!(pass.items, 1);
	assert_eq!(pass.progress, Progress::Done);
}
