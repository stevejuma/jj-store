use jj_store_capture::profile::CaptureProfile;

#[derive(Clone, Copy)]
pub(super) enum Availability {
    Unchecked,
    Available,
    Unavailable,
}
#[derive(Clone, Copy)]
pub(crate) struct ProfileRoots {
    pub profile: CaptureProfile,
    pub roots: usize,
    pub unchecked: usize,
    pub available: usize,
    pub unavailable: usize,
}
#[derive(Clone, Copy, Default)]
pub(crate) struct RootObservations {
    pub roots: usize,
    pub unchecked: usize,
    pub available: usize,
    pub unavailable: usize,
    pub rescan_pending: bool,
    pub harnesses: [Option<ProfileRoots>; 2],
}
impl RootObservations {
    pub fn configured(profiles: impl Iterator<Item = CaptureProfile>) -> Self {
        let mut result = Self::default();
        for profile in profiles {
            result.observe(profile, Availability::Unchecked, false);
        }
        result
    }
    pub(super) fn observe(
        &mut self,
        profile: CaptureProfile,
        availability: Availability,
        rescan: bool,
    ) {
        // Preparation admits only these two Codex surfaces before selecting roots.
        let index = match profile {
            CaptureProfile::CodexCliJsonlV1 => 0,
            CaptureProfile::CodexDesktopJsonlV1 => 1,
            _ => unreachable!("Codex profile was qualified before root selection"),
        };
        let group = self.harnesses[index].get_or_insert(ProfileRoots {
            profile,
            roots: 0,
            unchecked: 0,
            available: 0,
            unavailable: 0,
        });
        group.roots += 1;
        self.roots += 1;
        self.rescan_pending |= rescan;
        match availability {
            Availability::Unchecked => {
                group.unchecked += 1;
                self.unchecked += 1;
            }
            Availability::Available => {
                group.available += 1;
                self.available += 1;
            }
            Availability::Unavailable => {
                group.unavailable += 1;
                self.unavailable += 1;
            }
        }
    }
}
