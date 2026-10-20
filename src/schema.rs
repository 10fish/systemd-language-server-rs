use std::path::Path;

use tower_lsp::lsp_types::Url;

pub const COMMON_SECTIONS: &[&str] = &["Unit", "Install"];

pub const UNIT_KEYS: &[&str] = &[
    "Description",
    "Documentation",
    "Wants",
    "Requires",
    "Requisite",
    "BindsTo",
    "PartOf",
    "Conflicts",
    "Before",
    "After",
    "OnFailure",
    "OnSuccess",
    "RequiresMountsFor",
    "WantsMountsFor",
    "ConditionPathExists",
    "ConditionPathIsDirectory",
    "ConditionFileNotEmpty",
    "ConditionUser",
    "ConditionGroup",
    "AssertPathExists",
    "DefaultDependencies",
    "StopWhenUnneeded",
    "RefuseManualStart",
    "RefuseManualStop",
    "StartLimitIntervalSec",
    "StartLimitBurst",
    "SourcePath",
];

pub const INSTALL_KEYS: &[&str] = &[
    "Alias",
    "WantedBy",
    "RequiredBy",
    "UpheldBy",
    "Also",
    "DefaultInstance",
];

const SERVICE_KEYS: &[&str] = &[
    "Type",
    "ExecStart",
    "ExecStartPre",
    "ExecStartPost",
    "ExecStop",
    "ExecStopPost",
    "ExecReload",
    "Restart",
    "RestartSec",
    "RemainAfterExit",
    "User",
    "Group",
    "WorkingDirectory",
    "Environment",
    "EnvironmentFile",
    "RuntimeDirectory",
    "StateDirectory",
    "TimeoutStartSec",
    "TimeoutStopSec",
    "KillMode",
    "StandardInput",
    "StandardOutput",
    "StandardError",
];

const SOCKET_KEYS: &[&str] = &[
    "ListenStream",
    "ListenDatagram",
    "ListenSequentialPacket",
    "ListenFIFO",
    "ListenSpecial",
    "Accept",
    "Service",
    "SocketUser",
    "SocketGroup",
    "SocketMode",
    "DirectoryMode",
    "RemoveOnStop",
];

const TIMER_KEYS: &[&str] = &[
    "OnActiveSec",
    "OnBootSec",
    "OnStartupSec",
    "OnUnitActiveSec",
    "OnUnitInactiveSec",
    "OnCalendar",
    "AccuracySec",
    "RandomizedDelaySec",
    "FixedRandomDelay",
    "Unit",
    "Persistent",
    "WakeSystem",
    "RemainAfterElapse",
];

const PATH_KEYS: &[&str] = &[
    "PathExists",
    "PathExistsGlob",
    "PathChanged",
    "PathModified",
    "DirectoryNotEmpty",
    "Unit",
    "MakeDirectory",
    "DirectoryMode",
];

const MOUNT_KEYS: &[&str] = &[
    "What",
    "Where",
    "Type",
    "Options",
    "SloppyOptions",
    "LazyUnmount",
    "ForceUnmount",
    "DirectoryMode",
    "TimeoutSec",
];
const AUTOMOUNT_KEYS: &[&str] = &["Where", "ExtraOptions", "DirectoryMode", "TimeoutIdleSec"];
const SWAP_KEYS: &[&str] = &["What", "Priority", "Options", "TimeoutSec"];
const SLICE_KEYS: &[&str] = &[
    "CPUAccounting",
    "CPUWeight",
    "MemoryAccounting",
    "MemoryMax",
    "TasksAccounting",
    "TasksMax",
];
const SCOPE_KEYS: &[&str] = &[
    "RuntimeMaxSec",
    "RuntimeRandomizedExtraSec",
    "TimeoutStopSec",
    "OOMPolicy",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnitType {
    Service,
    Socket,
    Timer,
    Path,
    Mount,
    Automount,
    Swap,
    Target,
    Slice,
    Scope,
    Device,
    Unknown,
}

impl UnitType {
    pub fn from_uri(uri: &Url) -> Self {
        let Ok(path) = uri.to_file_path() else {
            return Self::Unknown;
        };
        Self::from_path(&path)
    }

    pub fn from_path(path: &Path) -> Self {
        let mut extension = path.extension().and_then(|value| value.to_str());
        if extension == Some("conf") {
            extension = path
                .parent()
                .and_then(Path::file_name)
                .and_then(|value| value.to_str())
                .and_then(|directory| directory.strip_suffix(".d"))
                .and_then(|unit| Path::new(unit).extension())
                .and_then(|value| value.to_str());
        }
        match extension {
            Some("service") => Self::Service,
            Some("socket") => Self::Socket,
            Some("timer") => Self::Timer,
            Some("path") => Self::Path,
            Some("mount") => Self::Mount,
            Some("automount") => Self::Automount,
            Some("swap") => Self::Swap,
            Some("target") => Self::Target,
            Some("slice") => Self::Slice,
            Some("scope") => Self::Scope,
            Some("device") => Self::Device,
            _ => Self::Unknown,
        }
    }

    pub fn specific_section(self) -> Option<&'static str> {
        match self {
            Self::Service => Some("Service"),
            Self::Socket => Some("Socket"),
            Self::Timer => Some("Timer"),
            Self::Path => Some("Path"),
            Self::Mount => Some("Mount"),
            Self::Automount => Some("Automount"),
            Self::Swap => Some("Swap"),
            Self::Slice => Some("Slice"),
            Self::Scope => Some("Scope"),
            Self::Target | Self::Device | Self::Unknown => None,
        }
    }

    pub fn sections(self) -> Vec<&'static str> {
        let mut sections = COMMON_SECTIONS.to_vec();
        if let Some(section) = self.specific_section() {
            sections.insert(1, section);
        }
        sections
    }
}

pub fn is_drop_in(uri: &Url) -> bool {
    let Ok(path) = uri.to_file_path() else {
        return false;
    };
    path.extension().and_then(|value| value.to_str()) == Some("conf")
        && path
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            .is_some_and(|directory| directory.ends_with(".d"))
}

pub fn directive_names(section: &str) -> &'static [&'static str] {
    match section {
        "Unit" => UNIT_KEYS,
        "Install" => INSTALL_KEYS,
        "Service" => SERVICE_KEYS,
        "Socket" => SOCKET_KEYS,
        "Timer" => TIMER_KEYS,
        "Path" => PATH_KEYS,
        "Mount" => MOUNT_KEYS,
        "Automount" => AUTOMOUNT_KEYS,
        "Swap" => SWAP_KEYS,
        "Slice" => SLICE_KEYS,
        "Scope" => SCOPE_KEYS,
        _ => &[],
    }
}

pub fn section_documentation(section: &str) -> Option<&'static str> {
    match section {
        "Unit" => Some("Common unit metadata, dependencies, ordering, and conditions."),
        "Install" => Some("Installation information used by `systemctl enable`."),
        "Service" => Some("Process startup, shutdown, restart, and execution environment."),
        "Socket" => Some("Listening endpoints used for socket activation."),
        "Timer" => Some("Time-based activation of another unit."),
        "Path" => Some("File system path monitoring that activates another unit."),
        "Mount" => Some("File system mount point configuration."),
        "Automount" => Some("On-demand automount point configuration."),
        "Swap" => Some("Swap device or file configuration."),
        "Slice" => Some("Resource-management hierarchy for processes."),
        "Scope" => Some("A group of externally created processes."),
        _ => None,
    }
}

pub fn directive_documentation(key: &str) -> Option<&'static str> {
    match key {
        "Description" => Some("A short description of the unit."),
        "Documentation" => Some("A list of URIs that reference documentation for the unit."),
        "Requires" => Some("Configures strong requirement dependencies on other units."),
        "Wants" => Some("Configures weaker dependencies on other units."),
        "After" => Some("Orders this unit after the listed units during startup."),
        "Before" => Some("Orders this unit before the listed units during startup."),
        "Type" => {
            Some("The service startup type, such as `simple`, `exec`, `oneshot`, or `notify`.")
        }
        "ExecStart" => Some("The command to run when the service starts."),
        "ExecStop" => Some("The command to run when the service stops."),
        "Restart" => Some("The restart policy applied when the service exits."),
        "WantedBy" => Some("Targets that receive a `.wants` link when this unit is enabled."),
        "ListenStream" => Some("An address, port, or file path for a listening stream socket."),
        "OnCalendar" => Some("A calendar event expression."),
        "Unit" => Some("The unit activated by this timer or path unit."),
        "What" => Some("The device, file, or resource to mount or enable as swap."),
        "Where" => Some("The mount point path."),
        _ => None,
    }
}

pub fn value_completions(section: &str, key: &str) -> &'static [&'static str] {
    match (section, key) {
        ("Service", "Type") => &[
            "simple",
            "exec",
            "forking",
            "oneshot",
            "dbus",
            "notify",
            "notify-reload",
            "idle",
        ],
        ("Service", "Restart") => &[
            "no",
            "on-success",
            "on-failure",
            "on-abnormal",
            "on-watchdog",
            "on-abort",
            "always",
        ],
        ("Service", "KillMode") => &["control-group", "mixed", "process", "none"],
        ("Socket", "Accept") | ("Timer", "Persistent") | ("Path", "MakeDirectory") => {
            &["yes", "no"]
        }
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_drop_in_unit_type() {
        let path = Path::new("/tmp/demo.service.d/override.conf");
        assert_eq!(UnitType::from_path(path), UnitType::Service);
        let uri = Url::from_file_path(path).unwrap();
        assert!(is_drop_in(&uri));
    }

    #[test]
    fn unknown_types_have_safe_common_sections() {
        assert_eq!(UnitType::Unknown.sections(), vec!["Unit", "Install"]);
    }
}
