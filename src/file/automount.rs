use super::Systemd;

pub struct AutomountFile;

impl AutomountFile {
    const SECTIONS: &[&str] = &["Automount"];

    const SECTION_AUTOMOUNT: &[&str] =
        &["Where", "ExtraOptions", "DirectoryMode", "TimeoutIdleSec"];
}

impl Systemd for AutomountFile {
    fn extention(&self) -> &'static str {
        "automount"
    }

    fn required(&self, sytax: super::Sytax) -> &[&str] {
        match sytax {
            super::Sytax::SectionName => Self::SECTIONS,
            super::Sytax::DirectiveKey(section) => {
                if section == "Automount" {
                    Self::SECTION_AUTOMOUNT
                } else {
                    super::unit::UnitFile {}.required(sytax)
                }
            }
            _ => &[],
        }
    }

    fn permits(&self, sytax: super::Sytax) -> &[&str] {
        match sytax {
            super::Sytax::SectionName => Self::SECTIONS,
            super::Sytax::DirectiveKey(section) => {
                if section == "Automount" {
                    Self::SECTION_AUTOMOUNT
                } else {
                    super::unit::UnitFile {}.permits(sytax)
                }
            }
            _ => &[],
        }
    }
}
