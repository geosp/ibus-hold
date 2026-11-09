# Build:
#   git archive --prefix=ibus-hold-0.1.0/ -o ~/rpmbuild/SOURCES/ibus-hold-0.1.0.tar.gz HEAD
#   rpmbuild -bb ibus-hold/fedora/ibus-hold.spec
# (needs network access: cargo downloads the crates listed in Cargo.lock)

# rust release builds carry no debug info to split out
%global debug_package %{nil}

Name:           ibus-hold
Version:        0.1.0
Release:        1%{?dist}
Summary:        Press-and-hold accent picker for ibus (macOS style)
License:        LGPL-2.1-or-later OR GPL-3.0-or-later
URL:            https://github.com/geosp/ibus-hold
Source0:        ibus-hold-%{version}.tar.gz

BuildRequires:  cargo
BuildRequires:  rust
Requires:       ibus

%description
Hold a letter such as "e" to get a popup with accented variants
(é è ê ë ...), then press 1-9 to pick one. A normal tap types the
letter as usual. Built on the librush ibus library.

%prep
%autosetup -n ibus-hold-%{version}

%build
cargo build --release --locked -p ibus-hold

%check
cargo test --release --locked -p ibus-hold

%install
install -Dm755 target/release/ibus-engine-hold %{buildroot}%{_libexecdir}/ibus-engine-hold
install -Dm644 ibus-hold/data/ibus-hold.xml %{buildroot}%{_datadir}/ibus/component/ibus-hold.xml

%files
%license LICENSE
%{_libexecdir}/ibus-engine-hold
%{_datadir}/ibus/component/ibus-hold.xml

%changelog
* Mon Nov 09 2026 geosp <gffajardo@gmail.com> - 0.1.0-1
- First package
