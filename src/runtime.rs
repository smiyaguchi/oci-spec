#![allow(non_camel_case_types)]
use super::serdes::{serialize, deserialize, SerDesError};

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    #[serde(default)]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
    #[serde(default)]
    pub destination: String,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub typ: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hook {
    #[serde(default)]
    pub path: String,    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Hooks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prestart: Option<Vec<Hook>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poststart: Option<Vec<Hook>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poststop: Option<Vec<Hook>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxMemory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swap: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "kernelTCP")]
    pub kernel_tcp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableOOMKiller")]
    pub disable_oom_killer: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCPU {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shares: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quota: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimeRuntime")]
    pub realtime_runtime: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "realtimePeriod")]
    pub realtime_period: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpus: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mems: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxPids {
    #[serde(default)]
    pub limit: i64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxBlockIO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "leafWeight")]
    pub leaf_weight: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "weightDevice")]
    pub weight_device: Option<Vec<LinuxWeightDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttleReadBpsDevice")]
    pub throttle_read_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttleWriteBpsDevice")]
    pub throttle_write_bps_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttleReadIOPSDevice")]
    pub throttle_read_iops_device: Option<Vec<LinuxThrottleDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "throttleWriteIOPSDevice")]
    pub throttle_write_iops_device: Option<Vec<LinuxThrottleDevice>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxHugepageLimit {
    #[serde(default, rename = "pageSize")]
    pub page_size: String,
    #[serde(default)]
    pub limit: i64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxInterfacePriority {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub priority: u32,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxWeightDevice {
    #[serde(default)]
    pub major: i64,
    #[serde(default)]
    pub minor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "leafWeight")]
    pub leaf_weight: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxThrottleDevice {
    #[serde(default)]
    pub major: i64,
    #[serde(default)]
    pub minor: i64,
    #[serde(default)]
    pub rate: u64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNetwork {
    #[serde(skip_serializing_if = "Option::is_none", rename = "classID")]
    pub class_id: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priorities: Option<Vec<LinuxInterfacePriority>>, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxRdma {
    #[serde(skip_serializing_if = "Option::is_none", rename = "hcaHandles")]
    pub hca_handles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hcaObjects")]
    pub hca_objects: Option<u32>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxDeviceType {
    b,
    c,
    u,
    p,
    a,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDeviceCgroup {
    #[serde(default)]
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub typ: Option<LinuxDeviceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDeviceCgroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<LinuxMemory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LinuxCPU>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<LinuxPids>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "blockIO")]
    pub block_io: Option<LinuxBlockIO>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<LinuxNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdma: Option<HashMap<String, LinuxRdma>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIDMapping {
    #[serde(default, rename = "hostID")]
    pub host_id: u32,
    #[serde(default, rename = "containerID")]
    pub container_id: u32,
    #[serde(default)]
    pub size: u32,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDevice {
    #[serde(default)]
    pub path: String,
    #[serde(rename = "type")]
    pub typ: LinuxDeviceType,
    #[serde(default)]
    pub major: i64,
    #[serde(default)]
    pub minor: i64,
    #[serde(skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxSeccompAction {
    SCMP_ACT_KILL,
    SCMP_ACT_TRAP,
    SCMP_ACT_ERRNO,
    SCMP_ACT_TRACE,
    SCMP_ACT_ALLOW,    
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxSeccompOperator {
    SCMP_CMP_NE,
    SCMP_CMP_LT,
    SCMP_CMP_LE,
    SCMP_CMP_EQ,
    SCMP_CMP_GE,
    SCMP_CMP_GT,
    SCMP_CMP_MASKED_EQ,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSeccompArg {
    #[serde(default)]
    pub index: usize,
    #[serde(default)]
    pub value: u64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueTwo")]
    pub value_two: Option<u64>,
    pub op: LinuxSeccompOperator,    
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Arch {
    SCMP_ARCH_NATIVE,
    SCMP_ARCH_X86,
    SCMP_ARCH_X86_64,
    SCMP_ARCH_X32,
    SCMP_ARCH_ARM,
    SCMP_ARCH_AARCH64,
    SCMP_ARCH_MIPS,
    SCMP_ARCH_MIPS64,
    SCMP_ARCH_MIPS64N32,
    SCMP_ARCH_MIPSEL,
    SCMP_ARCH_MIPSEL64,
    SCMP_ARCH_MIPSEL64N32,
    SCMP_ARCH_PPC,
    SCMP_ARCH_PPC64,
    SCMP_ARCH_PPC64LE,
    SCMP_ARCH_S390,
    SCMP_ARCH_S390X,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSyscall {
    #[serde(default)]
    pub names: Vec<String>,
    pub actions: LinuxSeccompAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<LinuxSeccompArg>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxSeccomp {
    #[serde(rename = "defaultAction")]
    pub default_action: LinuxSeccompAction,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<Arch>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syscalls: Option<Vec<LinuxSyscall>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIntelRdt {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "closID")]
    pub clos_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "l3CacheSchema")]
    pub l3_cache_schema: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "memBwSchema")]
    pub mem_bw_schema: Option<String>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "uidMappings")]
    pub uid_mappings: Option<Vec<LinuxIDMapping>>,    
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "gidMappings")]
    pub gid_mappings: Option<Vec<LinuxIDMapping>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sysctl: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<LinuxResources>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cgroupsPath")]
    pub cgroups_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<Vec<LinuxNamespace>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<LinuxDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seccomp: Option<LinuxSeccomp>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "rootfsPropagation")]
    pub rootfs_propagation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maskedPaths")]
    pub masked_paths: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "readonlyPaths")]
    pub readonly_paths: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "mountLabel")]
    pub mount_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "intelRdt")]
    pub intel_rdt: Option<LinuxIntelRdt>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<String>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Box {
    #[serde(default)]
    pub height: u64,
    #[serde(default)]
    pub width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    #[serde(default)]
    pub uid: u32,
    #[serde(default)]
    pub gid: u32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalGids")]
    pub additional_gids: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LinuxNamespaceType {
    pid,
    network,
    mount,
    ipc, 
    uts,
    user,
    cgroup,    
}
#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    pub typ: LinuxNamespaceType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct POSIXRlimit {
    #[serde(default, rename = "type")]
    pub typ: String,
    #[serde(default)]
    pub hard: u64,
    #[serde(default)]
    pub soft: u64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "consoleSize")]
    pub console_size: Option<Box>,
    pub user: User,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<String>>,
    #[serde(default)]
    pub cwd: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<LinuxCapabilities>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rlimits: Option<Vec<POSIXRlimit>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "noNewPrivileges")]
    pub no_new_privileges: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "apparmorProfile")]
    pub apparmor_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "oomScoreAdj")]
    pub oom_score_adj: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "selinuxLabel")]
    pub selinux_label: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarisAnet {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub linkname: String,    
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "lowerLink")]
    pub lowerlink: String,    
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "allowedAddress")]
    pub allowed_address: String,    
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "configureAllowedAddress")]
    pub configure_allowed_address: String,    
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub defrouter: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "linkProtection")]
    pub link_protection: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "macAddress")]
    pub mac_address: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarisCappedCPU {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub ncpus: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarisCappedMemory {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub physical: String,    
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub swap: String,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solaris {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub milestone: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "limitpriv")]
    pub limit_priv: String,
    #[serde(default, skip_serializing_if = "String::is_empty", rename = "maxShmMemory")]
    pub max_shm_memory: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anet: Vec<SolarisAnet>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cappedCPU")]
    pub capped_cpu: Option<SolarisCappedCPU>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cappedMemory")]
    pub capped_memory: Option<SolarisCappedMemory>,        
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    #[serde(default, rename = "ociVersion")]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")] 
    pub process: Option<Process>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<Linux>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solaris: Option<Solaris>,
}

impl Spec {
    pub fn load(path: &str) -> Result<Spec, SerDesError> {
        deserialize(path)    
    }
    pub fn save(&self, path: &str) -> Result<(), SerDesError> {
        serialize(self, path)    
    }
}
