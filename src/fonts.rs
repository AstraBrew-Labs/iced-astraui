use iced::Font;
use iced::font::{Family, Stretch, Style, Weight};

const FAMILY: Family = Family::Name("HarmonyOS Sans");

const fn harmony_os_sans(weight: Weight) -> Font {
    Font {
        family: FAMILY,
        weight,
        stretch: Stretch::Normal,
        style: Style::Normal,
    }
}

pub const THIN: Font = harmony_os_sans(Weight::Thin);
pub const LIGHT: Font = harmony_os_sans(Weight::Light);
pub const REGULAR: Font = harmony_os_sans(Weight::Normal);
pub const MEDIUM: Font = harmony_os_sans(Weight::Medium);
pub const BOLD: Font = harmony_os_sans(Weight::Bold);
pub const BLACK: Font = harmony_os_sans(Weight::Black);

pub const FONT_MAPPINGS: [(Font, &[u8]); 6] = [
    (
        THIN,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Thin.ttf"),
    ),
    (
        LIGHT,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Light.ttf"),
    ),
    (
        REGULAR,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Regular.ttf"),
    ),
    (
        MEDIUM,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Medium.ttf"),
    ),
    (
        BOLD,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Bold.ttf"),
    ),
    (
        BLACK,
        include_bytes!("../assets/fonts/HarmonyOS_Sans_Black.ttf"),
    ),
];

#[cfg(test)]
mod tests {
    use iced::font::Weight;

    use super::FONT_MAPPINGS;

    #[test]
    fn maps_every_bundled_weight() {
        let weights = FONT_MAPPINGS.map(|(font, _)| font.weight);

        assert_eq!(
            weights,
            [
                Weight::Thin,
                Weight::Light,
                Weight::Normal,
                Weight::Medium,
                Weight::Bold,
                Weight::Black,
            ]
        );
        assert!(FONT_MAPPINGS.iter().all(|(_, bytes)| !bytes.is_empty()));
    }
}
