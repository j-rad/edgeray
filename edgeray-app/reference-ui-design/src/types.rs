use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NavigationTab {
    Core,
    Nodes,
    Mesh,
    Topology,
    Tracer,
    Setup,
    Routing,
    Settings,
    Diagnostics,
}

impl NavigationTab {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavigationTab::Core => "core",
            NavigationTab::Nodes => "nodes",
            NavigationTab::Mesh => "mesh",
            NavigationTab::Topology => "topology",
            NavigationTab::Tracer => "tracer",
            NavigationTab::Setup => "setup",
            NavigationTab::Routing => "routing",
            NavigationTab::Settings => "settings",
            NavigationTab::Diagnostics => "diagnostics",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            NavigationTab::Core => "OVERVIEW",
            NavigationTab::Nodes => "NODES",
            NavigationTab::Mesh => "MESH",
            NavigationTab::Topology => "TOPOLOGY",
            NavigationTab::Tracer => "TRACER",
            NavigationTab::Setup => "SETUP",
            NavigationTab::Routing => "ROUTING",
            NavigationTab::Settings => "SETTINGS",
            NavigationTab::Diagnostics => "DIAGNOSTICS",
        }
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum NodeType {
    Proxy,
    Direct,
    Relay,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct NetworkNode {
    pub id: String,
    pub name: String,
    pub region: String,
    pub location: String,
    pub flag_url: String,
    pub latency: u32,
    pub jitter: u32,
    pub bandwidth: String,
    pub protocols: Vec<String>,
    pub active: bool,
    pub node_type: NodeType,
    pub status: String,
}

#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct MetricPoint {
    pub time: String,
    pub value: f64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum TracerStatus {
    Active,
    Success,
    Pending,
    Warning,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TracerDetail {
    pub label: String,
    pub value: String,
    pub color: Option<String>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TracerStep {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub status: TracerStatus,
    pub details: Vec<TracerDetail>,
    pub icon: String,
    pub color: String,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PeerType {
    Router,
    Cloud,
    Server,
    Device,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum PeerStatus {
    Good,
    Fair,
    Poor,
}

#[derive(Clone, PartialEq, Debug)]
pub struct MeshPeer {
    pub id: String,
    pub name: String,
    pub rtt: u32,
    pub x: f64,
    pub y: f64,
    pub peer_type: PeerType,
    pub status: PeerStatus,
}
