//! Protocol Form Components
//!
//! Complex Dioxus forms for all supported protocols matching rr-ui specification.

use crate::models::{Protocol, ServerConfig};
use dioxus::prelude::*;

use crate::components::forms::{
    Alert, AlertVariant, Button, ButtonVariant, FormCard, NumberInput, Select, TextInput, Toggle,
};

/// Protocol selection dropdown with visual badges
#[derive(Props, Clone, PartialEq)]
pub struct ProtocolSelectorProps {
    /// Current protocol
    pub protocol: Signal<Protocol>,
    /// Change handler
    #[props(default = None)]
    pub on_change: Option<EventHandler<Protocol>>,
}

#[component]
pub fn ProtocolSelector(props: ProtocolSelectorProps) -> Element {
    let mut protocol = props.protocol;

    let protocols = [
        (
            Protocol::Vless,
            "VLESS",
            "bg-blue-500/20 text-blue-400",
            "Modern, efficient protocol",
        ),
        (
            Protocol::Vmess,
            "VMess",
            "bg-purple-500/20 text-purple-400",
            "Classic V2Ray protocol",
        ),
        (
            Protocol::Trojan,
            "Trojan",
            "bg-amber-500/20 text-amber-400",
            "HTTPS-based stealth",
        ),
        (
            Protocol::Shadowsocks,
            "Shadowsocks",
            "bg-emerald-500/20 text-emerald-400",
            "Fast encrypted proxy",
        ),
        (
            Protocol::Hysteria2,
            "Hysteria2",
            "bg-pink-500/20 text-pink-400",
            "QUIC-based high-speed",
        ),
    ];

    rsx! {
        div {
            class: "grid grid-cols-2 sm:grid-cols-3 gap-3",
            for (proto, name, badge_class, desc) in protocols {
                button {
                    class: format!(
                        "flex flex-col items-start p-4 rounded-xl border transition-all text-left {}",
                        if *protocol.read() == proto {
                            "bg-primary/20 border-primary/50"
                        } else {
                            "bg-white/5 border-white/10 hover:bg-white/10"
                        }
                    ),
                    onclick: {
                        let on_change = props.on_change.clone();
                        move |_| {
                            protocol.set(proto.clone());
                            if let Some(ref handler) = on_change {
                                handler.call(proto.clone());
                            }
                        }
                    },
                    span {
                        class: format!("px-2 py-0.5 text-[10px] font-bold uppercase rounded {}", badge_class),
                        "{name}"
                    }
                    span {
                        class: "text-xs text-slate-400 mt-2",
                        "{desc}"
                    }
                }
            }
        }
    }
}

/// VLESS Protocol Form
#[derive(Props, Clone, PartialEq)]
pub struct VlessFormProps {
    /// Server config to edit (None for new)
    #[props(default = None)]
    pub config: Option<ServerConfig>,
    /// Save handler
    pub on_save: EventHandler<ServerConfig>,
    /// Cancel handler
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn VlessForm(props: VlessFormProps) -> Element {
    let existing = props.config.clone();

    // Form state
    let remarks = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.remarks.clone())
            .unwrap_or_default()
    });
    let address = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.address.clone())
            .unwrap_or_default()
    });
    let port = use_signal(|| existing.as_ref().map(|c| c.port as i64).unwrap_or(443));
    let uuid = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.uuid.clone())
            .unwrap_or_default()
    });
    let flow = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.flow.clone())
            .unwrap_or_default()
    });
    let network = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.network.clone())
            .unwrap_or_else(|| "tcp".to_string())
    });
    let security = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.security.clone())
            .unwrap_or_else(|| "tls".to_string())
    });
    let _encryption = use_signal(|| "none".to_string());
    let _encryption = use_signal(|| "none".to_string());
    let sni = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.sni.clone())
            .unwrap_or_default()
    });
    let fingerprint = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.fingerprint.clone())
            .unwrap_or_else(|| "chrome".to_string())
    });

    // REALITY settings
    let pbk = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.pbk.clone())
            .unwrap_or_default()
    });
    let sid = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.sid.clone())
            .unwrap_or_default()
    });

    let is_reality = security.read().as_str() == "reality";

    let save = move |_| {
        let config = ServerConfig {
            id: existing.as_ref().and_then(|c| c.id.clone()),
            remarks: remarks.read().clone(),
            address: address.read().clone(),
            port: *port.read() as u16,
            protocol: Protocol::Vless,
            uuid: Some(uuid.read().clone()),
            flow: if flow.read().is_empty() {
                None
            } else {
                Some(flow.read().clone())
            },
            network: Some(network.read().clone()),
            security: Some(security.read().clone()),
            sni: if sni.read().is_empty() {
                None
            } else {
                Some(sni.read().clone())
            },
            fingerprint: Some(fingerprint.read().clone()),
            pbk: if pbk.read().is_empty() {
                None
            } else {
                Some(pbk.read().clone())
            },
            sid: if sid.read().is_empty() {
                None
            } else {
                Some(sid.read().clone())
            },
            ..Default::default()
        };
        props.on_save.call(config);
    };

    rsx! {
        div {
            class: "space-y-6",

            FormCard {
                title: Some("Basic Settings".to_string()),
                TextInput {
                    label: "Remarks".to_string(),
                    value: remarks,
                    placeholder: "My VLESS Server".to_string(),
                    required: true,
                }
                div {
                    class: "grid grid-cols-3 gap-4",
                    TextInput {
                        label: "Address".to_string(),
                        value: address,
                        placeholder: "example.com".to_string(),
                        required: true,
                        class: "col-span-2".to_string(),
                    }
                    NumberInput {
                        label: "Port".to_string(),
                        value: port,
                        min: Some(1),
                        max: Some(65535),
                        required: true,
                    }
                }
                TextInput {
                    label: "UUID".to_string(),
                    value: uuid,
                    placeholder: "xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx".to_string(),
                    required: true,
                    help: Some("Client UUID for authentication".to_string()),
                }
            }

            FormCard {
                title: Some("Transport".to_string()),
                div {
                    class: "grid grid-cols-2 gap-4",
                    Select {
                        label: "Network".to_string(),
                        value: network,
                        options: vec![
                            ("tcp".to_string(), "TCP".to_string()),
                            ("ws".to_string(), "WebSocket".to_string()),
                            ("grpc".to_string(), "gRPC".to_string()),
                            ("kcp".to_string(), "mKCP".to_string()),
                            ("quic".to_string(), "QUIC".to_string()),
                            ("splithttp".to_string(), "SplitHTTP".to_string()),
                        ],
                    }
                    Select {
                        label: "Security".to_string(),
                        value: security,
                        options: vec![
                            ("none".to_string(), "None".to_string()),
                            ("tls".to_string(), "TLS".to_string()),
                            ("reality".to_string(), "REALITY".to_string()),
                        ],
                    }
                }
                Select {
                    label: "Flow".to_string(),
                    value: flow,
                    options: vec![
                        ("".to_string(), "None".to_string()),
                        ("xtls-rprx-vision".to_string(), "xtls-rprx-vision".to_string()),
                        ("xtls-rprx-vision-udp443".to_string(), "xtls-rprx-vision-udp443".to_string()),
                    ],
                    help: Some("XTLS flow control (for TLS/REALITY)".to_string()),
                }
            }

            FormCard {
                title: Some("TLS Settings".to_string()),
                TextInput {
                    label: "SNI (Server Name)".to_string(),
                    value: sni,
                    placeholder: "Leave empty to use address".to_string(),
                }
                Select {
                    label: "Fingerprint".to_string(),
                    value: fingerprint,
                    options: vec![
                        ("chrome".to_string(), "Chrome".to_string()),
                        ("firefox".to_string(), "Firefox".to_string()),
                        ("safari".to_string(), "Safari".to_string()),
                        ("ios".to_string(), "iOS".to_string()),
                        ("android".to_string(), "Android".to_string()),
                        ("edge".to_string(), "Edge".to_string()),
                        ("360".to_string(), "360 Browser".to_string()),
                        ("qq".to_string(), "QQ Browser".to_string()),
                        ("random".to_string(), "Random".to_string()),
                        ("randomized".to_string(), "Randomized".to_string()),
                    ],
                }
            }

            if is_reality {
                FormCard {
                    title: Some("REALITY Settings".to_string()),
                    description: Some("Advanced stealth settings for REALITY protocol".to_string()),
                    TextInput {
                        label: "Public Key".to_string(),
                        value: pbk,
                        placeholder: "Server's REALITY public key".to_string(),
                        required: true,
                    }
                    TextInput {
                        label: "Short ID".to_string(),
                        value: sid,
                        placeholder: "0-16 character hex string".to_string(),
                        help: Some("Optional short ID for additional authentication".to_string()),
                    }
                }
            }

            // Action buttons
            div {
                class: "flex items-center justify-end gap-3 pt-4 border-t border-white/10",
                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: move |_| props.on_cancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: save,
                    icon: Some("save".to_string()),
                    "Save Configuration"
                }
            }
        }
    }
}

/// Shadowsocks Protocol Form
#[derive(Props, Clone, PartialEq)]
pub struct ShadowsocksFormProps {
    /// Server config to edit
    #[props(default = None)]
    pub config: Option<ServerConfig>,
    /// Save handler
    pub on_save: EventHandler<ServerConfig>,
    /// Cancel handler
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn ShadowsocksForm(props: ShadowsocksFormProps) -> Element {
    let existing = props.config.clone();

    let remarks = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.remarks.clone())
            .unwrap_or_default()
    });
    let address = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.address.clone())
            .unwrap_or_default()
    });
    let port = use_signal(|| existing.as_ref().map(|c| c.port as i64).unwrap_or(8388));
    let password = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.password.clone())
            .unwrap_or_default()
    });
    let method = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.method.clone())
            .unwrap_or_else(|| "2022-blake3-aes-128-gcm".to_string())
    });

    let save = move |_| {
        let config = ServerConfig {
            id: existing.as_ref().and_then(|c| c.id.clone()),
            remarks: remarks.read().clone(),
            address: address.read().clone(),
            port: *port.read() as u16,
            protocol: Protocol::Shadowsocks,
            password: Some(password.read().clone()),
            method: Some(method.read().clone()),
            ..Default::default()
        };
        props.on_save.call(config);
    };

    rsx! {
        div {
            class: "space-y-6",

            FormCard {
                title: Some("Server".to_string()),
                TextInput {
                    label: "Remarks".to_string(),
                    value: remarks,
                    placeholder: "My SS Server".to_string(),
                    required: true,
                }
                div {
                    class: "grid grid-cols-3 gap-4",
                    TextInput {
                        label: "Address".to_string(),
                        value: address,
                        required: true,
                        class: "col-span-2".to_string(),
                    }
                    NumberInput {
                        label: "Port".to_string(),
                        value: port,
                        min: Some(1),
                        max: Some(65535),
                    }
                }
            }

            FormCard {
                title: Some("Encryption".to_string()),
                Select {
                    label: "Method".to_string(),
                    value: method,
                    options: vec![
                        ("2022-blake3-aes-128-gcm".to_string(), "2022-blake3-aes-128-gcm (Recommended)".to_string()),
                        ("2022-blake3-aes-256-gcm".to_string(), "2022-blake3-aes-256-gcm".to_string()),
                        ("2022-blake3-chacha20-poly1305".to_string(), "2022-blake3-chacha20-poly1305".to_string()),
                        ("aes-128-gcm".to_string(), "aes-128-gcm (Legacy)".to_string()),
                        ("aes-256-gcm".to_string(), "aes-256-gcm (Legacy)".to_string()),
                        ("chacha20-ietf-poly1305".to_string(), "chacha20-ietf-poly1305 (Legacy)".to_string()),
                    ],
                }
                TextInput {
                    label: "Password / Key".to_string(),
                    value: password,
                    input_type: "password".to_string(),
                    required: true,
                    help: Some("Base64 key for 2022 methods, password for legacy".to_string()),
                }
            }

            div {
                class: "flex items-center justify-end gap-3 pt-4 border-t border-white/10",
                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: move |_| props.on_cancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: save,
                    icon: Some("save".to_string()),
                    "Save Configuration"
                }
            }
        }
    }
}

/// Flow-J Protocol Form (EdgeRay proprietary)
#[derive(Props, Clone, PartialEq)]
pub struct FlowJFormProps {
    /// Server config to edit
    #[props(default = None)]
    pub config: Option<ServerConfig>,
    /// Save handler
    pub on_save: EventHandler<ServerConfig>,
    /// Cancel handler
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn FlowJForm(props: FlowJFormProps) -> Element {
    let existing = props.config.clone();

    let remarks = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.remarks.clone())
            .unwrap_or_default()
    });
    let address = use_signal(|| {
        existing
            .as_ref()
            .map(|c| c.address.clone())
            .unwrap_or_default()
    });
    let port = use_signal(|| existing.as_ref().map(|c| c.port as i64).unwrap_or(443));
    let uuid = use_signal(|| {
        existing
            .as_ref()
            .and_then(|c| c.uuid.clone())
            .unwrap_or_default()
    });

    // Flow-J specific settings
    let mode = use_signal(|| "auto".to_string());
    let enable_fec = use_signal(|| false);
    let enable_stealth = use_signal(|| true);

    rsx! {
        div {
            class: "space-y-6",

            Alert {
                variant: AlertVariant::Info,
                title: Some("Flow-J Protocol".to_string()),
                message: "EdgeRay's proprietary protocol with advanced stealth features and superior throughput.".to_string(),
            }

            FormCard {
                title: Some("Server".to_string()),
                TextInput {
                    label: "Remarks".to_string(),
                    value: remarks,
                    placeholder: "Flow-J Server".to_string(),
                    required: true,
                }
                div {
                    class: "grid grid-cols-3 gap-4",
                    TextInput {
                        label: "Address".to_string(),
                        value: address,
                        required: true,
                        class: "col-span-2".to_string(),
                    }
                    NumberInput {
                        label: "Port".to_string(),
                        value: port,
                        min: Some(1),
                        max: Some(65535),
                    }
                }
                TextInput {
                    label: "UUID".to_string(),
                    value: uuid,
                    required: true,
                }
            }

            FormCard {
                title: Some("Flow-J Mode".to_string()),
                Select {
                    label: "Transport Mode".to_string(),
                    value: mode,
                    options: vec![
                        ("auto".to_string(), "Auto (Recommended)".to_string()),
                        ("reality".to_string(), "REALITY Stealth".to_string()),
                        ("cdn".to_string(), "CDN Camouflage".to_string()),
                        ("mqtt".to_string(), "MQTT IoT Disguise".to_string()),
                    ],
                    help: Some("Auto mode selects the best transport based on network conditions".to_string()),
                }
            }

            FormCard {
                title: Some("Advanced".to_string()),
                Toggle {
                    label: "Enable FEC".to_string(),
                    value: enable_fec,
                    description: Some("Forward Error Correction for unreliable networks".to_string()),
                }
                Toggle {
                    label: "Stealth Mode".to_string(),
                    value: enable_stealth,
                    description: Some("Anti-ML traffic analysis with probabilistic shaping".to_string()),
                }
            }

            div {
                class: "flex items-center justify-end gap-3 pt-4 border-t border-white/10",
                Button {
                    variant: ButtonVariant::Ghost,
                    on_click: move |_| props.on_cancel.call(()),
                    "Cancel"
                }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| {
                        // Build Flow-J config
                        let config = ServerConfig {
                            id: existing.as_ref().and_then(|c| c.id.clone()),
                            remarks: remarks.read().clone(),
                            address: address.read().clone(),
                            port: *port.read() as u16,
                            protocol: Protocol::Vless, // Flow-J uses VLESS as base
                            uuid: Some(uuid.read().clone()),
                            // Flow-J specific fields would go here
                            ..Default::default()
                        };
                        props.on_save.call(config);
                    },
                    icon: Some("save".to_string()),
                    "Save Configuration"
                }
            }
        }
    }
}

/// Protocol form router - renders the appropriate form based on protocol type
#[derive(Props, Clone, PartialEq)]
pub struct ProtocolFormProps {
    /// Protocol type to render form for
    pub protocol: Protocol,
    /// Existing config to edit (optional)
    #[props(default = None)]
    pub config: Option<ServerConfig>,
    /// Save handler
    pub on_save: EventHandler<ServerConfig>,
    /// Cancel handler
    pub on_cancel: EventHandler<()>,
}

#[component]
pub fn ProtocolForm(props: ProtocolFormProps) -> Element {
    match props.protocol {
        Protocol::Vless => rsx! {
            VlessForm {
                config: props.config,
                on_save: props.on_save,
                on_cancel: props.on_cancel,
            }
        },
        Protocol::Shadowsocks => rsx! {
            ShadowsocksForm {
                config: props.config,
                on_save: props.on_save,
                on_cancel: props.on_cancel,
            }
        },
        // Add other protocols as needed
        _ => rsx! {
            Alert {
                variant: AlertVariant::Warning,
                message: format!("Form for {:?} protocol is not yet implemented", props.protocol),
            }
        },
    }
}
