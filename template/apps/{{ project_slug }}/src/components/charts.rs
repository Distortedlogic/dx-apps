use dioxus::prelude::*;
use dx_charming::{
  CharmingChart,
  charming::{Chart, component::Axis, series::Bar},
};

#[component]
pub fn StarterChart() -> Element {
  let options = Chart::new()
    .x_axis(Axis::new().data(vec!["One".to_string(), "Two".to_string(), "Three".to_string()]))
    .y_axis(Axis::new())
    .series(Bar::new().data(vec![3, 7, 5]));
  rsx! {
    div { class: "h-80 rounded-xl border border-border bg-card p-4 text-card-foreground",
      CharmingChart { options }
    }
  }
}
