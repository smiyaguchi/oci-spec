#![allow(non_camel_case_types)]
use super::serdes::{deserialize, serialize, to_string, to_writer, OciSpecError};

use std::collections::HashMap;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mount {
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
    #[serde(rename = "pageSize")]
    pub page_size: String,
    pub limit: i64,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxInterfacePriority {
    pub name: String,
    pub priority: u32,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxWeightDevice {
    pub major: i64,
    pub minor: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "leafWeight")]
    pub leaf_weight: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxThrottleDevice {
    pub major: i64,
    pub minor: i64,
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LinuxDeviceCgroupType {
    b,
    c,
    a,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDeviceCgroup {
    pub allow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub typ: Option<LinuxDeviceCgroupType>,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<LinuxMemory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<LinuxCPU>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pids: Option<LinuxPids>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "blockIO")]
    pub block_io: Option<LinuxBlockIO>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "hugepageLimits")]
    pub hugepage_limits: Option<Vec<LinuxHugepageLimit>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<LinuxNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rdma: Option<HashMap<String, LinuxRdma>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxIDMapping {
    #[serde(rename = "hostID")]
    pub host_id: u32,
    #[serde(rename = "containerID")]
    pub container_id: u32,
    pub size: u32,    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LinuxDeviceType {
    b,
    c,
    u,
    p,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxDevice {
    pub path: String,
    #[serde(rename = "type")]
    pub typ: LinuxDeviceType,
    pub major: i64,
    pub minor: i64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "fileMode")]
    pub file_mode: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uid: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gid: Option<u32>,    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LinuxSeccompAction {
    SCMP_ACT_KILL,
    SCMP_ACT_TRAP,
    SCMP_ACT_ERRNO,
    SCMP_ACT_TRACE,
    SCMP_ACT_ALLOW,    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    pub index: usize,
    pub value: u64,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "valueTwo")]
    pub value_two: Option<u64>,
    pub op: LinuxSeccompOperator,    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
pub enum LinuxCapabilityType {
    CAP_CHOWN,
    CAP_DAC_OVERRIDE,
    CAP_DAC_READ_SEARCH,
    CAP_FOWNER,
    CAP_FSETID,
    CAP_KILL,
    CAP_SETGID,
    CAP_SETUID,
    CAP_SETPCAP,
    CAP_LINUX_IMMUTABLE,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_ADMIN,
    CAP_NET_RAW,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_MODULE,
    CAP_SYS_RAWIO,
    CAP_SYS_CHROOT,
    CAP_SYS_PTRACE,
    CAP_SYS_PACCT,
    CAP_SYS_ADMIN,
    CAP_SYS_BOOT,
    CAP_SYS_NICE,
    CAP_SYS_RESOURCE,
    CAP_SYS_TIME,
    CAP_SYS_TTY_CONFIG,
    CAP_MKNOD,
    CAP_LEASE,
    CAP_AUDIT_WRITE,
    CAP_AUDIT_CONTROL,
    CAP_SETFCAP,
    CAP_MAC_OVERRIDE,
    CAP_MAC_ADMIN,
    CAP_SYSLOG,
    CAP_WAKE_ALARM,
    CAP_BLOCK_SUSPEND,
    CAP_AUDIT_READ,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxCapabilities {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounding: Option<Vec<LinuxCapabilityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective: Option<Vec<LinuxCapabilityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inheritable: Option<Vec<LinuxCapabilityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permitted: Option<Vec<LinuxCapabilityType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ambient: Option<Vec<LinuxCapabilityType>>,
}

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Box {
    pub height: u64,
    pub width: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uid: u32,
    pub gid: u32,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "additionalGids")]
    pub additional_gids: Option<Vec<u32>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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
    #[serde(rename = "type")]
    pub typ: String,
    pub hard: u64,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkname: Option<String>,    
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lowerLink")]
    pub lowerlink: Option<String>,    
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedAddress")]
    pub allowed_address: Option<String>,    
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "configureAllowedAddress")]
    pub configure_allowed_address: Option<String>,    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defrouter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "linkProtection")]
    pub link_protection: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "macAddress")]
    pub mac_address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarisCappedCPU {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ncpus: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SolarisCappedMemory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical: Option<String>,    
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<String>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Solaris {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "limitpriv")]
    pub limit_priv: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxShmMemory")]
    pub max_shm_memory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anet: Option<Vec<SolarisAnet>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cappedCPU")]
    pub capped_cpu: Option<SolarisCappedCPU>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cappedMemory")]
    pub capped_memory: Option<SolarisCappedMemory>,        
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsDevice {
    pub id: String,
    #[serde(rename = "idType")]
    pub id_type: String,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsMemoryResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsCPUResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shares: Option<u16>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub maximum: Option<u16>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsStorageResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iops: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bps: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sandboxSize")]
    pub sandbox_size: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsHyperV {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "utilityVMPath")]
    pub utility_vm_path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsNetwork {
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "endpointList")]
    pub endpoint_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowUnqualifiedDNSQuery")]
    pub allow_unqualified_dns_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "DNSSearchList")]
    pub dns_search_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkSharedContainerName")]
    pub network_shared_container_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "networkNamespace")]
    pub network_namespace: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WindowsResources {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<WindowsMemoryResources>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<WindowsCPUResources>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage: Option<WindowsStorageResources>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Windows {
    #[serde(rename = "layerFolders")]
    pub layer_folders: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<WindowsDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources: Option<WindowsResources>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "credentialSpec")]
    pub credential_spec: Option<HashMap<String, Option<serde_json::Value>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub servicing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "ignoreFlushesDuringBoot")]
    pub ignore_flushes_during_boot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyperv: Option<WindowsHyperV>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<WindowsNetwork>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMHypervisor {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMKernel {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initrd: Option<String>,    
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum VMFormat {
    raw,
    qcow2,
    vdi,
    vmdk,
    vhd,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VMImage {
    pub path: String,
    pub format: VMFormat,    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VM {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hypervisor: Option<VMHypervisor>,
    pub kernel: VMKernel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<VMImage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    #[serde(rename = "ociVersion")]
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")] 
    pub process: Option<Process>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<Root>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mounts: Option<Vec<Mount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linux: Option<Linux>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solaris: Option<Solaris>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub windows: Option<Windows>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm: Option<VM>,
}

impl Spec {
    pub fn load(path: &str) -> Result<Spec, OciSpecError> {
        deserialize(path)    
    }
    pub fn save(&self, path: &str) -> Result<(), OciSpecError> {
        serialize(self, path)    
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    #[serde(rename = "ociVersion")]
    pub version: String,
    pub id: String,
    pub status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<i32>,
    pub bundle: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<HashMap<String, String>>,
}

impl State {
    pub fn to_string(&self) -> Result<String, OciSpecError> {
        to_string(self)    
    }

    pub fn to_writer<W: Write>(&self, mut writer: W) -> Result<(), OciSpecError> {
        to_writer(self, &mut writer)    
    }
}
