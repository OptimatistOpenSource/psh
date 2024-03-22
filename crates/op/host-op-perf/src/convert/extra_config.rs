use crate::convert::Wrap;

type FromT = crate::profiling::perf::config::ExtraConfig;
type IntoT = perf_event_rs::counting::ExtraConfig;

impl TryFrom<&FromT> for Wrap<IntoT> {
    type Error = super::Error;

    fn try_from(value: &FromT) -> Result<Self, Self::Error> {
        #[cfg(not(feature = "linux-5.13"))]
        if value.remove_on_exec {
            return Err(Self::Error::UnsupportedOption(
                "ExtraConfig.remove_on_exec = true".to_string(),
            ));
        }

        #[rustfmt::skip]
        let val = IntoT {
            pinned:         value.pinned,
            exclusive:      value.exclusive,
            inherit:        value.inherit,
            inherit_stat:   value.inherit_stat,
            inherit_thread: value.inherit_thread,
            enable_on_exec: value.enable_on_exec,
            #[cfg(feature = "linux-5.13")]
            remove_on_exec: value.remove_on_exec,
        };
        Ok(Self(val))
    }
}
