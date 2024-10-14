use sd_utils::db::maybe_missing;

use super::{DeleteBehavior, FileData};

#[derive(Debug, Hash)]
pub struct RemoveBehavior;

impl DeleteBehavior for RemoveBehavior {
	async fn delete(file: FileData) -> Result<(), ()> {
		tracing::debug!(?file.full_path, "REMOVE ---");
		tokio::time::sleep(std::time::Duration::from_secs(2)).await; //  TODO(matheus-consoli):
		if maybe_missing(file.file_path.is_dir, "file_path.is_dir").unwrap() {
			tokio::fs::remove_dir_all(&file.full_path).await
		} else {
			tokio::fs::remove_file(&file.full_path).await
		};
		Ok(())
	}
}
