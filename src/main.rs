#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate hyper_native_tls;
extern crate hyper;

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

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/hello/Unknown")
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

#[get("/view?<para>")]
fn view(para: Para) -> String {
    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let client = Client::with_connector(connector);

    let root_url = "https://esi.evetech.net/verify";
//    let endpoint = Url::parse_with_params(&root_url, &[("types", Self::form_item_url(items, &mut item_count))]).unwrap();
    let urll = Url::parse(root_url).unwrap();

    let mut header = Headers::new();
    header.set(Authorization(format!("Bearer {}", para.access_token).to_owned()));

    // send the GET request
    let mut res = client.get(urll).headers(header).send().unwrap();

    println!("res: {}", res.status);

    format!("Hello, {} year old named {}!", para.access_token, para.token_type);
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    format!("text {}", body)
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect(),
    };

    Template::render("index", &context)
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
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect(),
    };
    Template::render("status", &context)
}

#[error(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, get, status, login, view, callback])
        .attach(Template::fairing())
        .catch(errors![not_found])
}

fn main() {
    rocket().launch();
}