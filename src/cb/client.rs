#![allow(dead_code)]
use reqwest::{Client, header};
use log::{info, debug};
use serde::de::DeserializeOwned;
use std::fmt::Debug;
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
        let res = self.make_paginated_request(&req_url)?;
        debug!("Get all devices response size: {}", &res.len());
        Ok(res)
    }

    //TODO make this recursive?
    //take a request and handle the pagination (if any)
    //function is generic over T which is the type of results we expect back
    pub fn make_paginated_request<T>(self, req_url: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
        where T: DeserializeOwned + Debug {

        let orig_url = req_url.to_owned();
        let mut orig_res = self.client.get(&orig_url).send()?;
        debug!("PAGINATED RESPONSE: {:?}", &orig_res);

        //extract text for parsing
        let req_body_txt = orig_res.text()?;

        //deserialize to CBPaginatedResponse generic over T
        let mut cb_device_resp: CbPaginatedResponse<T> = serde_json::from_str(&req_body_txt)?;

        // array to store results of type T we get back
        let mut results: Vec<T> = Vec::new();

        results.append(&mut cb_device_resp.results);

        //check if more results to get
        if cb_device_resp.totalResults > results.len() {
            loop {
                debug!("PAGINATION: start - {} | rows - 100", results.len() + 1);

                let mut cur_res = self.client.get(&orig_url)
                    .query(&[("start", results.len() + 1), ("rows", 100)])
                    .send()?;

                let res_body_txt = cur_res.text()?;
                let mut pagination_resp: CbPaginatedResponse<T> = serde_json::from_str(&res_body_txt)?;

                results.append( &mut pagination_resp.results);
                if !(results.len() < cb_device_resp.totalResults) {
                    break;
                }
            }
        }

        Ok(results)
    }

    pub fn get_all_devices_status(self) -> Result<Vec<CbDeviceStatus>, Box<dyn std::error::Error>> {
        let req_url = format!("{}{}", self.url, "integrationServices/v3/device");
        let res = self.make_paginated_request(&req_url)?;
        info!("Got all devices status: {}", &res.len());
        Ok(res)
    }

    pub fn set_device_policy(self, dev_id: i64, policy_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let req_url = format!("{}{}{}", self.url, "integrationServices/v3/device/", dev_id);
        let req_body = serde_json::to_string(&CbDeviceStatusChangeRequest::new(policy_name))?;

        let res = self.client.patch(&req_url)
            .body(req_body)
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send()?;

        dbg!(&res);

        Ok(())
    }
}