#![allow(warnings, rust_2018_idioms)]
mod nm;
mod nm_accesspoint;
mod nm_agentmanager;
mod nm_checkpoint;
mod nm_connection_active;
mod nm_device;
mod nm_device_adsl;
mod nm_device_bluetooth;
mod nm_device_bond;
mod nm_device_bridge;
mod nm_device_dummy;
mod nm_device_generic;
mod nm_device_infiniband;
mod nm_device_iptunnel;
mod nm_device_loopback;
mod nm_device_lowpan;
mod nm_device_macsec;
mod nm_device_macvlan;
mod nm_device_modem;
mod nm_device_olpcmesh;
mod nm_device_ovsbridge;
mod nm_device_ovsinterface;
mod nm_device_ovsport;
mod nm_device_ppp;
mod nm_device_statistics;
mod nm_device_team;
mod nm_device_tun;
mod nm_device_veth;
mod nm_device_vlan;
mod nm_device_vrf;
mod nm_device_vxlan;
mod nm_device_wifip2p;
mod nm_device_wimax;
mod nm_device_wired;
mod nm_device_wireguard;
mod nm_device_wireless;
mod nm_device_wpan;
mod nm_dhcp4config;
mod nm_dhcp6config;
mod nm_dnsmanager;
mod nm_ip4config;
mod nm_ip6config;
mod nm_ppp;
mod nm_secretagent;
mod nm_settings;
mod nm_settings_connection;
mod nm_vpn_connection;
mod nm_vpn_plugin;
mod nm_wifip2ppeer;
mod nm_wimax_nsp;

pub(super) use nm::OrgFreedesktopNetworkManager;
pub(super) use nm_accesspoint::OrgFreedesktopNetworkManagerAccessPoint;
pub(super) use nm_agentmanager::OrgFreedesktopNetworkManagerAgentManager;
pub(super) use nm_checkpoint::OrgFreedesktopNetworkManagerCheckpoint;
pub(super) use nm_connection_active::OrgFreedesktopNetworkManagerConnectionActive;
pub(super) use nm_device::OrgFreedesktopNetworkManagerDevice;
pub(super) use nm_device_adsl::OrgFreedesktopNetworkManagerDeviceAdsl;
pub(super) use nm_device_bluetooth::OrgFreedesktopNetworkManagerDeviceBluetooth;
pub(super) use nm_device_bond::OrgFreedesktopNetworkManagerDeviceBond;
pub(super) use nm_device_bridge::OrgFreedesktopNetworkManagerDeviceBridge;
pub(super) use nm_device_dummy::OrgFreedesktopNetworkManagerDeviceDummy;
pub(super) use nm_device_generic::OrgFreedesktopNetworkManagerDeviceGeneric;
pub(super) use nm_device_infiniband::OrgFreedesktopNetworkManagerDeviceInfiniband;
pub(super) use nm_device_iptunnel::OrgFreedesktopNetworkManagerDeviceIPTunnel;
pub(super) use nm_device_loopback::OrgFreedesktopNetworkManagerDeviceLoopback;
pub(super) use nm_device_lowpan::OrgFreedesktopNetworkManagerDeviceLowpan;
pub(super) use nm_device_macsec::OrgFreedesktopNetworkManagerDeviceMacsec;
pub(super) use nm_device_macvlan::OrgFreedesktopNetworkManagerDeviceMacvlan;
pub(super) use nm_device_modem::OrgFreedesktopNetworkManagerDeviceModem;
pub(super) use nm_device_olpcmesh::OrgFreedesktopNetworkManagerDeviceOlpcMesh;
pub(super) use nm_device_ovsbridge::OrgFreedesktopNetworkManagerDeviceOvsBridge;
pub(super) use nm_device_ovsinterface::OrgFreedesktopNetworkManagerDeviceOvsInterface;
pub(super) use nm_device_ovsport::OrgFreedesktopNetworkManagerDeviceOvsPort;
pub(super) use nm_device_ppp::OrgFreedesktopNetworkManagerDevicePpp;
pub(super) use nm_device_statistics::OrgFreedesktopNetworkManagerDeviceStatistics;
pub(super) use nm_device_team::OrgFreedesktopNetworkManagerDeviceTeam;
pub(super) use nm_device_tun::OrgFreedesktopNetworkManagerDeviceTun;
pub(super) use nm_device_veth::OrgFreedesktopNetworkManagerDeviceVeth;
pub(super) use nm_device_vlan::OrgFreedesktopNetworkManagerDeviceVlan;
pub(super) use nm_device_vrf::OrgFreedesktopNetworkManagerDeviceVrf;
pub(super) use nm_device_vxlan::OrgFreedesktopNetworkManagerDeviceVxlan;
pub(super) use nm_device_wifip2p::OrgFreedesktopNetworkManagerDeviceWifiP2P;
pub(super) use nm_device_wimax::OrgFreedesktopNetworkManagerDeviceWiMax;
pub(super) use nm_device_wired::OrgFreedesktopNetworkManagerDeviceWired;
pub(super) use nm_device_wireguard::OrgFreedesktopNetworkManagerDeviceWireGuard;
pub(super) use nm_device_wireless::OrgFreedesktopNetworkManagerDeviceWireless;
pub(super) use nm_device_wpan::OrgFreedesktopNetworkManagerDeviceWpan;
pub(super) use nm_dhcp4config::OrgFreedesktopNetworkManagerDHCP4Config;
pub(super) use nm_dhcp6config::OrgFreedesktopNetworkManagerDHCP6Config;
pub(super) use nm_dnsmanager::OrgFreedesktopNetworkManagerDnsManager;
pub(super) use nm_ip4config::OrgFreedesktopNetworkManagerIP4Config;
pub(super) use nm_ip6config::OrgFreedesktopNetworkManagerIP6Config;
pub(super) use nm_ppp::OrgFreedesktopNetworkManagerPPP;
pub(super) use nm_secretagent::OrgFreedesktopNetworkManagerSecretAgent;
pub(super) use nm_settings::OrgFreedesktopNetworkManagerSettings;
pub(super) use nm_settings_connection::OrgFreedesktopNetworkManagerSettingsConnection;
pub(super) use nm_vpn_connection::OrgFreedesktopNetworkManagerVPNConnection;
pub(super) use nm_vpn_plugin::OrgFreedesktopNetworkManagerVPNPlugin;
pub(super) use nm_wifip2ppeer::OrgFreedesktopNetworkManagerWifiP2PPeer;
pub(super) use nm_wimax_nsp::OrgFreedesktopNetworkManagerWiMaxNsp;
