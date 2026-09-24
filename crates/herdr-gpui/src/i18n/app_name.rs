#[cfg(test)]
mod tests;

const NAMING_THE_APP: &[&str] = &[
    "commands-about",
    "menus-about-herdr",
    "menus-herdr",
    "menus-hide-herdr",
    "menus-quit-herdr",
];

pub(super) fn named(id: &str, text: String) -> String {
    renamed(id, text, crate::WINDOW_TITLE)
}

fn renamed(id: &str, text: String, name: &str) -> String {
    if NAMING_THE_APP.contains(&id) {
        text.replace("Herdr", name)
    } else {
        text
    }
}
