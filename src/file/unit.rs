use super::Systemd;

/// common sections for all systemd configuration files

pub struct UnitFile;

impl UnitFile {
    const SECTIONS: &[&str] = &["Unit", "Install"];

    const SECTION_UNIT: &[&str] = &[
        "Description",
        "Documentation",
        "Wants",
        "Requires",
        "Requisite",
        "BindsTo",
        "PartOf",
        "Upholds",
        "Conflicts",
        "Before",
        "After",
        "OnFailure",
        "OnSuccess",
        "PropagatesReloadTo",
        "ReloadPropagatedFrom",
        "PropagatesStopTo",
        "StopPropagatedFrom",
        "JoinsNamespaceOf",
        "RequiresMountsFor",
        "WantsMountsFor",
        "OnSuccessJobMode",
        "OnFailureJobMode",
        "IgnoreOnIsolate",
        "StopWhenUnneeded",
        "RefuseManualStart",
        "RefuseManualStop",
        "AllowIsolate",
        "DefaultDependencies",
        "SurviveFinalKillSignal",
        "CollectMode",
        "FailureAction",
        "SuccessAction",
        "FailureActionExitStatus",
        "SuccessActionExitStatus",
        "JobTimeoutSec",
        "JobRunningTimeoutSec",
        "JobTimeoutAction",
        "JobTimeoutRebootArgument",
        "StartLimitIntervalSec",
        "intervalStartLimitBurst",
        "burstStartLimitAction",
        "RebootArgument",
        "SourcePath",
        "ConditionArchitecture",
        "ConditionFirmware",
        "ConditionVirtualization",
        "ConditionHost",
        "ConditionKernelCommandLine",
        "ConditionKernelVersion",
        "ConditionCredential",
        "ConditionEnvironment",
        "ConditionSecurity",
        "ConditionCapability",
        "ConditionACPower",
        "ConditionNeedsUpdate",
        "ConditionFirstBoot",
        "ConditionPathExists",
        "ConditionPathExistsGlob",
        "ConditionPathIsDirectory",
        "ConditionPathIsSymbolicLink",
        "ConditionPathIsMountPoint",
        "ConditionPathIsReadWrite",
        "ConditionPathIsEncrypted",
        "ConditionDirectoryNotEmpty",
        "ConditionFileNotEmpty",
        "ConditionFileIsExecutable",
        "ConditionUser",
        "ConditionGroup",
        "ConditionControlGroupController",
        "ConditionMemory",
        "ConditionCPUs",
        "ConditionCPUFeature",
        "ConditionOSRelease",
        "ConditionMemoryPressure",
        "ConditionCPUPressure",
        "ConditionIOPressure",
        "AssertArchitecture",
        "AssertVirtualization",
        "AssertHost",
        "AssertKernelCommandLine",
        "AssertKernelVersion",
        "AssertCredential",
        "AssertEnvironment",
        "AssertSecurity",
        "AssertCapability",
        "AssertACPower",
        "AssertNeedsUpdate",
        "AssertFirstBoot",
        "AssertPathExists",
        "AssertPathExistsGlob",
        "AssertPathIsDirectory",
        "AssertPathIsSymbolicLink",
        "AssertPathIsMountPoint",
        "AssertPathIsReadWrite",
        "AssertPathIsEncrypted",
        "AssertDirectoryNotEmpty",
        "AssertFileNotEmpty",
        "AssertFileIsExecutable",
        "AssertUser",
        "AssertGroup",
        "AssertControlGroupController",
        "AssertMemory",
        "AssertCPUs",
        "AssertCPUFeature",
        "AssertOSRelease",
        "AssertMemoryPressure",
        "AssertCPUPressure",
        "AssertIOPressure",
    ];

    const SECTION_INSTALL: &[&str] = &[
        "Alias",
        "WantedBy",
        "RequiredBy",
        "UpheldBy",
        "Also",
        "DefaultInstance",
    ];
}

impl Systemd for UnitFile {
    fn extention(&self) -> &'static str {
        "unit"
    }

    fn required(&self, sytax: super::Sytax) -> &[&str] {
        match sytax {
            super::Sytax::SectionName => Self::SECTIONS,
            super::Sytax::DirectiveKey(section) => match section {
                "Unit" => Self::SECTION_UNIT,
                "Install" => Self::SECTION_INSTALL,
                unknown => &[],
            },
            super::Sytax::DirectiveValue(section, key) => {
                // not supported yet
                &[]
            }
        }
    }

    fn permits(&self, sytax: super::Sytax) -> &[&str] {
        match sytax {
            super::Sytax::SectionName => Self::SECTIONS,
            // TODO: required keys
            super::Sytax::DirectiveKey(section) => match section {
                "Unit" => Self::SECTION_UNIT,
                "Install" => Self::SECTION_INSTALL,
                unknown => &[],
            },
            super::Sytax::DirectiveValue(section, key) => {
                // not supported yet
                &[]
            }
        }
    }
}
