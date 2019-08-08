use cb_rs::cb::client::CbClient;
use std::env;

fn setup() -> CbClient {
    let key = String::from("AAAA/AAAAAAAAAAAAAAAA BEES");
    let url = env::var("TESTURL").unwrap();
    CbClient::new(&key, &url).unwrap()
}

#[test]
fn get_all_device_status() {
    let client = setup();

    let r = client.get_all_devices_status().unwrap();

    assert_eq!(r.len(), 10);
}

#[test]
fn get_all_devices() {
    let client = setup();

    let r = client.get_all_devices().unwrap();

    assert_eq!(r.len(), 8);
}

#[test]
fn get_device_status() {
    let client = setup();

    let r = client.get_device_status(5707542).unwrap().unwrap();

    assert_eq!(r.organizationId, Some(2));
}

#[test]
fn set_device_policy() {
    let client = setup();

    let r = client.set_device_policy(4211, "Restrictive_Windows_Workstation").unwrap().unwrap();

    assert_eq!(r.policyName, Some("Restrictive_Windows_Workstation".to_owned()));
}

#[test]
fn get_device_status_search() {
    let client = setup();

    let search_params = &[("ipAddress", "1.2.3.4")];

    let r = client.get_device_status_search(search_params).unwrap();

    assert_eq!(r[0].deviceId, 218616);
}