#[cfg(feature = "gui")]
fn main() {
    slint_build::compile("src/gui/ui.slint").unwrap();
}

#[cfg(not(feature = "gui"))]
fn main() {}