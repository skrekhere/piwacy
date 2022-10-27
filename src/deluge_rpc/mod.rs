use http::header;
use serde_json::{Value};
pub struct Deluge {
    client: reqwest::Client,
    endpoint: String,
    id: u32,
}

impl Deluge {
    pub fn new(mut endpoint: String) -> Result<Deluge, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        if !endpoint.ends_with("/") {
            endpoint.push('/');
        }
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            "accept",
            header::HeaderValue::from_static("application/json"),
        );
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .default_headers(headers)
            .build()?;

        Ok(Deluge {
            client: client,
            endpoint: endpoint,
            id: 0,
        })
    }

    pub async fn login(&mut self, password: String) -> Result<(), Box<dyn std::error::Error>> {
        let _res = self
            .client
            .post(format!("{}json", self.endpoint))
            .body(format!(
                "{{\"method\": \"auth.login\", \"params\": [\"{}\"], \"id\": {}}}",
                password, self.id
            ))
            .send()
            .await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }
        Ok(())
    }

    pub async fn get_hosts(&mut self) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let _res = self
            .client
            .post(format!("{}json", self.endpoint))
            .body(format!(
                "{{\"method\": \"web.get_hosts\", \"params\": [], \"id\": {}}}",
                self.id
            ))
            .send()
            .await?;
        let res = _res.json::<serde_json::Value>().await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }
        Ok(res)
    }

    pub async fn get_host_status(
        &mut self,
        host: String,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let _res = self
            .client
            .post(format!("{}json", self.endpoint))
            .body(format!(
                "{{\"method\": \"web.get_host_status\", \"params\": [\"{}\"], \"id\": {}}}",
                host, self.id
            ))
            .send()
            .await?;
        let res = _res.json::<serde_json::Value>().await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }
        Ok(res)
    }

    pub async fn connect_host(&mut self, host: String) -> Result<(), Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(format!("{}json", self.endpoint))
            .body(format!(
                "{{\"method\": \"web.connect\", \"params\": [\"{}\"], \"id\": {}}}",
                host, self.id
            ))
            .send()
            .await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }
        Ok(())
    }

    pub async fn connect_to_first_available_host(
        &mut self,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let hosts = self.get_hosts().await?;

        for host in hosts.get("result").unwrap().as_array().unwrap() {
            let name = host.as_array().unwrap().get(0).unwrap().as_str().unwrap();

            let status = self.get_host_status(name.to_string()).await?;
            if status
                .get("result")
                .unwrap()
                .as_array()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .unwrap()
                .to_lowercase()
                == "connected"
            {
                self.connect_host(name.to_string()).await?;
                break;
            }
        }

        Ok(())
    }

    pub async fn add_magnet(
        &mut self,
        magnet: String,
        move_to: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let res = self.client.post(format!("{}json", self.endpoint)).body(format!("{{\"method\": \"core.add_torrent_magnet\", \"params\": [\"{}\", {{\"move_completed_path\": \"{}\"}}], \"id\": {}}}", magnet, move_to, self.id)).send().await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }

        Ok(())
    }

    pub async fn get_torrents_status(&mut self) -> Result<Value, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post(format!("{}json", self.endpoint))
            .body(format!(
                "{{\"method\": \"core.get_torrents_status\", \"params\": [\"\", \"\"], \"id\": {}}}",
                self.id
            ))
            .send()
            .await?;
        self.id += 1;
        if self.id >= 1024 {
            self.id = 0
        }

        Ok(res.json::<Value>().await.unwrap().get("result").unwrap().clone())
    }
}

/*let res = self.client.post("http://192.168.68.123:8112/json").body("{\"method\": \"system.listMethods\", \"params\": [], \"id\": 1}").send().await?;
let ip = res.text().await?;

println!("{:?}", ip);

let res = self.client.post("http://192.168.68.123:8112/json").body("{\"method\": \"auth.login\", \"params\": [\"WeLikeF1nn!\"], \"id\": 2}").send().await?;
let ip1 = res.text().await?;

println!("{:?}", ip1);

let res = self.client.post("http://192.168.68.123:8112/json").body("{\"method\": \"web.get_host_status\", \"params\": [\"20cac8467eac447d93a3969aaad79f94\"], \"id\": 2}").send().await?;
let ip2 = res.text().await?;

println!("{:?}", ip2);

let res = client.post("http://192.168.68.123:8112/json").body("{\"method\": \"web.connect\", \"params\": [\"20cac8467eac447d93a3969aaad79f94\"], \"id\": 2}").send().await?;
let ip3 = res.text().await?;

println!("{:?}", ip3);

let res = client.post("http://192.168.68.123:8112/json").body("{\"method\": \"daemon.get_version\", \"params\": [], \"id\": 2}").send().await?;
let ip4 = res.text().await?;

println!("{:?}", ip4);

let res = client.post("http://192.168.68.123:8112/json").body("{\"method\": \"core.add_torrent_magnet\", \"params\": [\"magnet:?xt=urn:btih:506BCDE8EAB66FD02C84EC18066DBE5408CB3467&dn=The+Nutcracker+and+the+Four+Realms+%282018%29+%5BBluRay%5D+%5B1080p%5D+%5BYTS%5D+%5BYIFY%5D&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2F9.rarbg.com%3A2710%2Fannounce&tr=udp%3A%2F%2Fp4p.arenabg.com%3A1337&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=udp%3A%2F%2Fopentracker.i2p.rocks%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce\", {\"move_completed_path\": \"/downloads/MV\"}], \"id\": 2}").send().await?;
let ip5 = re
s.text().await?;

println!("{:?}", ip5);*/
