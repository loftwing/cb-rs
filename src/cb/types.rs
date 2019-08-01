#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};

//
// RESPONSES
// Mostly for deserialization.

#[derive(Deserialize, Debug)]
pub struct CbDevicesResponse {
    pub latestTime: usize,
    pub success: bool,
    pub message: String,
    pub elapsed: usize,
    pub totalResults: usize,
    pub results: Vec<CbDevice>,
}

#[derive(Deserialize, Debug)]
pub struct CbDevice {
    pub quarantined: Option<String>,
    pub policyName: Option<String>,
    pub deviceName: Option<String>,
    pub emailAddress: Option<String>,
    pub deregistered: Option<String>,
    pub sensorVersion: Option<String>,
    pub wrapperAsCsvString: Option<String>,
    pub os: Option<String>,
    pub outofDate: Option<String>,
    pub lastCheckInDate: Option<String>,
    pub lastCheckInTime: Option<String>,
    pub deregisteredDate: Option<String>,
    pub bypassed: Option<String>,
    pub quarantinedDate: Option<String>,
    pub bypassedDate: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CbAllDevicesStatusResponse {
    pub latestTime: usize,
    pub success: bool,
    pub message: String,
    pub elapsed: usize,
    pub totalResults: usize,
    pub results: Vec<CbDeviceStatus>,
}

#[derive(Deserialize, Debug)]
pub struct CbSingleDeviceStatusResponse {
    pub success: bool,
    pub message: String,
    pub deviceInfo: Option<CbDeviceStatus>,
}

#[derive(Deserialize, Debug)]
pub struct CbDeviceStatus {
    pub adGroupId: usize,
    pub policyOverride: bool,
    pub currentSensorPolicyName: Option<String>,
    pub deviceMetaDataItemList: Option<Vec<String>>,
    pub lastDevicePolicyRequestedTime: Option<usize>, //i think this will be usize?
    pub lastDevicePolicyChangedTime: Option<usize>,
    pub lastPolicyUpdatedTime: Option<usize>,
    pub loginUserName: Option<String>,
    pub createTime: Option<i64>,
    pub lastReportedTime: Option<i64>,
    pub uninstallCode: Option<String>,
    pub organizationId: usize,
    pub deviceId: Option<usize>,
    pub deviceOwnerId: Option<usize>,
    pub deviceGuid: Option<String>,
    pub email: Option<String>,
    pub deviceType: Option<String>,
    pub deviceSessionId: Option<usize>,
    pub assignedToId: Option<usize>,
    pub assignedToName: Option<String>,
    pub firstName: Option<String>,
    pub lastName: Option<String>,
    pub middleName: Option<String>,
    pub activationCode: Option<String>,
    pub targetPriorityType: Option<String>,
    pub organizationName: Option<String>,
    pub osVersion: Option<String>,
    pub sensorVersion: Option<String>,
    pub activationCodeExpiryTime: Option<i64>,
    pub registeredTime: Option<i64>,
    pub lastContact: Option<i64>,
    pub windowsPlatform: Option<String>,
    pub vdiBaseDevice: Option<i64>,
    pub avStatus: Vec<String>,
    pub deregisteredTime: Option<i64>,
    pub sensorStates: Vec<String>,
    pub messages: Option<Vec<String>>,
    pub rootedBySensor: bool,
    pub rootedBySensorTime: Option<i64>,
    pub quarantined: bool,
    pub lastInternalIpAddress: Option<String>,
    pub macAddress: String,
    pub lastExternalIpAddress: Option<String>,
    pub lastLocation: Option<String>,
    pub sensorOutOfDate: bool,
    pub avUpdateServers: Option<Vec<String>>,
    pub passiveMode: bool,
    pub lastResetTime: Option<i64>,
    pub lastShutdownTime: Option<i64>,
    pub scanStatus: Option<String>,
    pub scanLastActionTime: i64,
    pub scanLastCompleteTime: i64,
    pub linuxKernelVersion: Option<String>,
    pub avEngine: Option<String>,
    pub avProductVersion: Option<String>,
    pub avAveVersion: Option<String>,
    pub avPackVersion: Option<String>,
    pub avVdfVersion: Option<String>,
    pub avLastScanTime: i64,
    pub virtualMachine: bool,
    pub virtualizationProvider: String,
    pub rootedByAnalytics: bool,
    pub rootedByAnalyticsTime: Option<i64>,
    pub avMaster: bool,
    pub firstVirusActivityTime: i64,
    pub lastVirusActivityTime: i64,
    pub testId: isize,
    pub uninstalledTime: Option<i64>,
    pub encodedActivationCode: Option<String>,
    pub originEventHash: Option<String>,
    pub status: Option<String>,
    pub name: Option<String>,
    pub policyId: Option<usize>,
    pub policyName: Option<String>,
}


//
//  REQUESTS
//

#[derive(Serialize, Debug)]
pub struct CbDeviceStatusChangeRequest {
    pub policyName: String,
}

impl CbDeviceStatusChangeRequest {
    pub fn new(policy_name: &str) -> CbDeviceStatusChangeRequest {
        CbDeviceStatusChangeRequest {
            policyName: policy_name.to_owned(),
        }
    }
}

