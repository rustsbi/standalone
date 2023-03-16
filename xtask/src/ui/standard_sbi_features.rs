use crate::{app::StandardSbiEnabled, ui::Builder, App};
use std::ops::ControlFlow;
use tui::{backend::Backend, layout::Constraint::*, Frame};

pub fn draw_standard_sbi_features<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let StandardSbiEnabled {
        timer,
        ipi,
        rfence,
        hsm,
        srst,
        pmu,
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
        vec!["Back", "back", ""],
    ]
    .iter()
    .map(|v| v.iter().map(|s| s.to_string()).collect::<Vec<_>>())
    .collect::<Vec<_>>();
    fn machine_mode_handle(idx: usize, app: &mut App) -> ControlFlow<(), ()> {
        match idx {
            0 => app.standard_sbi_enabled.timer = !app.standard_sbi_enabled.timer,
            1 => app.standard_sbi_enabled.ipi = !app.standard_sbi_enabled.ipi,
            2 => app.standard_sbi_enabled.rfence = !app.standard_sbi_enabled.rfence,
            3 => app.standard_sbi_enabled.hsm = !app.standard_sbi_enabled.hsm,
            4 => app.standard_sbi_enabled.srst = !app.standard_sbi_enabled.srst,
            5 => app.standard_sbi_enabled.pmu = !app.standard_sbi_enabled.pmu,
            6 => return ControlFlow::Break(()),
            _ => unreachable!(),
        };
        ControlFlow::Continue(())
    }
    Builder {
        title: "standard-sbi-features.title",
        header: vec!["id", "home.item", "home.brief"],
        items,
        item_translate_idx: vec![1, 2],
        widths: vec![Min(18), Min(30), Length(12)],
        control_flow_fn: machine_mode_handle,
    }
    .draw(f, app)
}
