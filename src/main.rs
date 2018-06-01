#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate hyper_native_tls;
extern crate hyper;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate ascii;

use std::borrow::Borrow;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request};
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
use std::fmt;
use hyper::net::HttpsConnector;
use hyper::header::Headers;
use hyper::header::Authorization;
use hyper::{Client, Url};
use hyper::client::Response;
use hyper_native_tls::NativeTlsClient;
use std::io::Read;
use serde_json::Value;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use std::vec::Vec;

#[derive(Copy, Clone)]
pub enum Tokens {
    ClientID,
    CallbackURL,
    SecretKey,
    Scope,
}

impl Tokens {
    fn name(&self) -> &'static str {
        match *self {
            Tokens::ClientID => "6721922e87274b38ae0cd015bc67a0c1",
            Tokens::SecretKey => "QAJDzfaVbrXOzgXzrJXzgMpdkmAGstHav7CMBA57",
            Tokens::CallbackURL => "http://localhost:8000/callback",
            Tokens::Scope => "esi-alliances.read_contacts.v1 \
                    esi-assets.read_assets.v1 \
                    esi-assets.read_corporation_assets.v1 \
                    esi-bookmarks.read_character_bookmarks.v1 \
                    esi-bookmarks.read_corporation_bookmarks.v1 \
                    esi-calendar.read_calendar_events.v1 \
                    esi-characters.read_agents_research.v1 \
                    esi-characters.read_blueprints.v1 \
                    esi-characters.read_contacts.v1 \
                    esi-characters.read_corporation_roles.v1 \
                    esi-characters.read_fatigue.v1 \
                    esi-characters.read_fw_stats.v1 \
                    esi-characters.read_loyalty.v1 \
                    esi-characters.read_medals.v1 \
                    esi-characters.read_notifications.v1 \
                    esi-characters.read_opportunities.v1 \
                    esi-characters.read_standings.v1 \
                    esi-characters.read_titles.v1 \
                    esi-characterstats.read.v1 \
                    esi-clones.read_clones.v1 \
                    esi-clones.read_implants.v1 \
                    esi-contracts.read_character_contracts.v1 \
                    esi-contracts.read_corporation_contracts.v1 \
                    esi-corporations.read_blueprints.v1 \
                    esi-corporations.read_contacts.v1 \
                    esi-corporations.read_container_logs.v1 \
                    esi-corporations.read_corporation_membership.v1 \
                    esi-corporations.read_divisions.v1 \
                    esi-corporations.read_facilities.v1 \
                    esi-corporations.read_fw_stats.v1 \
                    esi-corporations.read_medals.v1 \
                    esi-corporations.read_outposts.v1 \
                    esi-corporations.read_standings.v1 \
                    esi-corporations.read_starbases.v1 \
                    esi-corporations.read_structures.v1 \
                    esi-corporations.read_titles.v1 \
                    esi-corporations.track_members.v1 \
                    esi-fittings.read_fittings.v1 \
                    esi-fleets.read_fleet.v1 \
                    esi-industry.read_character_jobs.v1 \
                    esi-industry.read_character_mining.v1 \
                    esi-industry.read_corporation_jobs.v1 \
                    esi-industry.read_corporation_mining.v1 \
                    esi-killmails.read_corporation_killmails.v1 \
                    esi-killmails.read_killmails.v1 \
                    esi-location.read_location.v1 \
                    esi-location.read_online.v1 \
                    esi-location.read_ship_type.v1 \
                    esi-mail.read_mail.v1 \
                    esi-markets.read_character_orders.v1 \
                    esi-markets.read_corporation_orders.v1 \
                    esi-planets.manage_planets.v1 \
                    esi-planets.read_customs_offices.v1 \
                    esi-skills.read_skillqueue.v1 \
                    esi-skills.read_skills.v1 \
                    esi-universe.read_structures.v1 \
                    esi-wallet.read_character_wallet.v1 \
                    esi-wallet.read_corporation_wallets.v1",
        }
    }
}

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<String>,
}

#[get("/login")]
fn login() -> Redirect {
    Redirect::to(format!("https://login.eveonline.com/oauth/authorize?\
    response_type=token&redirect_uri={}\
    &client_id={}&scope={}",
                         Tokens::CallbackURL.name(),
                         Tokens::ClientID.name(),
                         Tokens::Scope.name()
    ).as_ref())
}

#[get("/callback")]
fn callback() -> Template {
    let mut context = HashMap::new();
    context.insert("test", "test");
    #[allow(unused_variables)]
        Template::render("callback", context)
}


#[derive(Debug)]
struct HeaderCount(usize);

impl fmt::Display for HeaderCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for HeaderCount {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, ()> {
        println!("request :{} ", request.get_param_str(0).unwrap_or("".into()));
        Success(HeaderCount(request.headers().len()))
    }
}

#[derive(FromForm)]
struct Para {
    access_token: String,
    token_type: String,
    expires_in: Option<u8>,
}

fn get_character_id(access_token: String) -> String {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let baseURL = String::from("https://esi.evetech.net");

    let mut verifyURL = baseURL;
    verifyURL = verifyURL +"/verify";
//    let endpoint = Url::parse_with_params(&root_url, &[("types", Self::form_item_url(items, &mut item_count))]).unwrap();
    let urll = Url::parse(verifyURL.as_ref()).unwrap();

    let mut header = Headers::new();
    header.set(Authorization(format!("Bearer {}", access_token).to_owned()));

    // send the GET request
    let mut res = client.get(urll).headers(header).send().unwrap();

    println!("res: {}", res.status);

    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    let text: Value = serde_json::from_str(body.as_str()).unwrap();
    println!("id: {}", text["CharacterID"]);

    format!("{}", text["CharacterID"])
}

#[derive(Serialize, Deserialize)]
struct ResultTemplateContext {
    contents: String,
}

#[derive(FromForm)]
struct ViewPara {
    base: String,
    characterID: String,
    hook: String,
    token: String,
}

#[get("/view?<para>")]
fn view(para: ViewPara) -> Template {
    println!("{}/{}/{}/{}",
            para.base, para.characterID, para.hook, para.token);
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

//    let characterID = get_character_id(para.access_token.clone());

    let baseURL = String::from("https://esi.evetech.net");
//    let hook = String::from("skills/");
    let mut portraitURL = format!("/dev/{}/{}/{}?datasource=tranquility&token={}",
                                  para.base, para.characterID, para.hook, para.token);
//    println!("test {}", portraitURL);
    portraitURL = baseURL + portraitURL.as_str();

//    println!("test {}", portraitURL);

    let purl = Url::parse(portraitURL.as_ref()).unwrap();

    let mut res = client.get(purl).send().unwrap();

    let mut body1 = String::new();
    res.read_to_string(&mut body1).unwrap();

    let mut map = std::collections::HashMap::new();
    map.insert("contents", body1);
    Template::render("result", &map)
}

#[derive(Serialize)]
struct Dis {
    display: String,
    url: String,
}

#[get("/links?<para>")]
fn links(para: Para) -> Template {
    let characterID = get_character_id(para.access_token.clone());

//    let baseURL = String::from("https://esi.evetech.net");
    let hooks: Vec<&str> = vec!["",
                                "skills",
                                "agents_research", "assets",
                                "calendar", "blueprints",
                                "roles", "stats",
                                "contacts", "contracts",
                                "mining", "industry/jobs",
                                "mail", "orders", "orders/history"];
//    let mut portraitURL: String = format!("/characters/{}/{}"
//                                  ,characterID, hook);

    let mut links: Vec<Dis> = Vec::new();

    for hook in hooks.iter() {
        let tmp = Dis {
            url: format!("/view?base=characters&characterID={}&hook={}&token={}" ,
                         characterID, hook.to_string(), para.access_token),
            display: format!("/characters/{}/{}" ,
                             characterID, hook.to_string())
        };
        links.push(tmp );
    }

    let mut map =
        std::collections::HashMap::new();

    map.insert("items", links);
    Template::render("links", &map)
}



#[derive(Serialize)]
struct LinksTemplateContext {
    items: Vec<String>
}

#[derive(Serialize)]
struct TemplateContext1 {
    path: String,
    items: Vec<String>,
}

#[get("/status")]
fn status() -> Template {
    let context = TemplateContext1 {
        path: "Status".to_string(),
        items: vec!["One", "Two", "Three"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    };
    Template::render("status", &context)
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map =
        std::collections::HashMap::new();

    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("templates/").join(file)).ok()
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("templates/index.html")
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/",
               routes![index, status, login, view,
                callback, links])
        .mount("/file",
               routes![files])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}