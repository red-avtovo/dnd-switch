mod models;

use crate::models::DndState;
use log::{debug, info};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yewprint::{Spinner, Switch};

#[macro_use]
extern crate stdweb;
use crate::stdweb::unstable::TryInto;

static BE_PORT: i32 = 8001;

struct Model {
    be_url: String,
    link: ComponentLink<Self>,
    active: bool,
    loading: bool,
    broken: bool,
}

use futures::future::TryFutureExt;

enum Msg {
    Toggle,
    On,
    Off,
    TogglePerform,
    Load,
    LoadPerform(bool),
    Done,
    Broken,
}

async fn wrap<F: std::future::Future>(f: F, done_cb: yew::Callback<F::Output>) {
    done_cb.emit(f.await);
}

async fn turn_on(url: String) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/state", url))
        .json(&DndState { state: true })
        .send()
        .and_then(|_| async move { Ok(()) })
        .map_err(|_| "unable to perform the call".to_string())
        .await
}

async fn turn_off(url: String) -> Result<(), String> {
    reqwest::Client::new()
        .post(format!("{}/state", url))
        .json(&DndState { state: false })
        .send()
        .and_then(|_| async move { Ok(()) })
        .map_err(|_| "unable to perform the call".to_string())
        .await
}

async fn load_state(url: String) -> Result<bool, String> {
    let res: Result<bool, String> = reqwest::Client::new()
        .get(format!("{}/state", url))
        .send()
        .and_then(|res| async move { res.json::<DndState>().await })
        .and_then(|dnd: DndState| async move { Ok(dnd.state) })
        .map_err(|_| "unable to perform the call".to_string())
        .await;
    debug!("Check successful: {}", res.is_ok());
    res
}

fn get_be_url() -> String {
    let hostname: String = (js! {
        return window.location.hostname
    })
    .try_into()
    .unwrap();
    format!("http://{}:{}", hostname, BE_PORT)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let s = Self {
            be_url: get_be_url(),
            link,
            active: false,
            loading: false,
            broken: false,
        };
        s.link.send_message(Msg::Load);
        s
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => match self.active {
                true => self.link.send_message(Msg::Off),
                false => self.link.send_message(Msg::On),
            },
            Msg::On => {
                self.loading = true;
                spawn_local(wrap(
                    turn_on(self.be_url.clone()),
                    self.link.callback(|res| match res {
                        Ok(_) => Msg::TogglePerform,
                        Err(_) => Msg::Done,
                    }),
                ))
            }
            Msg::Off => {
                self.loading = true;
                spawn_local(wrap(
                    turn_off(self.be_url.clone()),
                    self.link.callback(|res| match res {
                        Ok(_) => Msg::TogglePerform,
                        Err(_) => Msg::Done,
                    }),
                ))
            }
            Msg::TogglePerform => {
                self.active = !self.active;
                self.loading = false;
            }
            Msg::Done => self.loading = false,
            Msg::Load => {
                self.loading = true;
                spawn_local(wrap(
                    load_state(self.be_url.clone()),
                    self.link.callback(|res| match res {
                        Ok(s) => Msg::LoadPerform(s),
                        Err(_) => Msg::Broken,
                    }),
                ))
            }
            Msg::LoadPerform(s) => {
                self.active = s;
                self.loading = false;
            }
            Msg::Broken => {
                self.active = false;
                self.broken = true;
                self.loading = false;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let spinner_class = match self.loading {
            false => "hidden",
            true => "",
        }
        .to_string();

        return html! {
            <div class={"wrapper"}>
                <div class={"container"}>
                    <h1>{"Do not disturb traffic"}</h1>
                    <Switch disabled={self.broken || self.loading} checked={self.active} large={true} onclick=self.link.callback(|_| Msg::Toggle) />
                    <Spinner class=classes!(spinner_class) size={50.0} />
                </div>
            </div>
        };
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Model>::new().mount_to_body();
}
