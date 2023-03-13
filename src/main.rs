use rocket::serde::json::{Json};
use rocket::serde::{Serialize, Deserialize};
// use rocket::Response;
use rocket::response::{status};
use rocket::http::{Status};

#[macro_use] extern crate rocket;

#[derive(Serialize, Deserialize, Debug)]
struct UserAuth<'r> {
    code: &'r str,
}

#[derive(Serialize, Deserialize, Debug)]
struct TokenResponse {
    token_type: String,
    expires_in: Option<i32>,
    access_token: String,
    scope: String,
    refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterRetweet {
    id: String,
    cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterRetweetMeta {
    result_count: i64,
    next_token: Option<String>,
    previous_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterFollower<'r> {
    username: &'r str,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterUser {
    id: String,
    name: String,
    username: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterError {
    detail: Option<String>,
    title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterUserResponse {
    data: Option<TwitterUser>,
    errors: Option<Vec<TwitterError>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TwitterUsersResponse {
    data: Vec<TwitterUser>,
    meta: TwitterRetweetMeta,
}

#[derive(Serialize, Deserialize, Debug)]
struct RetweetResponse {
    user_id: String,
    tweet_id: String,
    handle: String,
    name: String,
    timestamp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct RetweetUserResponse {
    username: String,
    tweet_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TweetResponse {
    retweet_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiscordUserResponse {
    id: String,
    username: String,
    avatar: Option<String>,
    discriminator: String,
    locale: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubUserResponse {
    id: i32,
    login: String,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResponseResult<T> {
    code: i32,
    data: Option<T>,
    message: Option<String>,
}

const GITHUB_BASE_API: &str = "https://github.com";
const GITHUB_TOKEN_API: &str = "/login/oauth/access_token";
const GITHUB_USER_ME_API: &str = "https://api.github.com/user";
const GITHUB_REDIRECT_URI: &str = "https://qescr-raaaa-aaaak-qbufa-cai.ic0.app";
const GITHUB_CLIENT_ID: &str = "f2e94d951f9681739f57";
const GITHUB_CLIENT_SECRET: &str = "31495fe947ff03fa4ebf7c71c242d21b4dffa6f3";

#[post("/github/user", format = "json", data = "<user_auth>")]
async fn github_user(user_auth: Json<UserAuth<'_>>) -> Json<ResponseResult<String>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let params = [("code", user_auth.code)
            , ("client_id", GITHUB_CLIENT_ID)
            , ("client_secret", GITHUB_CLIENT_SECRET)
            , ("redirect_uri", GITHUB_REDIRECT_URI)];

    let res = client.post(String::from(GITHUB_BASE_API) + GITHUB_TOKEN_API)
        .header(reqwest::header::ACCEPT, "application/json")
        .form(&params)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_code"))});
    }
    let token_response = res.json::<TokenResponse>().await.unwrap();

    let res = client.get(String::from(GITHUB_USER_ME_API))
        .header(reqwest::header::AUTHORIZATION, String::from("token ") + &token_response.access_token)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::USER_AGENT, "Application/Ratels")
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let github_user_res = res.json::<GithubUserResponse>().await.unwrap();
    Json(ResponseResult::<String> {code: 200, data: Some(github_user_res.login), message: None})
}

const DISCORD_BASE_API: &str = "https://discord.com";
const DISCORD_TOKEN_API: &str = "/api/v10/oauth2/token";
const DISCORD_USER_ME_API: &str = "/api/v10/users/@me";
const DISCORD_REDIRECT_URI: &str = "https://qescr-raaaa-aaaak-qbufa-cai.ic0.app";
const DISCORD_CLIENT_ID: &str = "982876194980634686";
const DISCORD_CLIENT_SECRET: &str = "VNWaLmQNT6RXcB0j-fnWEhymYtLEdmlr";

#[post("/discord/user", format = "json", data = "<user_auth>")]
async fn discord_user(user_auth: Json<UserAuth<'_>>) -> Json<ResponseResult<String>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let params = [("code", user_auth.code)
            , ("grant_type", "authorization_code")
            , ("client_id", DISCORD_CLIENT_ID)
            , ("client_secret", DISCORD_CLIENT_SECRET)
            , ("redirect_uri", DISCORD_REDIRECT_URI)];

    let res = client.post(String::from(DISCORD_BASE_API) + DISCORD_TOKEN_API)
        .form(&params)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_code"))});
    }
    let token_response = res.json::<TokenResponse>().await.unwrap();

    let res = client.get(String::from(DISCORD_BASE_API) + DISCORD_USER_ME_API)
        .header(reqwest::header::AUTHORIZATION, token_response.token_type + " " + &token_response.access_token)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let discord_user_res = res.json::<DiscordUserResponse>().await.unwrap();
    Json(ResponseResult::<String> {code: 200, data: Some(discord_user_res.username + "#" + &discord_user_res.discriminator), message: None})
}

const TWITTER_BASE_API: &str = "https://api.twitter.com";
const TWITTER_TOKEN_API: &str = "/2/oauth2/token";
const TWITTER_USER_ME_API: &str = "/2/users/me";
const TWITTER_USER_USERNAME_API: &str = "/2/users/by/username/";
const TWITTER_REDIRECT_URI: &str = "https://qescr-raaaa-aaaak-qbufa-cai.ic0.app";
const TWITTER_CLIENT_ID: &str = "dVJLSzBFeVBEVTh6blhXdjY2VUQ6MTpjaQ";
const TWITTER_BEARER_TOKEN: &str = "Bearer AAAAAAAAAAAAAAAAAAAAAGt2cwEAAAAAeLCD9v62qm%2Fa84rdvX8cuPKnTow%3DGRAu59Gzg589E1rp39iXpmPTHbnwMv2zr0no8rZ6ezskBTA8WC";

#[post("/twitter/user", format = "json", data = "<user_auth>")]
async fn twitter_user(user_auth: Json<UserAuth<'_>>) -> Json<ResponseResult<String>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let params = [("code", user_auth.code)
            , ("grant_type", "authorization_code")
            , ("client_id", TWITTER_CLIENT_ID)
            , ("redirect_uri", TWITTER_REDIRECT_URI)
            , ("code_verifier", "challenge")];

    let res = client.post(String::from(TWITTER_BASE_API) + TWITTER_TOKEN_API)
        .form(&params)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_code"))});
    }
    let token_response = res.json::<TokenResponse>().await.unwrap();

    let res = client.get(String::from(TWITTER_BASE_API) + TWITTER_USER_ME_API)
        .header(reqwest::header::AUTHORIZATION, token_response.token_type + " " + &token_response.access_token)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let twitter_user_res = res.json::<TwitterUserResponse>().await.unwrap();

    let username = match twitter_user_res.errors {
        Some(_errors) => return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_user"))}),
        None => match twitter_user_res.data {
            Some(data) => data.username,
            None => return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_user"))}),
        }
    };

    Json(ResponseResult::<String> {code: 200, data: Some(username), message: None})
}

const TWITTER_PICKER_RETWEET_API: &str = "https://api.twitterpicker.com/tweet/retweets";

#[post("/twitter/retweet", format = "json", data = "<retweet>")]
async fn twitter_retweet(retweet: Json<TwitterRetweet>) -> Json<ResponseResult<Vec<RetweetUserResponse>>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let cursor : u64 = match &retweet.cursor {
        Some(res_cursor) => res_cursor.parse::<u64>().unwrap(),
        None => 9000000000000000000,
    };
    let params = [
        ("id", &retweet.id),
        ("cursor", &(cursor - 1).to_string()),
    ];
    let url = reqwest::Url::parse_with_params(TWITTER_PICKER_RETWEET_API, &params).unwrap();
    let res = client.get(url)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let retweet_res = res.json::<Vec<RetweetResponse>>().await.unwrap();
    let mut retweet_users = Vec::new();
    for retweet in retweet_res {
        let retweet_user = RetweetUserResponse {
            username: retweet.handle,
            tweet_id: retweet.tweet_id,
        };
        retweet_users.push(retweet_user);
    }
    Json(ResponseResult::<Vec<RetweetUserResponse>> {code: 200, data: Some(retweet_users), message: None})
}

const TWITTER_PICKER_RETWEET_DATA_API: &str = "https://api.twitterpicker.com/tweet/data";

#[post("/twitter/retweet/count", format = "json", data = "<retweet>")]
async fn twitter_retweet_count(retweet: Json<TwitterRetweet>) -> Json<ResponseResult<TweetResponse>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let params = [
        ("id", &retweet.id),
    ];
    let url = reqwest::Url::parse_with_params(TWITTER_PICKER_RETWEET_DATA_API, &params).unwrap();
    let res = client.get(url)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        let error = res.text().await.unwrap();
        println!("{}", error);
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("twitter_error"))});
    }

    let retweet_res = res.json::<TweetResponse>().await.unwrap();
    Json(ResponseResult::<TweetResponse> {code: 200, data: Some(retweet_res), message: None})
}

#[post("/twitter/follower", format = "json", data = "<follower>")]
async fn twitter_follower(follower: Json<TwitterFollower<'_>>) -> Json<ResponseResult<Vec<String>>> {
    let client = reqwest::Client::new();
    // let proxy = reqwest::Proxy::all("http://127.0.0.1:1086").unwrap();
    // let client = reqwest::Client::builder().proxy(proxy).build().unwrap();

    let res = client.get(String::from(TWITTER_BASE_API) + TWITTER_USER_USERNAME_API + follower.username)
        .header(reqwest::header::AUTHORIZATION, TWITTER_BEARER_TOKEN)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let twitter_user_res = res.json::<TwitterUserResponse>().await.unwrap();

    let user_id = match twitter_user_res.errors {
        Some(_errors) => return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_user"))}),
        None => match twitter_user_res.data {
            Some(data) => data.id,
            None => return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_user"))}),
        }
    };

    let res = client.get(String::from(TWITTER_BASE_API) + &format!("/2/users/{}/followers", user_id))
        .header(reqwest::header::AUTHORIZATION, TWITTER_BEARER_TOKEN)
        .send()
        .await
        .unwrap();

    if res.status() != 200 {
        return Json(ResponseResult {code: 500, data: None, message: Some(String::from("invalid_token"))});
    }

    let follower_res = res.json::<TwitterUsersResponse>().await.unwrap();
    let mut followers = Vec::new();
    for follower in follower_res.data {
        followers.push(follower.username);
    }
    Json(ResponseResult::<Vec<String>> {code: 200, data: Some(followers), message: None})
}

#[catch(default)]
fn default() -> status::Custom<String> {//Json<ResponseResult<String>> {
    // let mut response = Response::new();
    // response.set_status(Status::Ok);
    // response.set_header(ContentType::JSON);
    status::Custom(Status::Ok, String::from("{\"code\": 500, \"message\": \"system_error\"}"))
    // Json(ResponseResult::<String> {code: 500, data: None, message: Some(String::from("system_error"))})
}

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount("/api/v1/ratels", routes![twitter_user, twitter_retweet, twitter_retweet_count, twitter_follower, discord_user, github_user])
        .register("/", catchers![default])
}