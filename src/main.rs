#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::Request;
use rocket::response::Redirect;
use rocket_contrib::Template;
use std::collections::HashMap;
//use std::fmt;

#[derive(Copy, Clone)]
pub enum Tokens {
    ClientID,
    CallbackURL,
    SecretKey,
    Scope,
}

impl Tokens {
//    fn id_str(&self) -> &'static str {
//        match *self {
//            Hubs::Jita    => "60003760",
//            Hubs::Amarr   => "60008494",
//            Hubs::Dodixie => "60011866",
//            Hubs::Rens    => "60004588",
//            Hubs::Hek     => "60005686",
//        }
//    }
    fn name(&self) -> &'static str {
        match *self {
            Tokens::ClientID    => "6721922e87274b38ae0cd015bc67a0c1",
            Tokens::SecretKey   => "QAJDzfaVbrXOzgXzrJXzgMpdkmAGstHav7CMBA57",
            Tokens::CallbackURL => "http://localhost:8000/callback",
            Tokens::Scope       => "esi-alliances.read_contacts.v1 \
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
    items: Vec<String>
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

#[get("/view")]
fn view() -> &'static str {
    "view output"
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
    let context = TemplateContext {
        name: name,
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect()
    };

    Template::render("index", &context)
}


#[derive(Serialize)]
struct TemplateContext1 {
    path: String,
    items: Vec<String>
}

#[get("/status")]
fn status() -> Template {
    let context = TemplateContext1 {
        path: "Status".to_string(),
        items: vec!["One", "Two", "Three"].iter().map(|s| s.to_string()).collect()
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