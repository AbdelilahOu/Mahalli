use serde_json::Value;
use tauri::State;

use service::{ListArgs, MutationsService, NewInventory, QueriesService};

use crate::{AppState, commands::Fail};

use super::{Seccess, SResult};

#[tauri::command]
pub async fn list_inventory(state: State<'_, AppState>, args: ListArgs) -> SResult<Value> {
    let _ = state.db_conn;
    match QueriesService::list_inventory(&state.db_conn, args).await {
        Ok(res) => Ok(Seccess {
            error: None,
            message: None,
            data: Some(res),
        }),
        Err(err) => {
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

#[tauri::command]
pub async fn create_inventory(state: State<'_, AppState>, mvm: NewInventory) -> SResult<String> {
    let _ = state.db_conn;
    match MutationsService::create_inventory(&state.db_conn, mvm).await {
        Ok(id) => Ok(Seccess::<String> {
            error: None,
            message: Option::Some(String::from("inventory created successfully")),
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

#[tauri::command]
pub async fn delete_inventory(state: State<'_, AppState>, id: String) -> SResult<String> {
    let _ = state.db_conn;
    match MutationsService::delete_inventory(&state.db_conn, id).await {
        Ok(_) => Ok(Seccess::<String> {
            error: None,
            message: Option::Some(String::from("inventory deleted successfully")),
            data: None,
        }),
        Err(err) => {
            Err(Fail {
                error: Some(err.to_string()),
                message: None,
            })
        }
    }
}

