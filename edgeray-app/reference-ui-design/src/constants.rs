use crate::types::*;

pub const USER_AVATAR: &str =
    "https://images.unsplash.com/photo-1507003211169-0a1dd7228f2d?fit=crop&w=100&h=100";

pub fn get_nodes() -> Vec<NetworkNode> {
    vec![
        NetworkNode {
            id: "JP-TYO-B4".to_string(),
            name: "Tokyo Edge Pro-01".to_string(),
            region: "Japan".to_string(),
            location: "Tokyo, Japan".to_string(),
            flag_url: "https://flagcdn.com/w80/jp.png".to_string(),
            latency: 24,
            jitter: 2,
            bandwidth: "1.2 Gbps".to_string(),
            protocols: vec![
                "VLESS".to_string(),
                "XTLS".to_string(),
                "Reality".to_string(),
            ],
            active: true,
            node_type: NodeType::Proxy,
            status: "Online".to_string(),
        },
        NetworkNode {
            id: "US-SFO-09".to_string(),
            name: "San Francisco Cloud".to_string(),
            region: "USA".to_string(),
            location: "San Francisco, CA".to_string(),
            flag_url: "https://flagcdn.com/w80/us.png".to_string(),
            latency: 164,
            jitter: 12,
            bandwidth: "2.5 Gbps".to_string(),
            protocols: vec!["HY2".to_string(), "BBR".to_string()],
            active: false,
            node_type: NodeType::Direct,
            status: "Online".to_string(),
        },
        NetworkNode {
            id: "SG-SIN-11".to_string(),
            name: "Singapore G-Net".to_string(),
            region: "Singapore".to_string(),
            location: "Singapore".to_string(),
            flag_url: "https://flagcdn.com/w80/sg.png".to_string(),
            latency: 58,
            jitter: 5,
            bandwidth: "800 Mbps".to_string(),
            protocols: vec!["Trojan".to_string(), "WS".to_string()],
            active: false,
            node_type: NodeType::Relay,
            status: "Online".to_string(),
        },
    ]
}

pub fn get_mesh_peers() -> Vec<MeshPeer> {
    vec![
        MeshPeer {
            id: "ME".to_string(),
            name: "ME_LOCAL".to_string(),
            rtt: 0,
            x: 50.0,
            y: 50.0,
            peer_type: PeerType::Device,
            status: PeerStatus::Good,
        },
        MeshPeer {
            id: "TYO".to_string(),
            name: "TYO-092".to_string(),
            rtt: 32,
            x: 80.0,
            y: 25.0,
            peer_type: PeerType::Server,
            status: PeerStatus::Good,
        },
        MeshPeer {
            id: "FRA".to_string(),
            name: "FRA-841".to_string(),
            rtt: 124,
            x: 15.0,
            y: 30.0,
            peer_type: PeerType::Cloud,
            status: PeerStatus::Fair,
        },
        MeshPeer {
            id: "LAX".to_string(),
            name: "LAX-012".to_string(),
            rtt: 188,
            x: 75.0,
            y: 70.0,
            peer_type: PeerType::Router,
            status: PeerStatus::Poor,
        },
        MeshPeer {
            id: "LDN".to_string(),
            name: "LDN-442".to_string(),
            rtt: 48,
            x: 25.0,
            y: 65.0,
            peer_type: PeerType::Server,
            status: PeerStatus::Good,
        },
    ]
}

pub fn get_tracer_steps() -> Vec<TracerStep> {
    vec![
        TracerStep {
            id: "1".to_string(),
            title: "Source App".to_string(),
            subtitle: "Web Browser Core".to_string(),
            status: TracerStatus::Success,
            icon: "AppWindow".to_string(),
            color: "primary".to_string(),
            details: vec![
                TracerDetail {
                    label: "Process ID".to_string(),
                    value: "#4492".to_string(),
                    color: None,
                },
                TracerDetail {
                    label: "Local Port".to_string(),
                    value: "59221".to_string(),
                    color: None,
                },
            ],
        },
        TracerStep {
            id: "2".to_string(),
            title: "Rule Match".to_string(),
            subtitle: "Direct Connect".to_string(),
            status: TracerStatus::Success,
            icon: "Filter".to_string(),
            color: "cyber-purple".to_string(),
            details: vec![
                TracerDetail {
                    label: "Policy".to_string(),
                    value: "Internal-Auto".to_string(),
                    color: None,
                },
                TracerDetail {
                    label: "Keyword".to_string(),
                    value: "edge-ray".to_string(),
                    color: None,
                },
            ],
        },
        TracerStep {
            id: "3".to_string(),
            title: "Outbound Proxy".to_string(),
            subtitle: "Tokyo Edge-01".to_string(),
            status: TracerStatus::Success,
            icon: "Router".to_string(),
            color: "success-emerald".to_string(),
            details: vec![
                TracerDetail {
                    label: "Latency".to_string(),
                    value: "24ms".to_string(),
                    color: Some("text-success-emerald".to_string()),
                },
                TracerDetail {
                    label: "Protocol".to_string(),
                    value: "Reality".to_string(),
                    color: None,
                },
            ],
        },
    ]
}
