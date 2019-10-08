use reqwest::{Client, Url, Result, Response};

const BASE_URL: &str = "http://download.unity3d.com/download_unity/";

fn load_resource() -> Result<Response> {
    let client = Client::new();
    let url = Url::parse(BASE_URL).unwrap();
    let url = url.join("download_unity/6d84dfc57ccf/unity-2017.4.9f1-win.ini").unwrap();
    client.get(url).send()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_load_a_resource() {
        let r = load_resource();
        assert!(r.is_ok());
    }
}
