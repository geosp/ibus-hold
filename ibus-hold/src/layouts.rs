//! Keyboard layouts offered as separate input sources.
//!
//! GNOME loads the keyboard layout named in each engine's `<layout>`, so one
//! engine is listed per layout: e.g. "German (Press and Hold)" under German in
//! Settings. All of them behave the same; the accent popup works with keysyms,
//! so it is independent of the layout.
//!
//! The component XML (`data/ibus-hold.xml`) is generated from this table:
//! `ibus-engine-hold --print-component-xml > ibus-hold/data/ibus-hold.xml`.
//! A test checks that the committed file is up to date.

/// One input source: (engine name, xkb layout, ISO 639 language, display name)
pub struct Layout {
    pub engine: &'static str,
    pub xkb: &'static str,
    pub language: &'static str,
    /// Name as shown in GNOME Settings: `Language (details)`
    pub longname: &'static str,
}

impl Layout {
    /// The `<longname>` for the component XML: only the `details` part,
    /// because GNOME Settings shows ibus engines as `Language (longname)`.
    pub fn ibus_longname(&self) -> &'static str {
        self.longname
            .split_once(" (")
            .and_then(|(_, rest)| rest.strip_suffix(')'))
            .unwrap_or(self.longname)
    }
}

const fn l(
    engine: &'static str,
    xkb: &'static str,
    language: &'static str,
    longname: &'static str,
) -> Layout {
    Layout {
        engine,
        xkb,
        language,
        longname,
    }
}

/// The US engine keeps the plain name `hold` so existing setups keep working.
pub const LAYOUTS: &[Layout] = &[
    l("hold", "us", "en", "English (US, Press and Hold)"),
    l("hold:gb", "gb", "en", "English (UK, Press and Hold)"),
    l("hold:de", "de", "de", "German (Press and Hold)"),
    l(
        "hold:ch",
        "ch",
        "de",
        "German (Switzerland, Press and Hold)",
    ),
    l("hold:fr", "fr", "fr", "French (Press and Hold)"),
    l("hold:ca", "ca", "fr", "French (Canada, Press and Hold)"),
    l("hold:es", "es", "es", "Spanish (Press and Hold)"),
    l(
        "hold:latam",
        "latam",
        "es",
        "Spanish (Latin American, Press and Hold)",
    ),
    l("hold:pt", "pt", "pt", "Portuguese (Press and Hold)"),
    l("hold:br", "br", "pt", "Portuguese (Brazil, Press and Hold)"),
    l("hold:it", "it", "it", "Italian (Press and Hold)"),
    l("hold:se", "se", "sv", "Swedish (Press and Hold)"),
    l("hold:no", "no", "nb", "Norwegian (Press and Hold)"),
    l("hold:dk", "dk", "da", "Danish (Press and Hold)"),
    l("hold:fi", "fi", "fi", "Finnish (Press and Hold)"),
    l("hold:pl", "pl", "pl", "Polish (Press and Hold)"),
    l("hold:tr", "tr", "tr", "Turkish (Press and Hold)"),
];

pub fn is_engine(name: &str) -> bool {
    LAYOUTS.iter().any(|l| l.engine == name)
}

/// The ibus component XML listing every engine.
pub fn component_xml(bus_name: &str, version: &str) -> String {
    let license = "LGPL-2.1-or-later OR GPL-3.0-or-later";
    let mut engines = String::new();
    for l in LAYOUTS {
        engines.push_str(&format!(
            r#"    <engine>
      <name>{engine}</name>
      <language>{language}</language>
      <license>{license}</license>
      <author>geosp</author>
      <layout>{xkb}</layout>
      <longname>{longname}</longname>
      <description>Hold a letter to pick an accented variant, like on macOS</description>
      <rank>0</rank>
      <symbol>Hå</symbol>
    </engine>
"#,
            engine = l.engine,
            language = l.language,
            xkb = l.xkb,
            longname = l.ibus_longname(),
        ));
    }
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<!-- Generated from ibus-hold/src/layouts.rs (see the print-component-xml option); do not edit by hand. -->
<!-- Install to: /usr/share/ibus/component/ibus-hold.xml -->
<component>
  <name>{bus_name}</name>
  <description>Press-and-hold accent picker</description>
  <exec>/usr/libexec/ibus-engine-hold --ibus</exec>
  <version>{version}</version>
  <author>geosp</author>
  <license>{license}</license>
  <homepage>https://github.com/geosp/ibus-hold</homepage>
  <textdomain>ibus-hold</textdomain>

  <engines>
{engines}  </engines>
</component>
"#
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn engine_names_are_unique() {
        for (i, a) in LAYOUTS.iter().enumerate() {
            assert!(
                LAYOUTS[i + 1..].iter().all(|b| b.engine != a.engine),
                "duplicate engine {}",
                a.engine
            );
        }
        assert!(is_engine("hold"));
        assert!(is_engine("hold:de"));
        assert!(!is_engine("hold:xx"));
    }

    #[test]
    fn longnames_have_language_and_details() {
        for l in LAYOUTS {
            let details = l.ibus_longname();
            assert_ne!(
                details, l.longname,
                "{}: expected `Language (details)`",
                l.engine
            );
            assert!(details.ends_with("Press and Hold"), "{}", l.engine);
        }
        assert_eq!(LAYOUTS[0].ibus_longname(), "US, Press and Hold");
    }

    #[test]
    fn xml_comments_are_valid() {
        // `--` inside an XML comment makes the whole file unparsable
        let xml = component_xml(crate::BUS_NAME, "0.0.0");
        for comment in xml.split("<!--").skip(1) {
            let body = comment.split("-->").next().unwrap();
            assert!(!body.contains("--"), "invalid XML comment: {body}");
        }
    }

    #[test]
    fn install_guide_lists_every_layout() {
        let doc = include_str!("../../doc/ibus-install.md");
        for l in LAYOUTS {
            let row = format!("| {} | `{}` | `{}` |", l.longname, l.engine, l.xkb);
            assert!(doc.contains(&row), "doc/ibus-install.md is missing: {row}");
        }
    }

    #[test]
    fn committed_component_xml_is_up_to_date() {
        let committed = include_str!("../data/ibus-hold.xml");
        let generated = component_xml(crate::BUS_NAME, env!("CARGO_PKG_VERSION"));
        assert!(
            committed == generated,
            "data/ibus-hold.xml is stale; regenerate it with:\n  \
             cargo run -p ibus-hold -- --print-component-xml > ibus-hold/data/ibus-hold.xml"
        );
    }
}
