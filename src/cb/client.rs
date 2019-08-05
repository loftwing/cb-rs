#![allow(dead_code)]
use reqwest::{Client, header};
use super::types::*;


pub struct CbClient {
    client: Client,
    url: String,
}

//TODO: refactor this into trait?
impl CbClient {
    pub fn new(apikey: &str, url: &str) -> Result<CbClient, Box<dyn std::error::Error>> {
        let mut headers = header::HeaderMap::new();
        let auth_header = header::HeaderValue::from_str(apikey)?;
        headers.insert("X-Auth-Token", auth_header);

        let client = Client::builder()
            .default_headers(headers)
            .build()?;
        
        Ok(CbClient{client: client, url: format!("https://{}/", url.to_owned())})
    }

    pub fn get_device_status(self, device_id: i64) -> Result<Option<CbDeviceStatus>, Box<dyn std::error::Error>> {
        let req_url = format!("{}{}{}", self.url, "integrationServices/v3/device/", device_id);
        let mut res = self.client.get(&req_url).send()?;

        let cbstatus_txt = res.text()?;
        
        let cbstatusresponse: CbSingleDeviceStatusResponse = serde_json::from_str(&cbstatus_txt)?;

        Ok(cbstatusresponse.deviceInfo)        
    }

    pub fn get_all_devices(self) -> Result<Vec<CbDevice>, Box<dyn std::error::Error>> {
        let req_url = format!("{}{}", self.url, "integrationServices/v3/device/all?fileFormat=json");
        let mut res = self.client.get(&req_url).send()?;

        let cb_device_txt = res.text()?;
        let cb_device_resp: CbDevicesResponse = serde_json::from_str(&cb_device_txt)?;


        Ok(cb_device_resp.results)
    }

    pub fn get_all_devices_status(self) -> Result<Vec<CbDeviceStatus>, Box<dyn std::error::Error>> {
        let req_url = format!("{}{}", self.url, "integrationServices/v3/device");
        let mut res = self.client.get(&req_url).send()?;

        let cb_device_txt = res.text()?;
        let cb_device_resp: CbAllDevicesStatusResponse = serde_json::from_str(&cb_device_txt)?;

        Ok(cb_device_resp.results)
    }

    pub fn set_device_policy(self, dev_id: i64, policy_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let req_url = format!("{}{}{}", self.url, "integrationServices/v3/device/", dev_id);
        let req_body = serde_json::to_string(&CbDeviceStatusChangeRequest::new(policy_name))?;

        let mut res = self.client.patch(&req_url)
            .body(req_body)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()?;

        dbg!(&res);

        Ok(())
    }
}