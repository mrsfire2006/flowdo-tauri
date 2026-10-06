use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use better_auth::{
    __private_core::types::{AuthRequest, AuthResponse, HttpMethod},
    plugins::{EmailPasswordPlugin, SessionManagementPlugin},
    seaorm::{Database, DatabaseConnection, SeaOrmStore},
    AuthConfig, AuthSchema, BetterAuth,
};
use chrono::{DateTime, Utc};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel,
    QueryFilter, Schema,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tauri::State;
use tokio::sync::Mutex;

mod entities {
    use better_auth::seaorm::sea_orm::entity::prelude::*;
    use better_auth::seaorm::AuthEntity;

    pub mod user {
        use super::*;

        #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
        #[auth(role = "user")]
        #[sea_orm(table_name = "users")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub name: Option<String>,
            pub email: Option<String>,
            pub email_verified: bool,
            pub image: Option<String>,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod session {
        use super::*;

        #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
        #[auth(role = "session")]
        #[sea_orm(table_name = "sessions")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub expires_at: DateTimeUtc,
            pub token: String,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
            pub ip_address: Option<String>,
            pub user_agent: Option<String>,
            pub user_id: String,
            pub active: bool,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod account {
        use super::*;

        #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
        #[auth(role = "account")]
        #[sea_orm(table_name = "accounts")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub account_id: String,
            pub provider_id: String,
            pub user_id: String,
            pub access_token: Option<String>,
            pub refresh_token: Option<String>,
            pub id_token: Option<String>,
            pub access_token_expires_at: Option<DateTimeUtc>,
            pub refresh_token_expires_at: Option<DateTimeUtc>,
            pub scope: Option<String>,
            pub password: Option<String>,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }

    pub mod verification {
        use super::*;

        #[derive(Clone, Debug, serde::Serialize, DeriveEntityModel, AuthEntity)]
        #[auth(role = "verification")]
        #[sea_orm(table_name = "verifications")]
        pub struct Model {
            #[sea_orm(primary_key, auto_increment = false)]
            pub id: String,
            pub identifier: String,
            pub value: String,
            pub expires_at: DateTimeUtc,
            pub created_at: DateTimeUtc,
            pub updated_at: DateTimeUtc,
        }

        #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
        pub enum Relation {}

        impl ActiveModelBehavior for ActiveModel {}
    }
}

mod task_entity {
    use better_auth::seaorm::sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel, serde::Serialize)]
    #[sea_orm(table_name = "task")]
    #[serde(rename_all = "camelCase")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false)]
        pub id: String,
        pub user_id: String,
        pub title: Option<String>,
        pub description: Option<String>,
        pub status: String,
        pub priority: String,
        pub estimated_minutes: Option<i32>,
        pub focus_started_at: Option<DateTimeUtc>,
        pub focus_elapsed_seconds: i32,
        pub completed_at: Option<DateTimeUtc>,
        pub created_at: DateTimeUtc,
        pub updated_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

struct AppAuthSchema;

impl AuthSchema for AppAuthSchema {
    type User = entities::user::Model;
    type Session = entities::session::Model;
    type Account = entities::account::Model;
    type Verification = entities::verification::Model;
}

pub struct AuthState {
    auth: Arc<BetterAuth<AppAuthSchema>>,
    database: DatabaseConnection,
    session_cookies: Mutex<Vec<String>>,
    session_file: PathBuf,
    remember_session: Mutex<bool>,
    request_lock: Mutex<()>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthClientResult<T> {
    is_success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_msg: Option<String>,
    status_code: u16,
}

impl<T> AuthClientResult<T> {
    fn success(value: T) -> Self {
        Self {
            is_success: true,
            value: Some(value),
            error_msg: None,
            status_code: 200,
        }
    }

    fn failure(error: AuthFailure) -> Self {
        Self {
            is_success: false,
            value: None,
            error_msg: Some(error.message),
            status_code: error.status_code,
        }
    }
}

#[derive(Debug)]
struct AuthFailure {
    message: String,
    status_code: u16,
}

impl AuthFailure {
    fn new(message: impl Into<String>, status_code: u16) -> Self {
        Self {
            message: message.into(),
            status_code,
        }
    }
}

impl From<String> for AuthFailure {
    fn from(message: String) -> Self {
        Self::new(message, 500)
    }
}

impl AuthState {
    pub async fn initialize(data_dir: &Path) -> Result<Self, String> {
        fs::create_dir_all(data_dir).map_err(|error| error.to_string())?;
        let secret = load_or_create_secret(data_dir)?;
        let session_file = data_dir.join("auth-session");
        let session_cookies = match fs::read_to_string(&session_file) {
            Ok(cookies) => cookies.lines().map(str::to_string).collect::<Vec<_>>(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(format!("Could not read the saved auth session: {error}")),
        };
        let db_path = data_dir.join("flowdodb.sqlite");
        let database_url = format!("sqlite://{}?mode=rwc", db_path.display());
        let database = Database::connect(&database_url)
            .await
            .map_err(|error| format!("Could not open the auth database: {error}"))?;

        migrate_auth_tables(&database).await?;

        let config = AuthConfig::new(secret)
            .app_name("Flowdo")
            .base_url("http://localhost")
            .trusted_origins(vec![
                "tauri://localhost".to_string(),
                "http://tauri.localhost".to_string(),
                "https://tauri.localhost".to_string(),
            ])
            .password_min_length(3);
        let store = SeaOrmStore::<AppAuthSchema>::new(config.clone(), database.clone());
        let auth = BetterAuth::<AppAuthSchema>::new(config)
            .store(store)
            .plugin(
                EmailPasswordPlugin::new()
                    .enable_signup(true)
                    .password_min_length(3),
            )
            .plugin(SessionManagementPlugin::new())
            .build()
            .await
            .map_err(|error| format!("Could not initialize Better Auth: {error}"))?;

        Ok(Self {
            auth: Arc::new(auth),
            database,
            session_cookies: Mutex::new(session_cookies.clone()),
            session_file,
            remember_session: Mutex::new(!session_cookies.is_empty()),
            request_lock: Mutex::new(()),
        })
    }

    async fn request(
        &self,
        method: HttpMethod,
        path: &str,
        body: Option<Value>,
        with_session: bool,
    ) -> Result<Value, AuthFailure> {
        let _request_guard = self.request_lock.lock().await;
        let mut headers = HashMap::from([
            ("content-type".to_string(), "application/json".to_string()),
            ("origin".to_string(), "tauri://localhost".to_string()),
            ("user-agent".to_string(), "Flowdo Desktop".to_string()),
        ]);

        if with_session {
            let cookies = self.session_cookies.lock().await;
            if !cookies.is_empty() {
                headers.insert("cookie".to_string(), cookies.join("; "));
            }
        }

        let request_body = body
            .map(|value| {
                serde_json::to_vec(&value).map_err(|error| AuthFailure::new(error.to_string(), 500))
            })
            .transpose()?;
        let request = AuthRequest::from_parts(
            method,
            path.to_string(),
            headers,
            request_body,
            HashMap::new(),
        );
        let response = self.auth.handle_request(request).await.map_err(|error| {
            AuthFailure::new(format!("Authentication request failed: {error}"), 500)
        })?;

        if (200..300).contains(&response.status) {
            let updated_cookies = cookies_from_response(&response);
            if !updated_cookies.is_empty() {
                *self.session_cookies.lock().await = updated_cookies.clone();
                if *self.remember_session.lock().await {
                    write_private_file(&self.session_file, &updated_cookies.join("\n"))
                        .map_err(AuthFailure::from)?;
                } else {
                    remove_if_exists(&self.session_file).map_err(AuthFailure::from)?;
                }
            }
        }

        decode_response(response)
    }
}

fn write_private_file(path: &Path, content: &str) -> Result<(), String> {
    fs::write(path, content).map_err(|error| error.to_string())?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o600))
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn remove_if_exists(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error.to_string()),
    }
}

fn load_or_create_secret(data_dir: &Path) -> Result<String, String> {
    let secret_path = data_dir.join("auth-secret");
    match fs::read_to_string(&secret_path) {
        Ok(secret) if secret.trim().len() >= 32 => Ok(secret.trim().to_string()),
        Ok(_) => Err(
            "The stored Better Auth secret is invalid; remove auth-secret to regenerate it".into(),
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            let secret = format!(
                "{}{}",
                uuid::Uuid::new_v4().simple(),
                uuid::Uuid::new_v4().simple()
            );
            write_private_file(&secret_path, &secret)?;
            Ok(secret)
        }
        Err(error) => Err(format!("Could not read the Better Auth secret: {error}")),
    }
}

async fn migrate_auth_tables(database: &DatabaseConnection) -> Result<(), String> {
    let schema = Schema::new(database.get_database_backend());
    let statements = [
        schema
            .create_table_from_entity(entities::user::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(entities::session::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(entities::account::Entity)
            .if_not_exists()
            .to_owned(),
        schema
            .create_table_from_entity(entities::verification::Entity)
            .if_not_exists()
            .to_owned(),
    ];

    for statement in statements {
        database
            .execute(&statement)
            .await
            .map_err(|error| format!("Could not migrate the auth database: {error}"))?;
    }

    let task_table = r#"
        CREATE TABLE IF NOT EXISTS task (
            id TEXT PRIMARY KEY NOT NULL,
            user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            title TEXT,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'INBOX' CHECK (status IN ('INBOX', 'IN_PROGRESS', 'DONE')),
            priority TEXT NOT NULL DEFAULT 'MEDIUM' CHECK (priority IN ('LOW', 'MEDIUM', 'HIGH')),
            estimated_minutes INTEGER,
            focus_started_at TEXT,
            focus_elapsed_seconds INTEGER NOT NULL DEFAULT 0,
            completed_at TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
    "#;
    database
        .execute_unprepared(task_table)
        .await
        .map_err(|error| format!("Could not create the task table: {error}"))?;
    database
        .execute_unprepared(
            "CREATE INDEX IF NOT EXISTS idx_task_user_updated ON task(user_id, updated_at)",
        )
        .await
        .map_err(|error| format!("Could not index the task table: {error}"))?;
    Ok(())
}

fn cookies_from_response(response: &AuthResponse) -> Vec<String> {
    response
        .headers
        .get_all("set-cookie")
        .filter_map(|header| header.split(';').next())
        .filter(|cookie| cookie.contains('='))
        .map(str::to_string)
        .collect()
}

fn decode_response(response: AuthResponse) -> Result<Value, AuthFailure> {
    let value = serde_json::from_slice::<Value>(&response.body)
        .unwrap_or_else(|_| Value::String(String::from_utf8_lossy(&response.body).into_owned()));
    if (200..300).contains(&response.status) {
        Ok(value)
    } else {
        let message = value
            .get("message")
            .and_then(Value::as_str)
            .or_else(|| value.get("error").and_then(Value::as_str))
            .unwrap_or("Authentication request was rejected");
        Err(AuthFailure::new(message, response.status))
    }
}

fn command_result<T>(result: Result<T, AuthFailure>) -> AuthClientResult<T> {
    match result {
        Ok(value) => AuthClientResult::success(value),
        Err(error) => AuthClientResult::failure(error),
    }
}

#[tauri::command]
pub async fn auth_sign_up(
    state: State<'_, AuthState>,
    name: String,
    email: String,
    password: String,
) -> Result<AuthClientResult<Value>, String> {
    *state.remember_session.lock().await = true;
    Ok(command_result(
        state
            .request(
                HttpMethod::Post,
                "/sign-up/email",
                Some(json!({ "name": name, "email": email, "password": password })),
                false,
            )
            .await,
    ))
}

#[tauri::command]
pub async fn auth_sign_in(
    state: State<'_, AuthState>,
    email: String,
    password: String,
    remember_me: Option<bool>,
) -> Result<AuthClientResult<Value>, String> {
    let remember_me = remember_me.unwrap_or(true);
    *state.remember_session.lock().await = remember_me;
    if !remember_me {
        if let Err(error) = remove_if_exists(&state.session_file) {
            return Ok(AuthClientResult::failure(AuthFailure::new(error, 500)));
        }
    }
    Ok(command_result(
        state
            .request(
                HttpMethod::Post,
                "/sign-in/email",
                Some(json!({ "email": email, "password": password, "rememberMe": remember_me })),
                false,
            )
            .await,
    ))
}

#[tauri::command]
pub async fn auth_get_session(
    state: State<'_, AuthState>,
) -> Result<AuthClientResult<Value>, String> {
    if state.session_cookies.lock().await.is_empty() {
        return Ok(AuthClientResult::success(Value::Null));
    }
    match state
        .request(HttpMethod::Get, "/get-session", None, true)
        .await
    {
        Ok(session) if session.is_null() => {
            state.session_cookies.lock().await.clear();
            match remove_if_exists(&state.session_file) {
                Ok(()) => Ok(AuthClientResult::success(Value::Null)),
                Err(error) => Ok(AuthClientResult::failure(AuthFailure::new(error, 500))),
            }
        }
        Ok(session) => Ok(AuthClientResult::success(session)),
        Err(error) if error.status_code < 500 => {
            state.session_cookies.lock().await.clear();
            match remove_if_exists(&state.session_file) {
                Ok(()) => Ok(AuthClientResult::success(Value::Null)),
                Err(error) => Ok(AuthClientResult::failure(AuthFailure::new(error, 500))),
            }
        }
        Err(error) => Ok(AuthClientResult::failure(error)),
    }
}

#[tauri::command]
pub async fn auth_sign_out(state: State<'_, AuthState>) -> Result<AuthClientResult<Value>, String> {
    let result = if state.session_cookies.lock().await.is_empty() {
        Ok(json!({ "success": true }))
    } else {
        state
            .request(HttpMethod::Post, "/sign-out", Some(json!({})), true)
            .await
    };
    match result {
        Ok(value) => {
            state.session_cookies.lock().await.clear();
            *state.remember_session.lock().await = false;
            match remove_if_exists(&state.session_file) {
                Ok(()) => Ok(AuthClientResult::success(value)),
                Err(error) => Ok(AuthClientResult::failure(AuthFailure::new(error, 500))),
            }
        }
        Err(error) => Ok(AuthClientResult::failure(error)),
    }
}

async fn authenticated_user_id(state: &AuthState) -> Result<String, AuthFailure> {
    if state.session_cookies.lock().await.is_empty() {
        return Err(AuthFailure::new("You are not signed in.", 401));
    }
    let session = state
        .request(HttpMethod::Get, "/get-session", None, true)
        .await?;
    session
        .get("user")
        .and_then(|user| user.get("id"))
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| AuthFailure::new("Your session has expired. Please sign in again.", 401))
}

fn task_view(task: task_entity::Model) -> Value {
    json!({
        "id": task.id,
        "title": task.title,
        "description": task.description,
        "status": task.status,
        "priority": task.priority,
        "durationMinutes": task.estimated_minutes,
        "focusStartedAt": task.focus_started_at,
        "focusElapsedSeconds": task.focus_elapsed_seconds,
        "completedAt": task.completed_at,
        "createdAt": task.created_at,
        "updatedAt": task.updated_at,
    })
}

fn parse_task_datetime(value: &Value, field: &str) -> Result<Option<DateTime<Utc>>, AuthFailure> {
    if value.is_null() {
        return Ok(None);
    }
    let value = value.as_str().ok_or_else(|| {
        AuthFailure::new(format!("{field} must be an ISO timestamp or null."), 400)
    })?;
    DateTime::parse_from_rfc3339(value)
        .map(|date| Some(date.with_timezone(&Utc)))
        .map_err(|_| AuthFailure::new(format!("{field} must be a valid ISO timestamp."), 400))
}

fn optional_string_patch(
    patch: &Value,
    field: &str,
) -> Result<Option<Option<String>>, AuthFailure> {
    match patch.get(field) {
        None => Ok(None),
        Some(Value::Null) => Ok(Some(None)),
        Some(Value::String(value)) => Ok(Some(Some(value.clone()))),
        Some(_) => Err(AuthFailure::new(
            format!("{field} must be a string or null."),
            400,
        )),
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskInput {
    title: Option<String>,
    description: Option<String>,
    estimated_minutes: Option<i32>,
    priority: Option<String>,
}

#[tauri::command]
pub async fn task_list(state: State<'_, AuthState>) -> Result<AuthClientResult<Value>, String> {
    let user_id = match authenticated_user_id(&state).await {
        Ok(user_id) => user_id,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    let tasks = task_entity::Entity::find()
        .filter(task_entity::Column::UserId.eq(user_id))
        .all(&state.database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500));
    let tasks = match tasks {
        Ok(tasks) => tasks,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    let views = tasks.into_iter().map(task_view).collect::<Vec<_>>();
    Ok(AuthClientResult::success(Value::Array(views)))
}

#[tauri::command]
pub async fn task_create(
    state: State<'_, AuthState>,
    task: CreateTaskInput,
) -> Result<AuthClientResult<Value>, String> {
    let user_id = match authenticated_user_id(&state).await {
        Ok(user_id) => user_id,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    if task.estimated_minutes.is_some_and(|minutes| minutes < 0) {
        return Ok(AuthClientResult::failure(AuthFailure::new(
            "estimatedMinutes cannot be negative.",
            400,
        )));
    }
    let priority = task.priority.unwrap_or_else(|| "MEDIUM".to_string());
    if !["LOW", "MEDIUM", "HIGH"].contains(&priority.as_str()) {
        return Ok(AuthClientResult::failure(AuthFailure::new(
            "priority must be LOW, MEDIUM, or HIGH.",
            400,
        )));
    }
    let now = Utc::now();
    let model = task_entity::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        user_id: Set(user_id),
        title: Set(task.title),
        description: Set(task.description),
        status: Set("INBOX".to_string()),
        priority: Set(priority),
        estimated_minutes: Set(task.estimated_minutes),
        focus_started_at: Set(None),
        focus_elapsed_seconds: Set(0),
        completed_at: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };
    let created = model
        .insert(&state.database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500));
    let created = match created {
        Ok(created) => created,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    Ok(AuthClientResult::success(task_view(created)))
}

async fn update_task_record(
    database: &DatabaseConnection,
    user_id: &str,
    id: &str,
    patch: Value,
) -> Result<Value, AuthFailure> {
    let model = task_entity::Entity::find_by_id(id)
        .filter(task_entity::Column::UserId.eq(user_id))
        .one(database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500))?
        .ok_or_else(|| AuthFailure::new("Task not found.", 404))?;
    let mut active = model.into_active_model();

    if let Some(value) = optional_string_patch(&patch, "title")? {
        active.title = Set(value);
    }
    if let Some(value) = optional_string_patch(&patch, "description")? {
        active.description = Set(value);
    }
    if let Some(status) = patch.get("status") {
        let status = status
            .as_str()
            .ok_or_else(|| AuthFailure::new("status must be a string.", 400))?;
        if !["INBOX", "IN_PROGRESS", "DONE"].contains(&status) {
            return Err(AuthFailure::new(
                "status must be INBOX, IN_PROGRESS, or DONE.",
                400,
            ));
        }
        active.status = Set(status.to_string());
        active.completed_at = Set((status == "DONE").then(Utc::now));
    }
    if let Some(priority) = patch.get("priority") {
        let priority = priority
            .as_str()
            .ok_or_else(|| AuthFailure::new("priority must be a string.", 400))?;
        if !["LOW", "MEDIUM", "HIGH"].contains(&priority) {
            return Err(AuthFailure::new(
                "priority must be LOW, MEDIUM, or HIGH.",
                400,
            ));
        }
        active.priority = Set(priority.to_string());
    }
    if let Some(minutes) = patch.get("estimatedMinutes") {
        let minutes = if minutes.is_null() {
            None
        } else {
            Some(
                minutes
                    .as_i64()
                    .filter(|value| (0..=i32::MAX as i64).contains(value))
                    .ok_or_else(|| {
                        AuthFailure::new(
                            "estimatedMinutes must be a non-negative integer or null.",
                            400,
                        )
                    })? as i32,
            )
        };
        active.estimated_minutes = Set(minutes);
    }
    if let Some(elapsed) = patch.get("focusElapsedSeconds") {
        let elapsed = elapsed
            .as_i64()
            .filter(|value| (0..=i32::MAX as i64).contains(value))
            .ok_or_else(|| {
                AuthFailure::new("focusElapsedSeconds must be a non-negative integer.", 400)
            })?;
        active.focus_elapsed_seconds = Set(elapsed as i32);
    }
    if let Some(started_at) = patch.get("focusStartedAt") {
        active.focus_started_at = Set(parse_task_datetime(started_at, "focusStartedAt")?);
    }
    active.updated_at = Set(Utc::now());

    let updated = active
        .update(database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500))?;
    Ok(task_view(updated))
}

#[tauri::command]
pub async fn task_update(
    state: State<'_, AuthState>,
    id: String,
    task: Value,
) -> Result<AuthClientResult<Value>, String> {
    let user_id = match authenticated_user_id(&state).await {
        Ok(user_id) => user_id,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    Ok(command_result(
        update_task_record(&state.database, &user_id, &id, task).await,
    ))
}

#[tauri::command]
pub async fn task_delete(
    state: State<'_, AuthState>,
    id: String,
) -> Result<AuthClientResult<Value>, String> {
    let user_id = match authenticated_user_id(&state).await {
        Ok(user_id) => user_id,
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    let model = task_entity::Entity::find_by_id(&id)
        .filter(task_entity::Column::UserId.eq(user_id))
        .one(&state.database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500));
    let model = match model {
        Ok(Some(model)) => model,
        Ok(None) => {
            return Ok(AuthClientResult::failure(AuthFailure::new(
                "Task not found.",
                404,
            )))
        }
        Err(error) => return Ok(AuthClientResult::failure(error)),
    };
    let deleted = task_entity::Entity::delete_by_id(model.id)
        .exec(&state.database)
        .await
        .map_err(|error| AuthFailure::new(error.to_string(), 500));
    if let Err(error) = deleted {
        return Ok(AuthClientResult::failure(error));
    }
    Ok(AuthClientResult::success(json!({ "success": true })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn email_password_registration_and_logout_flow() {
        let data_dir = std::env::temp_dir().join(format!("flowdo-auth-{}", uuid::Uuid::new_v4()));
        let auth = AuthState::initialize(&data_dir)
            .await
            .expect("test auth state should initialize");
        assert!(data_dir.join("flowdodb.sqlite").exists());
        *auth.remember_session.lock().await = true;

        let too_short_password = auth
            .request(
                HttpMethod::Post,
                "/sign-up/email",
                Some(json!({
                    "name": "Short Password",
                    "email": "flowdo-short-password@example.com",
                    "password": "ab"
                })),
                false,
            )
            .await;
        assert!(too_short_password.is_err());

        let registration = auth
            .request(
                HttpMethod::Post,
                "/sign-up/email",
                Some(json!({
                    "name": "Flowdo Test",
                    "email": "flowdo-auth-test@example.com",
                    "password": "abc"
                })),
                false,
            )
            .await
            .expect("registration should succeed");
        assert!(registration.get("user").is_some());
        let user_id = registration["user"]["id"]
            .as_str()
            .expect("registered user should have an ID")
            .to_string();

        let now = Utc::now();
        let task = task_entity::ActiveModel {
            id: Set("test-task-1".to_string()),
            user_id: Set(user_id.clone()),
            title: Set(Some("Integration task".to_string())),
            description: Set(None),
            status: Set("INBOX".to_string()),
            priority: Set("MEDIUM".to_string()),
            estimated_minutes: Set(Some(25)),
            focus_started_at: Set(None),
            focus_elapsed_seconds: Set(0),
            completed_at: Set(None),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(&auth.database)
        .await
        .expect("task should be created in SQLite");

        let updated_task = update_task_record(
            &auth.database,
            &user_id,
            &task.id,
            json!({
                "status": "IN_PROGRESS",
                "focusStartedAt": Utc::now().to_rfc3339(),
                "focusElapsedSeconds": 12
            }),
        )
        .await
        .expect("owned task should be updated");
        assert_eq!(updated_task["status"], "IN_PROGRESS");
        assert_eq!(updated_task["focusElapsedSeconds"], 12);

        let listed_tasks = task_entity::Entity::find()
            .filter(task_entity::Column::UserId.eq(&user_id))
            .all(&auth.database)
            .await
            .expect("tasks should be listed for their owner");
        assert_eq!(listed_tasks.len(), 1);

        let removed = task_entity::Entity::delete_by_id(task.id)
            .exec(&auth.database)
            .await
            .expect("task should be deletable");
        assert_eq!(removed.rows_affected, 1);

        let auth = AuthState::initialize(&data_dir)
            .await
            .expect("saved auth session should initialize");
        let session = auth
            .request(HttpMethod::Get, "/get-session", None, true)
            .await
            .expect("session lookup should succeed");
        assert!(session.get("user").is_some());

        auth.request(HttpMethod::Post, "/sign-out", Some(json!({})), true)
            .await
            .expect("logout should succeed");
        auth.session_cookies.lock().await.clear();
        remove_if_exists(&auth.session_file).expect("saved session should be removed");

        let login = auth
            .request(
                HttpMethod::Post,
                "/sign-in/email",
                Some(json!({
                    "email": "flowdo-auth-test@example.com",
                    "password": "abc",
                    "rememberMe": true
                })),
                false,
            )
            .await
            .expect("login should succeed");
        assert!(login.get("user").is_some());

        auth.request(HttpMethod::Post, "/sign-out", Some(json!({})), true)
            .await
            .expect("logout after login should succeed");
        auth.session_cookies.lock().await.clear();

        let session = auth
            .request(HttpMethod::Get, "/get-session", None, true)
            .await
            .expect("session lookup after logout should succeed");
        assert!(session.is_null());

        fs::remove_dir_all(data_dir).expect("test auth data should be removed");
    }

    #[test]
    fn client_result_uses_the_shared_frontend_contract() {
        let payload = serde_json::to_value(AuthClientResult::success(json!({ "id": "user-1" })))
            .expect("result should serialize");
        assert_eq!(payload["isSuccess"], true);
        assert_eq!(payload["value"]["id"], "user-1");
        assert_eq!(payload["statusCode"], 200);
        assert!(payload.get("errorMsg").is_none());
    }
}
