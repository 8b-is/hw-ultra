use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum GovernorPolicy {
    Performance,
    Powersave,
    OnDemand,
    Conservative,
    Schedutil,
    Unknown,
}

static POLICY: AtomicU8 = AtomicU8::new(0xFF);

pub struct Governor;

impl Governor {
    pub fn set_policy(p: GovernorPolicy) {
        let val = match p {
            GovernorPolicy::Performance => 0,
            GovernorPolicy::Powersave => 1,
            GovernorPolicy::OnDemand => 2,
            GovernorPolicy::Conservative => 3,
            GovernorPolicy::Schedutil => 4,
            GovernorPolicy::Unknown => 5,
        };
        POLICY.store(val, Ordering::Release);
    }

    pub fn get_policy() -> GovernorPolicy {
        let cached = POLICY.load(Ordering::Acquire);
        if cached != 0xFF {
            return match cached {
                0 => GovernorPolicy::Performance,
                1 => GovernorPolicy::Powersave,
                2 => GovernorPolicy::OnDemand,
                3 => GovernorPolicy::Conservative,
                4 => GovernorPolicy::Schedutil,
                _ => GovernorPolicy::Unknown,
            };
        }
        let detected = read_governor_sysfs();
        if detected != GovernorPolicy::Unknown {
            Self::set_policy(detected);
        }
        detected
    }

    pub fn apply(&self) {
        let policy = Self::get_policy();
        let freq = crate::power::dvfs::current_frequency();
        match policy {
            GovernorPolicy::Performance | GovernorPolicy::Schedutil => {
                if freq > 0 {
                    crate::power::dvfs::set_frequency(freq);
                }
            }
            GovernorPolicy::Powersave => {}
            GovernorPolicy::OnDemand => {
                if freq > 0 {
                    crate::power::dvfs::set_frequency(freq);
                }
            }
            GovernorPolicy::Conservative => {
                if freq > 0 {
                    crate::power::dvfs::set_frequency(freq / 2);
                }
            }
            GovernorPolicy::Unknown => {}
        }
    }
}

fn read_governor_sysfs() -> GovernorPolicy {
    GovernorPolicy::Unknown
}
