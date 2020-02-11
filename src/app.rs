use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::{
    html,
    services::storage::{Area, StorageService},
    virtual_dom::VNode,
    Component, ComponentLink, ShouldRender,
};

use yew_router::{components::RouterButton, route::Route, router::Router, Switch};

// use crate::pages::home;

// const KEY: &str = "yew.funfun.self";

pub struct App {
    storage: StorageService,
    state: State,
}

#[derive(Serialize, Deserialize)]
pub struct State {}

pub enum Msg {
    Nope,
}

#[derive(Switch, Debug, PartialEq, Clone)]
pub enum AppRoute {
    #[to = "/about"]
    About,
    #[to = "/services"]
    Services,
    #[to = "/contact"]
    Contact,
    #[to = "/page-not-found"]
    NotFound(String),
    #[to = "/home"]
    Home,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local);

        let state = State {};
        App { storage, state }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        info!("update!");
        false
    }

    fn view(&self) -> VNode {
        info!("rendered!");
        html! {
        <div id="layout">
            //<!-- Menu toggle -->
            <a href="#menu" id="menuLink" class="menu-link">
                //<!-- Hamburger icon -->
                <span></span>
            </a>

            <div id="menu">
                <div class="pure-menu">
                    <a class="pure-menu-heading" href="#">{"Company"}</a>

                    <ul class="pure-menu-list">
                        <RouterButton<AppRoute> route=AppRoute::Home> {"Go to H"} </RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route=AppRoute::About> {"Go to A"} </RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route=AppRoute::Services> {"Go to S"} </RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route=AppRoute::Contact> {"Go to C"} </RouterButton<AppRoute>>
                    </ul>
                </div>
            </div>
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| {
                    info!("render! {:?}", switch);
                    match switch {
                        AppRoute::Home => html!{ "Home" },
                        AppRoute::About => html!{ "About" },
                        AppRoute::Services => html!{ "Service" },
                        AppRoute::Contact => html!{ "Contact" },
                        _ => html!{"Page not found"},
                    }
                })
                redirect = Router::redirect(|route: Route| {
                    AppRoute::NotFound(route.route)
                })
            />
        </div>
        }
    }
}

impl App {}
