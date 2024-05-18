use crate::libs::prelude::*;
use crate::libs::prelude_router::*;
use crate::libs::prelude_entity::*;
use crate::entity::users::{Users, self};

pub fn routes() -> Router {
    Router::new()
        .nest("/v1", v1())
}

fn v1() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/login/cookie", post(login_cookie))
        .route("/session", get(session))
        .route("/session/sales", get(sales))
}

#[derive(Debug, Serialize, Deserialize)]
struct RegisterParam {
    name: String,
    phone: String,
    password: String,
}

async fn register(
    Extension(db): Extension<Arc<PgPool>>,
    Json(data): Json<RegisterParam>
) -> Result<Json<Users>> {
    let data = users::Data {
        name: data.name,
        phone: data.phone,
        password: crate::libs::password::hash(data.password.as_bytes())?,
        role: users::Role::Customer,
        ..Default::default()
    };
    let user = users::create(&*db, &data).await?;
    Ok(Json(user))
}

#[derive(Debug, Serialize)]
struct TokenResponse {
    token: String
}

#[derive(Debug, Clone, Deserialize)]
struct LoginParam {
    phone: String,
    password: String,
}

async fn login(
    Extension(db): Extension<Arc<PgPool>>,
    Json(data): Json<LoginParam>,
) -> Result<Json<TokenResponse>> {
    let Some(user) = users::find_by_phone(&*db, &data.phone).await? else {
        return Err(Error::InvalidCredential);
    };

    if false == crate::libs::password::verify(&user.data.password, &data.password.as_bytes()).is_ok() {
        return Err(Error::InvalidCredential);
    };

    let role_data = auth::create_role_token_data(&*db, &user).await?;
    let token = UserToken::sign(user, role_data).await?;

    return Ok(Json(TokenResponse { token }));
}

async fn login_cookie(
    db: Extension<Arc<PgPool>>,
    data: Json<LoginParam>,
) -> Result {
    let Json(response) = login(db, data).await?;
    let cookie = if cfg!(debug_assertions) {
        format!("access_token={}; Path=/; HttpOnly;",response.token.clone())
    } else {
        format!("access_token={}; Path=/; HttpOnly; Secure; SameSite=None",response.token.clone())
    };
    println!("{cookie}");
    let set_cookie = ([("set-cookie", cookie)], Json(response)).into_response();
    Ok(set_cookie)
}

async fn session(user: UserToken) -> Json<UserToken> {
    Json(user)
}

async fn sales(Auth(sales): Auth<Sales>) -> Json<UserToken> {
    Json(sales.0)
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[sqlx::test]
    async fn test(db: PgPool) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let db = Arc::new(db);
        let passwd = "passwd123".to_string();

        let register_param = RegisterParam {
            name: "Burden".into(),
            phone: "089636632561".into(),
            password: passwd.clone(),
        };

        let Json(user) = register(Extension(db.clone()), Json(register_param)).await?;

        let login_param = LoginParam {
            phone: user.data.phone.clone(),
            password: passwd.clone(),
        };

        let _token = login(Extension(db.clone()), Json(login_param.clone())).await?;
        let _cookie = login_cookie(Extension(db), Json(login_param)).await?;

        Ok(())
    }
}
