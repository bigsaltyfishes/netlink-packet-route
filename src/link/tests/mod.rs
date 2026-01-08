// SPDX-License-Identifier: MIT
//
// Test modules are gated by content, not blanket-disabled per OS:
//  - tests exercising Linux-only types/messages are gated to Linux;
//  - OS-neutral tests (message, sriov, xdp) run everywhere;
//  - FreeBSD-specific tests live in `freebsd.rs`.

#[cfg(all(test, not(target_os = "freebsd")))]
mod afspec;
#[cfg(all(test, not(target_os = "freebsd")))]
mod amt;
#[cfg(all(test, not(target_os = "freebsd")))]
mod bareudp;
#[cfg(all(test, not(target_os = "freebsd")))]
mod batadv;
#[cfg(all(test, not(target_os = "freebsd")))]
mod bond;
#[cfg(all(test, not(target_os = "freebsd")))]
mod bridge;
#[cfg(all(test, not(target_os = "freebsd")))]
mod can;
#[cfg(all(test, not(target_os = "freebsd")))]
mod dpll_pin;
#[cfg(all(test, target_os = "freebsd"))]
mod freebsd;
#[cfg(all(test, not(target_os = "freebsd")))]
mod geneve;
#[cfg(all(test, not(target_os = "freebsd")))]
mod gre;
#[cfg(all(test, not(target_os = "freebsd")))]
mod gtp;
#[cfg(all(test, not(target_os = "freebsd")))]
mod hsr;
#[cfg(all(test, not(target_os = "freebsd")))]
mod inet;
#[cfg(all(test, not(target_os = "freebsd")))]
mod ipoib;
#[cfg(all(test, not(target_os = "freebsd")))]
mod iptunnel;
#[cfg(all(test, not(target_os = "freebsd")))]
mod ipvlan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod ipvtap;
#[cfg(all(test, not(target_os = "freebsd")))]
mod loopback;
#[cfg(all(test, not(target_os = "freebsd")))]
mod macsec;
#[cfg(all(test, not(target_os = "freebsd")))]
mod macvlan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod macvtap;
#[cfg(test)]
mod message;
#[cfg(all(test, not(target_os = "freebsd")))]
mod netdevsim;
#[cfg(all(test, not(target_os = "freebsd")))]
mod netkit;
#[cfg(all(test, not(target_os = "freebsd")))]
mod pfcp;
#[cfg(all(test, not(target_os = "freebsd")))]
mod prop_list;
#[cfg(all(test, not(target_os = "freebsd")))]
mod rmnet;
#[cfg(test)]
mod sriov;
#[cfg(all(test, not(target_os = "freebsd")))]
mod statistics;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vcan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod veth;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vlan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vrf;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vti;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vxcan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod vxlan;
#[cfg(all(test, not(target_os = "freebsd")))]
mod wireguard;
#[cfg(all(test, not(target_os = "freebsd")))]
mod wireless;
#[cfg(all(test, not(target_os = "freebsd")))]
mod wwan;
#[cfg(test)]
mod xdp;
#[cfg(all(test, not(target_os = "freebsd")))]
mod xfrm;
