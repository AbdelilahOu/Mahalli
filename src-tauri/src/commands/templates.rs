use tauri::State;

use service::{MutationsService, NewTemplate};

use crate::AppState;

use super::{Fail, Seccess, SResult};

#[tauri::command]
pub async fn create_template(state: State<'_, AppState>, template: NewTemplate) -> SResult<String> {
	let _ = state.db_conn;
	match MutationsService::create_template(&state.db_conn, template).await {
		Ok(id) => Ok(Seccess::<String> {
			error: None,
			message: Option::Some(String::from("template created successfully")),
			data: Some(id),
		}),
		Err(err) => {
			Err(Fail {
				error: Some(err.to_string()),
				message: None,
			})
		}
	}
}