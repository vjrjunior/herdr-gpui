use super::*;

#[test]
fn every_message_naming_the_app_exists() {
    for id in NAMING_THE_APP {
        assert!(crate::i18n::takes(id).is_some(), "{id} is not a message");
    }
}

#[test]
fn messages_naming_the_app_take_the_build_name() {
    assert_eq!(
        renamed("menus-about-herdr", "About Herdr".into(), "Orbita"),
        "About Orbita"
    );
    assert_eq!(renamed("menus-herdr", "Herdr".into(), "Orbita"), "Orbita");
}

#[test]
fn other_messages_keep_naming_herdr() {
    assert_eq!(
        renamed(
            "menus-qa-herdr-not-detected",
            "Show Herdr Not Detected".into(),
            "Orbita"
        ),
        "Show Herdr Not Detected"
    );
}

#[test]
fn shown_labels_carry_the_build_name() {
    assert_eq!(
        crate::i18n::english("menus-quit-herdr", &[]),
        format!("Quit {}", crate::WINDOW_TITLE)
    );
}
