use super::*;
use rustix::fs::{self, RenameFlags};
use std::fs::File;
const NAMES: [&str; 2] = ["tree_state", "checkout"];
pub(super) struct Destination {
    path: PathBuf,
    working: File,
    name: String,
    private: File,
}
impl Destination {
    pub fn open(
        recovery: &RecoveredHistory,
        plan: &TreeCheckoutPlanV1,
        create: bool,
    ) -> Result<Self> {
        let path = recovery.root.join(".jj/working_copy");
        let working = physical::directory(&path)?;
        let name = format!(".lapis-stack-native-{}", plan.id()?);
        let private = match records::destination::optional(&working, &name)? {
            Some(dir) => dir,
            None if create => records::destination::create(&working, &name)?,
            None => return Err(Error::NotFound),
        };
        Ok(Self {
            path,
            working,
            name,
            private,
        })
    }
    pub fn load(&self, budget: &Budget) -> Result<Option<State>> {
        physical::read(&self.private, "progress", 65536, true, budget)?
            .map(|data| {
                let state: State =
                    serde_json::from_slice(&data.bytes).map_err(|_| Error::Corrupt)?;
                if state.encode()? != data.bytes {
                    return Err(Error::Corrupt);
                }
                Ok(state)
            })
            .transpose()
    }
    pub fn prepare(
        &self,
        plan: &TreeCheckoutPlanV1,
        inventory: &PhysicalInventoryV1,
        physical: m::RecordDigest,
        bytes: &[Vec<u8>; 2],
        budget: &Budget,
    ) -> Result<State> {
        if let Some(state) = self.load(budget)? {
            if state.plan != plan.id()?
                || state.inventory != inventory.id()?
                || state.physical != physical
            {
                return Err(Error::EpochChanged);
            }
            check_frame(&self.private, &state, budget)?;
            self.selections(&state, plan, bytes, budget)?;
            return Ok(state);
        }
        for name in files::entries(&self.private, 4)? {
            if !["tree_state", "checkout", "next"].contains(&name.as_str()) {
                return Err(Error::EpochChanged);
            }
        }
        let stamp = files::directory_stamp(&self.private)?;
        if stamp[3] & 0o7777 != 0o700 {
            return Err(Error::EpochChanged);
        }
        let tree = files::prepare_mode(&self.private, "tree_state", &bytes[0], 0o600, budget)?;
        let checkout = files::prepare_mode(&self.private, "checkout", &bytes[1], 0o600, budget)?;
        let state = State {
            version: 1,
            plan: plan.id()?,
            inventory: inventory.id()?,
            physical,
            private: [stamp[0], stamp[1]],
            tree,
            checkout,
        };
        state.encode()?;
        self.selections(&state, plan, bytes, budget)?;
        Ok(state)
    }
    pub fn save(&self, state: &State, budget: &Budget) -> Result<()> {
        if self.load(budget)?.is_some() {
            check_frame(&self.private, state, budget)?;
            records::synced::acknowledge(&self.private, "progress", budget)?;
            return Ok(());
        }
        let snapshot = files::prepare_mode(&self.private, "next", &state.encode()?, 0o600, budget)?;
        let file = files::sync_file(&self.private, "next", &snapshot)?;
        budget.check()?;
        fs::renameat_with(
            &self.private,
            "next",
            &self.private,
            "progress",
            RenameFlags::NOREPLACE,
        )
        .map_err(|_| Error::EpochChanged)?;
        file.sync_all().map_err(|_| Error::Unavailable)?;
        self.private.sync_all().map_err(|_| Error::Unavailable)?;
        check_frame(&self.private, state, budget)
    }
    pub fn check(&self, recovery: &RecoveredHistory, state: &State, budget: &Budget) -> Result<()> {
        records::destination::same(&self.working, &physical::directory(&self.path)?)?;
        records::destination::same(&self.private, &physical::child(&self.working, &self.name)?)?;
        check_frame(&self.private, state, budget)?;
        recovery.guard.check(&recovery.root)
    }
    pub fn selections(
        &self,
        state: &State,
        plan: &TreeCheckoutPlanV1,
        bytes: &[Vec<u8>; 2],
        budget: &Budget,
    ) -> Result<[bool; 2]> {
        selections(&self.working, &self.private, state, plan, bytes, budget)
    }
    pub fn select(
        &self,
        index: usize,
        state: &State,
        plan: &TreeCheckoutPlanV1,
        bytes: &[Vec<u8>; 2],
        budget: &Budget,
    ) -> Result<()> {
        // Re-acknowledge an observed frame after unknown directory-sync outcome
        // before selecting either native file. Mere visibility is not durability.
        check_frame(&self.private, state, budget)?;
        records::synced::acknowledge(&self.private, "progress", budget)?;
        self.private.sync_all().map_err(|_| Error::Unavailable)?;
        let selected = self.selections(state, plan, bytes, budget)?;
        if index == 1 && !selected[0] {
            return Err(Error::EpochChanged);
        }
        if !selected[index] {
            let file = files::sync_file(
                &self.private,
                NAMES[index],
                if index == 0 {
                    &state.tree
                } else {
                    &state.checkout
                },
            )?;
            budget.check()?;
            // Legitimate metadata writers hold the foreground lease. This is
            // exact native atomic replacement, not arbitrary-actor filesystem CAS.
            self.selections(state, plan, bytes, budget)?;
            fs::renameat(&self.private, NAMES[index], &self.working, NAMES[index])
                .map_err(|_| Error::Unavailable)?;
            file.sync_all().map_err(|_| Error::Unavailable)?;
            self.private.sync_all().map_err(|_| Error::Unavailable)?;
            self.working.sync_all().map_err(|_| Error::Unavailable)?;
        }
        records::synced::acknowledge(&self.working, NAMES[index], budget)?;
        self.working.sync_all().map_err(|_| Error::Unavailable)?;
        check_frame(&self.private, state, budget)
    }
}
pub(super) fn check_frame(private: &File, state: &State, budget: &Budget) -> Result<()> {
    let stamp = files::directory_stamp(private)?;
    if stamp[..2] != state.private || stamp[3] & 0o7777 != 0o700 {
        return Err(Error::EpochChanged);
    }
    let (snapshot, data) = files::read(private, "progress", budget)?.ok_or(Error::Incomplete)?;
    if data.bytes != state.encode()? || snapshot.stamp[2] & 0o7777 != 0o600 {
        return Err(Error::Corrupt);
    }
    for name in files::entries(private, 4)? {
        if !["tree_state", "checkout", "progress"].contains(&name.as_str()) {
            return Err(Error::Corrupt);
        }
    }
    Ok(())
}
pub(super) fn selections(
    working: &File,
    private: &File,
    state: &State,
    plan: &TreeCheckoutPlanV1,
    bytes: &[Vec<u8>; 2],
    budget: &Budget,
) -> Result<[bool; 2]> {
    if state.plan != plan.id()? {
        return Err(Error::Corrupt);
    }
    let mut selected = [false; 2];
    for (i, expected) in [&state.tree, &state.checkout].into_iter().enumerate() {
        let target = files::read(working, NAMES[i], budget)?.ok_or(Error::Incomplete)?;
        let staged = files::read(private, NAMES[i], budget)?;
        if expected.stamp[0] != files::directory_stamp(working)?[0]
            || expected.stamp[2] & 0o7777 != 0o600
        {
            return Err(Error::Corrupt);
        }
        let (original, fingerprint) = if i == 0 {
            (&plan.tree_state, plan.tree_fingerprint)
        } else {
            (&plan.checkout, plan.checkout_fingerprint)
        };
        if target.1.bytes == *original && target.1.fingerprint == fingerprint {
            let staged = staged.ok_or(Error::EpochChanged)?;
            if staged.1.bytes != bytes[i] || !expected.matches(&staged.0, false) {
                return Err(Error::EpochChanged);
            }
        } else if target.1.bytes == bytes[i]
            && expected.matches(&target.0, true)
            && staged.is_none()
        {
            selected[i] = true;
        } else {
            return Err(Error::EpochChanged);
        }
    }
    if selected == [false, true] {
        return Err(Error::EpochChanged);
    }
    Ok(selected)
}
