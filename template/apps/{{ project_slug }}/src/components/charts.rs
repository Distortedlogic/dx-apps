use dioxus::prelude::*;
use dx_charming::{
  CharmingChart,
  charming::{Chart, component::Axis, series::Bar},
};

#[component]
pub fn StarterChart() -> Element {
  let options = Chart::new().x_axis(Axis::new().data(["One", "Two", "Three"])).y_axis(Axis::new()).series(Bar::new().data([3, 7, 5]));
  rsx! {
    div { class: "h-80 rounded-xl border border-border bg-card p-4 text-card-foreground",
      CharmingChart { options }
    }
  }
}
