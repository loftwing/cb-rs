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

    let client = cb_rs::client::CbClient::new(&key, &url)?;

    let r = client.get_all_devices()?;

    for x in &r {
        println!("{:?}: {:?}", x.deviceName, x.policyName);
    }

    let monitored: Vec<cb_rs::types::CbDevice> = r.into_iter().filter(|x| x.policyName == Some("Monitored".to_owned())).collect();

    for x in &monitored {
        println!("{}", x.deviceName.as_ref().unwrap_or(&"NO_NAME".to_owned()));
    }
    Ok(())

}
```

* Get all device's status
```rust
let r = client.get_all_devices_status()?;
```

* Get single device's status
```rust
let r = client.get_device_status(1234)?;
```

* Get device status by searching
```rust
let search_params = &[("ipAddress", "1.2.3.4"), ("hostName", "computer")]?;
let r = client.get_device_status_search(search_params)?;
```

* Set device policy
```rust
let r = client.set_device_policy(1234, "Scorched Earth")?;
```


