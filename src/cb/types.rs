#![allow(non_snake_case, dead_code)]
use serde::{Deserialize, Serialize};

//
// RESPONSES
//

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
pub struct CbPaginatedResponse<T> {
    pub latestTime: usize,
    pub success: bool,
    pub message: String,
    pub elapsed: usize,
    pub totalResults: usize,
    pub results: Vec<T>,
}

#[derive(Deserialize, Debug)]
pub struct CbSingleDeviceStatusResponse {
    pub success: bool,
    pub message: String,
    pub deviceInfo: Option<CbDeviceStatus>,
}

//TODO: get rid of all these options i have no idea what im doing
#[derive(Deserialize, Serialize, Debug)]
pub struct CbDeviceStatus {
    pub adGroupId: Option<usize>,
    pub policyOverride: Option<bool>,
    pub currentSensorPolicyName: Option<String>,
    pub deviceMetaDataItemList: Option<Vec<String>>,
    pub lastDevicePolicyRequestedTime: Option<usize>, //i think this will be usize?
    pub lastDevicePolicyChangedTime: Option<usize>,
    pub lastPolicyUpdatedTime: Option<usize>,
    pub loginUserName: Option<String>,
    pub createTime: Option<i64>,
    pub lastReportedTime: Option<i64>,
    pub uninstallCode: Option<String>,
    pub organizationId: Option<usize>,
    pub deviceId: usize,
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
    pub avStatus: Option<Vec<String>>,
    pub deregisteredTime: Option<i64>,
    pub sensorStates: Option<Vec<String>>,
    pub messages: Option<Vec<String>>,
    pub rootedBySensor: Option<bool>,
    pub rootedBySensorTime: Option<i64>,
    pub quarantined: Option<bool>,
    pub lastInternalIpAddress: Option<String>,
    pub macAddress: Option<String>,
    pub lastExternalIpAddress: Option<String>,
    pub lastLocation: Option<String>,
    pub sensorOutOfDate: Option<bool>,
    pub avUpdateServers: Option<Vec<String>>,
    pub passiveMode: Option<bool>,
    pub lastResetTime: Option<i64>,
    pub lastShutdownTime: Option<i64>,
    pub scanStatus: Option<String>,
    pub scanLastActionTime: Option<i64>,
    pub scanLastCompleteTime: Option<i64>,
    pub linuxKernelVersion: Option<String>,
    pub avEngine: Option<String>,
    pub avProductVersion: Option<String>,
    pub avAveVersion: Option<String>,
    pub avPackVersion: Option<String>,
    pub avVdfVersion: Option<String>,
    pub avLastScanTime: Option<i64>,
    pub virtualMachine: Option<bool>,
    pub virtualizationProvider: Option<String>,
    pub rootedByAnalytics: Option<bool>,
    pub rootedByAnalyticsTime: Option<i64>,
    pub avMaster: Option<bool>,
    pub firstVirusActivityTime: Option<i64>,
    pub lastVirusActivityTime: Option<i64>,
    pub testId: Option<isize>,
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

