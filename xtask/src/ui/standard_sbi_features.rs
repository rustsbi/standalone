use crate::{app::StandardSbiEnabled, ui::Builder, App};
use ratatui::{layout::Constraint::*, Frame};
use std::ops::ControlFlow;

pub fn draw_standard_sbi_features(f: &mut Frame, app: &mut App) {
    let StandardSbiEnabled {
        timer,
        ipi,
        rfence,
        hsm,
        srst,
        pmu,
        dbcn,
        susp,
        cppc,
        nacl,
        sta,
    } = app.standard_sbi_enabled;
    fn choose_str(enabled: bool) -> &'static str {
        match enabled {
            true => "standard-sbi-features.enabled",
            false => "standard-sbi-features.disabled",
        }
    }
    #[rustfmt::skip]
    let items = vec![
        vec!["TimerExtension", "standard-sbi-features.timer", choose_str(timer) ],
        vec!["IpiExtension", "standard-sbi-features.ipi", choose_str(ipi)],
        vec!["RfenceExtension", "standard-sbi-features.rfence", choose_str(rfence)],
        vec!["HsmExtension", "standard-sbi-features.hsm", choose_str(hsm)],
        vec!["SrstExtension", "standard-sbi-features.srst", choose_str(srst)],
        vec!["PmuExtension", "standard-sbi-features.pmu", choose_str(pmu)],
        vec!["DbcnExtension", "standard-sbi-features.dbcn", choose_str(dbcn)],
        vec!["SuspExtension", "standard-sbi-features.susp", choose_str(susp)],
        vec!["CppcExtension", "standard-sbi-features.cppc", choose_str(cppc)],
        vec!["NaclExtension", "standard-sbi-features.nacl", choose_str(nacl)],
        vec!["StaExtension", "standard-sbi-features.sta", choose_str(sta)],
        vec!["Back", "back", ""],
    ];
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.standard_sbi_enabled.timer = !app.standard_sbi_enabled.timer,
            1 => app.standard_sbi_enabled.ipi = !app.standard_sbi_enabled.ipi,
            2 => app.standard_sbi_enabled.rfence = !app.standard_sbi_enabled.rfence,
            3 => app.standard_sbi_enabled.hsm = !app.standard_sbi_enabled.hsm,
            4 => app.standard_sbi_enabled.srst = !app.standard_sbi_enabled.srst,
            5 => app.standard_sbi_enabled.pmu = !app.standard_sbi_enabled.pmu,
            6 => app.standard_sbi_enabled.dbcn = !app.standard_sbi_enabled.dbcn,
            7 => app.standard_sbi_enabled.susp = !app.standard_sbi_enabled.susp,
            8 => app.standard_sbi_enabled.cppc = !app.standard_sbi_enabled.cppc,
            9 => app.standard_sbi_enabled.nacl = !app.standard_sbi_enabled.nacl,
            10 => app.standard_sbi_enabled.sta = !app.standard_sbi_enabled.sta,
            11 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "standard-sbi-features.title",
        header: vec!["id", "home.item", "home.brief"],
        items,
        item_translate_idx: vec![1, 2],
        widths: vec![Length(18), Min(30), Length(12)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
