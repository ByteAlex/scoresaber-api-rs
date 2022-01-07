use reqwest::{Client, Request};
use serde::de::DeserializeOwned;
use crate::model::{DifficultyEnum, PlayerCollection, PlayerScoreCollection, ScoreSort, SortCategory, SortOrder};

const BASE_URL: &'static str = "https://scoresaber.com";

pub enum Route {
    // Leaderboards
    GetLeaderboards(RouteParams<(), GetLeaderboardsQuery>),
    GetLeaderboardInfoById(RouteParams<u32, NoQuery>),
    GetLeaderboardInfoByHash(RouteParams<String, GetLeaderboardInfoByHashQuery>),
    GetLeaderboardScoresById(RouteParams<u32, GetLeaderboardScoresByIdQuery>),
    GetLeaderboardScoresByHash(RouteParams<String, GetLeaderboardScoresByHashQuery>),
    GetLeaderboardDifficulties(RouteParams<String, NoQuery>),
    // Players
    GetPlayers(RouteParams<(), GetPlayersQuery>),
    GetPlayersCount(RouteParams<(), GetPlayersCountQuery>),
    GetPlayerBasic(RouteParams<String, NoQuery>),
    GetPlayerFull(RouteParams<String, NoQuery>),
    GetPlayerScores(RouteParams<String, GetPlayerScoresQuery>),
}

pub trait Query {
    fn parse(self) -> Vec<(&'static str, String)>;
}

macro_rules! query_optional {
    ($obj:ident, $field:expr, $name:literal) => {
        if let Some(val) = $field {
            $obj.push(($name, val.to_string()))
        }
    }
}

pub struct RouteParams<P, Q: Query> {
    path: P,
    query: Q,
}

impl<P, Q: Query> RouteParams<P, Q> {
    pub fn new(path: P, query: Q) -> Self {
        Self { path, query }
    }
}

#[derive(Copy, Clone, Default)]
pub struct NoQuery {}

impl Query for NoQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        Vec::new()
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetLeaderboardsQuery {
    pub search: Option<String>,
    pub verified: Option<bool>,
    pub ranked: Option<bool>,
    pub qualified: Option<bool>,
    pub loved: Option<bool>,
    pub min_star: Option<u32>,
    pub max_star: Option<u32>,
    pub category: Option<SortCategory>,
    pub sort: Option<SortOrder>,
    pub unique: Option<bool>,
    pub page: Option<u32>,
    pub with_metadata: Option<bool>,
}

impl Query for GetLeaderboardsQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        query_optional!(parse, self.search, "search");
        query_optional!(parse, self.verified, "verified");
        query_optional!(parse, self.ranked, "ranked");
        query_optional!(parse, self.qualified, "qualified");
        query_optional!(parse, self.min_star, "minStar");
        query_optional!(parse, self.max_star, "maxStar");
        query_optional!(parse, self.category, "category");
        query_optional!(parse, self.sort, "sort");
        query_optional!(parse, self.unique, "unique");
        query_optional!(parse, self.page, "page");
        query_optional!(parse, self.with_metadata, "withMetadata");
        parse
    }
}

#[derive(Debug, Clone)]
pub struct GetLeaderboardInfoByHashQuery {
    pub difficulty: DifficultyEnum,
    pub game_mode: Option<String>,
}

impl Query for GetLeaderboardInfoByHashQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        parse.push(("difficulty", self.difficulty.to_string()));
        query_optional!(parse, self.game_mode, "gameMode");
        parse
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetLeaderboardScoresByIdQuery {
    pub countries: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub with_metadata: Option<bool>,
}

impl Query for GetLeaderboardScoresByIdQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        query_optional!(parse, self.countries, "countries");
        query_optional!(parse, self.search, "search");
        query_optional!(parse, self.page, "page");
        query_optional!(parse, self.with_metadata, "withMetadata");
        parse
    }
}

#[derive(Debug, Clone)]
pub struct GetLeaderboardScoresByHashQuery {
    pub difficulty: DifficultyEnum,
    pub game_mode: Option<String>,
    pub countries: Option<String>,
    pub search: Option<String>,
    pub page: Option<u32>,
    pub with_metadata: Option<bool>,
}

impl Query for GetLeaderboardScoresByHashQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        parse.push(("difficulty", self.difficulty.to_string()));
        query_optional!(parse, self.game_mode, "gameMode");
        query_optional!(parse, self.countries, "countries");
        query_optional!(parse, self.search, "search");
        query_optional!(parse, self.page, "page");
        query_optional!(parse, self.with_metadata, "withMetadata");
        parse
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetPlayersQuery {
    pub search: Option<String>,
    pub page: Option<u32>,
    pub countries: Option<String>,
    pub with_metadata: Option<bool>,
}

impl Query for GetPlayersQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        query_optional!(parse, self.search, "search");
        query_optional!(parse, self.page, "page");
        query_optional!(parse, self.countries, "countries");
        query_optional!(parse, self.with_metadata, "withMetadata");
        parse
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetPlayersCountQuery {
    pub search: Option<String>,
    pub countries: Option<String>,
}

impl Query for GetPlayersCountQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        query_optional!(parse, self.search, "search");
        query_optional!(parse, self.countries, "countries");
        parse
    }
}

#[derive(Debug, Clone, Default)]
pub struct GetPlayerScoresQuery {
    pub limit: Option<u32>,
    pub sort: Option<ScoreSort>,
    pub page: Option<u32>,
    pub with_metadata: Option<bool>,
}

impl Query for GetPlayerScoresQuery {
    fn parse(self) -> Vec<(&'static str, String)> {
        let mut parse = Vec::new();
        query_optional!(parse, self.limit, "limit");
        query_optional!(parse, self.sort, "sort");
        query_optional!(parse, self.page, "page");
        query_optional!(parse, self.with_metadata, "withMetadata");
        parse
    }
}

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new(http: Client) -> Self {
        Self {
            client: http,
        }
    }

    pub async fn execute<'de, T: DeserializeOwned>(&self, route: Route) -> reqwest::Result<T> {
        let request = route.to_request(&self);
        self.client.execute(request).await?
            .text()
            .await
            .map(|res| {
                serde_json::from_str(res.as_str())
                    .map_err(|err| {
                        println!("{}", res);
                        println!("{}", err);
                    })
                    .expect("Failed to parse...")
            })
    }

    pub async fn get_players(&self, query: Option<GetPlayersQuery>) -> reqwest::Result<PlayerCollection> {
        self.execute(Route::GetPlayers(RouteParams::new((), query.unwrap_or_default()))).await
    }

    pub async fn get_player_scores(&self, player_id: String, query: Option<GetPlayerScoresQuery>) -> reqwest::Result<PlayerScoreCollection> {
        self.execute(Route::GetPlayerScores(RouteParams::new(player_id, query.unwrap_or_default()))).await
    }
}

impl Route {
    fn build_url(&self) -> String {
        let mut url = BASE_URL.to_owned();
        match self {
            Route::GetLeaderboards(_) => url.push_str(format!("/api/leaderboards").as_str()),
            Route::GetLeaderboardInfoById(p) => url.push_str(format!("/api/leaderboard/by-id/{}/info", p.path).as_str()),
            Route::GetLeaderboardInfoByHash(p) => url.push_str(format!("/api/leaderboard/by-hash/{}/info", p.path).as_str()),
            Route::GetLeaderboardScoresById(p) => url.push_str(format!("/api/leaderboard/by-id/{}/scores", p.path).as_str()),
            Route::GetLeaderboardScoresByHash(p) => url.push_str(format!("/api/leaderboard/by-hash/{}/scores", p.path).as_str()),
            Route::GetLeaderboardDifficulties(p) => url.push_str(format!("/api/leaderboard/get-difficulties/{}", p.path).as_str()),
            Route::GetPlayers(_) => url.push_str(format!("/api/players").as_str()),
            Route::GetPlayersCount(_) => url.push_str(format!("/api/players/count").as_str()),
            Route::GetPlayerBasic(p) => url.push_str(format!("/api/player/{}/basic", p.path).as_str()),
            Route::GetPlayerFull(p) => url.push_str(format!("/api/player/{}/full", p.path).as_str()),
            Route::GetPlayerScores(p) => url.push_str(format!("/api/player/{}/scores", p.path).as_str()),
        }
        return url;
    }

    fn to_request(self, api: &ApiClient) -> Request {
        let req = api.client.get(self.build_url());
        let params = match self {
            Route::GetLeaderboards(params) => params.query.parse(),
            Route::GetLeaderboardInfoById(params) => params.query.parse(),
            Route::GetLeaderboardInfoByHash(params) => params.query.parse(),
            Route::GetLeaderboardScoresById(params) => params.query.parse(),
            Route::GetLeaderboardScoresByHash(params) => params.query.parse(),
            Route::GetLeaderboardDifficulties(params) => params.query.parse(),
            Route::GetPlayers(params) => params.query.parse(),
            Route::GetPlayersCount(params) => params.query.parse(),
            Route::GetPlayerBasic(params) => params.query.parse(),
            Route::GetPlayerFull(params) => params.query.parse(),
            Route::GetPlayerScores(params) => params.query.parse(),
        };

        req.query(&params).build().unwrap()
    }
}