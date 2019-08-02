# CB-RS

## Examples
* Get all devices
```rust
use std::env;
use std::fs;
use cb_rs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = env::var("CBAPI")?;
    let url = env::var("CBURL")?;

    let cb_client = cb_rs::client::CbClient::new(&key, &url)?;

    let r = cb_client.get_all_devices()?;

    for x in &r {
        println!("{:?}: {:?}", x.deviceName, x.policyName);
    }

    let monitored: Vec<cb_rs::types::CbDevice> = r.into_iter().filter(|x| x.policyName == Some("Monitored".to_owned())).collect();

    for x in &monitored {
        println!("{}", x.deviceName.as_ref().unwrap_or(&"NO_NAME".to_owned()));
    }
    // let f = fs::read("C:\\users\\1045\\repos\\cb-rs\\single_device_status.json")?;
    // let r: cb::types::CbSingleDeviceStatusResponse = serde_json::from_str(std::str::from_utf8(&f)?)?;

    // dbg!(r);
    Ok(())

}
```

