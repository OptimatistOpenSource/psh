use crate::convert::Wrap;

type FromT = crate::profiling::perf::config::ExtraConfig;
type IntoT = perf_event_rs::counting::ExtraConfig;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = IntoT {
            pinned:         value.pinned,
            exclusive:      value.exclusive,
            inherit:        value.inherit,
            inherit_stat:   value.inherit_stat,
            inherit_thread: value.inherit_thread,
            enable_on_exec: value.enable_on_exec,
            remove_on_exec: value.remove_on_exec,
        };
        Self(val)
    }
}
