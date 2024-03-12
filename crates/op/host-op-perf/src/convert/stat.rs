use crate::convert::Wrap;

const _: () = {
    type FromT = perf_event_rs::counting::CounterStat;
    type IntoT = crate::profiling::perf::counter::CounterStat;

    impl From<&FromT> for Wrap<IntoT> {
        fn from(value: &FromT) -> Self {
            #[rustfmt::skip]
            let val = IntoT {
                event_id:     value.event_id,
                event_count:  value.event_count,
                time_enabled: value.time_enabled,
                time_running: value.time_running,
            };
            Self(val)
        }
    }
};

const _: () = {
    type FromT = perf_event_rs::counting::CounterGroupStat;
    type IntoT = crate::profiling::perf::counter_group::CounterGroupStat;

    impl From<&FromT> for Wrap<IntoT> {
        fn from(value: &FromT) -> Self {
            #[rustfmt::skip]
            let val = IntoT {
                time_enabled:  value.time_enabled,
                time_running:  value.time_running,
                member_counts: value.member_counts.clone().into_iter().collect(),
            };
            Self(val)
        }
    }
};
