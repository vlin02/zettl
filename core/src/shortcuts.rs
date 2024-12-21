
enum Modifier {
  Command,
  Control,
  Shift,
  Option,
}

#[derive(serde_json::Serialize)]
struct Shortcut {
  modifiers: Vec<Modifier>,
  key: char
}