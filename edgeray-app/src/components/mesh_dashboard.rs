use crate::ui::mesh::peer_topology::PeerTopology;
use dioxus::prelude::*;

#[component]
pub fn MeshDashboard() -> Element {
    rsx! {
        div {
            class: "w-full h-full flex flex-col",
            PeerTopology {}
        }
    }
}
