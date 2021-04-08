use crate::error::Error;
use crate::models::*;

//and_then trait
use futures::future::TryFutureExt;

#[derive(Clone)]
pub struct UnifiClient {
    pub(crate) url: String,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) site: String,
    pub(crate) group: String,
    pub(crate) client: reqwest::Client,
}

impl UnifiClient {
    pub async fn get_state(&self) -> Result<Option<Rate>, Error> {
        self.authenticate()
            .and_then(|_| async move { self.get_client_group_rate().await })
            .and_then(|c| async move { Ok(c.map(|i| i.into())) })
            .err_into()
            .await
    }

    pub async fn set_state(&self, down: i32, up: i32) -> Result<(), Error> {
        self.authenticate()
            .and_then(|_| async move { self.get_client_group_rate().await })
            .err_into()
            .and_then(|oc: Option<ClientGroupResponseData>| async move {
                match oc {
                    Some(c) => Ok(c),
                    None => Err(Error::from_str("no group found!")),
                }
            })
            .and_then(|c| async move {
                let updated = ClientGroupResponseData {
                    max_down: down,
                    max_up: up,
                    ..c
                };
                self.set_client_group_rate(updated).err_into().await
            })
            .await
    }

    async fn authenticate(&self) -> reqwest::Result<()> {
        info!("Authenticate");
        let a = Auth {
            username: self.user.to_string(),
            password: self.password.to_string(),
        };
        self.client
            .post(format!("{}/api/login", self.url))
            .json(&a)
            .send()
            .and_then(|_| async move { Ok(()) })
            .await
    }

    async fn get_client_group_rate(&self) -> reqwest::Result<Option<ClientGroupResponseData>> {
        self.client
            .get(format!("{}/api/s/{}/list/usergroup", self.url, self.site))
            .send()
            .and_then(|res| async move { res.json::<ClientGroupsResponse>().await })
            .and_then(|cg: ClientGroupsResponse| async move { Ok(cg.data) })
            .and_then(|vec: Vec<ClientGroupResponseData>| async move {
                let maybe_item = vec.iter().find(|item| item.name == self.group);
                debug!("Rate: {:#?}", maybe_item);
                Ok(maybe_item.cloned())
            })
            .await
    }

    async fn set_client_group_rate(&self, data: ClientGroupResponseData) -> reqwest::Result<()> {
        self.client
            .put(format!(
                "{}/api/s/{}/rest/usergroup/{}",
                self.url, self.site, &data.id
            ))
            .json(&data)
            .send()
            .and_then(|_| async move { Ok(()) })
            .await
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[actix_rt::test]
    async fn get_state_test() {
        env_logger::init();
        let client: UnifiClient = UnifiClient {
            url: "https://unifi:8443".to_string(),
            user: "admin".to_string(),
            password: "admin123".to_string(),
            site: "default".to_string(),
            group: "De-prio".to_string(),
            client: reqwest::Client::builder()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        };

        match client.get_state().await {
            Ok(r) => {
                assert_eq!(
                    r,
                    Some(Rate {
                        max_up: -1,
                        max_down: -1
                    })
                )
            }
            Err(e) => error!("{}", e),
        };
    }

    #[actix_rt::test]
    async fn set_state_test() {
        env_logger::init();
        let client: UnifiClient = UnifiClient {
            url: "https://unifi:8443".to_string(),
            user: "admin".to_string(),
            password: "admin123".to_string(),
            site: "default".to_string(),
            group: "De-prio".to_string(),
            client: reqwest::Client::builder()
                .cookie_store(true)
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap(),
        };
        let res = client.set_state(-1, -1).await;
        assert!(res.is_ok())
    }
}
