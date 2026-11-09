//! Press-and-hold state machine.
//!
//! Pure logic, independent of D-Bus: key events go in, UI [`Action`]s come
//! out. The engine (`engine.rs`) turns those actions into ibus signals.
//!
//! ```text
//!            accentable key down            timer fired (still held)
//!   Idle ───────────────────────▶ Pending ─────────────────────────▶ Popup
//!    ▲                               │                                 │
//!    │   key up: commit base letter  │   digit / Enter: commit choice  │
//!    └───────────────────────────────┴─────────────────────────────────┘
//! ```
use crate::accents::Accents;

/// A key, reduced to what the state machine cares about.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// A printable character (letters, punctuation, ...), excluding ASCII digits
    Char(char),
    /// An ASCII digit `0` ~ `9`
    Digit(u8),
    Left,
    Right,
    Up,
    Down,
    Enter,
    Escape,
    BackSpace,
    /// Anything else (modifier keys, function keys, ...)
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyEvent {
    pub key: Key,
    /// Physical key position; used to match a release with its press, because
    /// the keysym can change in between (e.g. Shift released before `E`).
    pub keycode: u32,
    /// `true` for key press, `false` for key release
    pub down: bool,
    /// Ctrl, Alt, Super, ... are held (Shift does not count)
    pub special_mods: bool,
}

/// Something the engine must show or do.
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Show this text as preedit (not yet typed into the application)
    SetPreedit(String),
    /// Hide the preedit text
    ClearPreedit,
    /// Type this text into the application
    Commit(String),
    /// Show (or update) the candidate popup
    ShowTable {
        candidates: Vec<String>,
        cursor: usize,
    },
    /// Hide the candidate popup
    HideTable,
    /// Call [`HoldState::timer_fired`] with this generation after the hold delay
    StartTimer(u64),
}

/// Result of processing one key event.
#[derive(Debug, Clone, PartialEq)]
pub struct Outcome {
    /// `true`: the key was consumed; `false`: pass it on to the application
    pub captured: bool,
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, PartialEq)]
enum Mode {
    Idle,
    /// An accentable key is down; its letter is shown as preedit.
    Pending {
        base: char,
        keycode: u32,
    },
    /// The candidate popup is open.
    Popup {
        base: char,
        keycode: u32,
        /// the key that opened the popup is still physically held
        held: bool,
        candidates: Vec<String>,
        cursor: usize,
    },
}

#[derive(Debug, Clone)]
pub struct HoldState {
    accents: Accents,
    mode: Mode,
    /// Incremented on every new press; lets stale timers be ignored.
    generation: u64,
}

impl HoldState {
    pub fn new(accents: Accents) -> Self {
        Self {
            accents,
            mode: Mode::Idle,
            generation: 0,
        }
    }

    /// Process a key press or release.
    pub fn key(&mut self, ev: KeyEvent) -> Outcome {
        let mut actions = Vec::new();
        let mode = std::mem::replace(&mut self.mode, Mode::Idle);

        let captured = match mode {
            Mode::Idle => self.idle_key(ev, &mut actions),

            Mode::Pending { base, keycode } => {
                if ev.keycode == keycode {
                    if ev.down {
                        // autorepeat while held: swallow
                        self.mode = Mode::Pending { base, keycode };
                    } else {
                        // released before the delay: a normal tap
                        actions.push(Action::ClearPreedit);
                        actions.push(Action::Commit(base.to_string()));
                    }
                    true
                } else if !ev.down {
                    // release of some other key (e.g. Shift): ignore
                    self.mode = Mode::Pending { base, keycode };
                    false
                } else {
                    // another key pressed (fast typing / rollover):
                    // type the pending letter, then handle the new key normally
                    actions.push(Action::ClearPreedit);
                    actions.push(Action::Commit(base.to_string()));
                    self.idle_key(ev, &mut actions)
                }
            }

            Mode::Popup {
                base,
                keycode,
                held,
                candidates,
                cursor,
            } => {
                if held && ev.keycode == keycode {
                    // autorepeat or release of the key that opened the popup:
                    // the popup stays open
                    self.mode = Mode::Popup {
                        base,
                        keycode,
                        held: ev.down,
                        candidates,
                        cursor,
                    };
                    true
                } else if !ev.down {
                    self.mode = Mode::Popup {
                        base,
                        keycode,
                        held,
                        candidates,
                        cursor,
                    };
                    false
                } else {
                    self.popup_key(ev, base, keycode, held, candidates, cursor, &mut actions)
                }
            }
        };

        Outcome { captured, actions }
    }

    fn idle_key(&mut self, ev: KeyEvent, actions: &mut Vec<Action>) -> bool {
        if let Key::Char(c) = ev.key
            && ev.down
            && !ev.special_mods
            && self.accents.contains_key(&c)
        {
            self.generation += 1;
            self.mode = Mode::Pending {
                base: c,
                keycode: ev.keycode,
            };
            actions.push(Action::SetPreedit(c.to_string()));
            actions.push(Action::StartTimer(self.generation));
            return true;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn popup_key(
        &mut self,
        ev: KeyEvent,
        base: char,
        keycode: u32,
        held: bool,
        candidates: Vec<String>,
        cursor: usize,
        actions: &mut Vec<Action>,
    ) -> bool {
        if ev.special_mods {
            // e.g. Ctrl+S while the popup is open: type the plain letter and
            // let the shortcut through
            close(actions, base.to_string());
            return false;
        }

        let len = candidates.len();
        let keep_open = |cursor: usize, actions: &mut Vec<Action>, this: &mut Self| {
            actions.push(Action::ShowTable {
                candidates: candidates.clone(),
                cursor,
            });
            this.mode = Mode::Popup {
                base,
                keycode,
                held,
                candidates: candidates.clone(),
                cursor,
            };
        };

        match ev.key {
            Key::Digit(d) if d >= 1 && (d as usize) <= len => {
                close(actions, candidates[d as usize - 1].clone());
                true
            }
            Key::Left | Key::Up => {
                keep_open((cursor + len - 1) % len, actions, self);
                true
            }
            Key::Right | Key::Down => {
                keep_open((cursor + 1) % len, actions, self);
                true
            }
            Key::Enter => {
                close(actions, candidates[cursor].clone());
                true
            }
            Key::Escape => {
                // cancel: keep the plain letter
                close(actions, base.to_string());
                true
            }
            Key::BackSpace => {
                // delete the letter entirely
                actions.push(Action::HideTable);
                actions.push(Action::ClearPreedit);
                true
            }
            Key::Digit(_) => {
                // out-of-range digit: ignore, keep the popup
                keep_open(cursor, actions, self);
                true
            }
            _ => {
                // any other key: type the plain letter, then handle the key
                close(actions, base.to_string());
                self.idle_key(ev, actions)
            }
        }
    }

    /// The hold delay elapsed for the press with this `generation`.
    pub fn timer_fired(&mut self, generation: u64) -> Vec<Action> {
        match &self.mode {
            Mode::Pending { base, keycode } if generation == self.generation => {
                let (base, keycode) = (*base, *keycode);
                let candidates = self.accents[&base].clone();
                self.mode = Mode::Popup {
                    base,
                    keycode,
                    held: true,
                    candidates: candidates.clone(),
                    cursor: 0,
                };
                vec![Action::ShowTable {
                    candidates,
                    cursor: 0,
                }]
            }
            // stale timer: the key was released, or another press started
            _ => vec![],
        }
    }

    /// A candidate was clicked in the popup (`index` within the popup).
    pub fn select(&mut self, index: usize) -> Vec<Action> {
        if let Mode::Popup { candidates, .. } = &self.mode
            && let Some(c) = candidates.get(index)
        {
            let mut actions = Vec::new();
            close(&mut actions, c.clone());
            self.mode = Mode::Idle;
            return actions;
        }
        vec![]
    }

    /// Drop all state without typing anything (focus out, reset, disable).
    ///
    /// The pending preedit uses commit-on-focus-out mode, so ibus itself
    /// types the plain letter when focus is lost.
    pub fn clear(&mut self) -> Vec<Action> {
        self.generation += 1;
        let actions = match self.mode {
            Mode::Idle => vec![],
            Mode::Pending { .. } => vec![Action::ClearPreedit],
            Mode::Popup { .. } => vec![Action::HideTable, Action::ClearPreedit],
        };
        self.mode = Mode::Idle;
        actions
    }
}

fn close(actions: &mut Vec<Action>, text: String) {
    actions.push(Action::HideTable);
    actions.push(Action::ClearPreedit);
    actions.push(Action::Commit(text));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::accents::builtin;

    const KC_E: u32 = 26;
    const KC_T: u32 = 28;
    const KC_SHIFT: u32 = 50;

    fn down(key: Key, keycode: u32) -> KeyEvent {
        KeyEvent {
            key,
            keycode,
            down: true,
            special_mods: false,
        }
    }

    fn up(key: Key, keycode: u32) -> KeyEvent {
        KeyEvent {
            down: false,
            ..down(key, keycode)
        }
    }

    fn commits(actions: &[Action]) -> Vec<String> {
        actions
            .iter()
            .filter_map(|a| match a {
                Action::Commit(t) => Some(t.clone()),
                _ => None,
            })
            .collect()
    }

    /// press `e` and let the timer fire; returns the state with the popup open
    fn open_popup() -> HoldState {
        let mut s = HoldState::new(builtin());
        let o = s.key(down(Key::Char('e'), KC_E));
        assert_eq!(o.actions.last(), Some(&Action::StartTimer(1)));
        let a = s.timer_fired(1);
        assert!(matches!(a[0], Action::ShowTable { cursor: 0, .. }));
        s
    }

    #[test]
    fn tap_types_the_letter() {
        let mut s = HoldState::new(builtin());
        let o = s.key(down(Key::Char('e'), KC_E));
        assert!(o.captured);
        assert_eq!(o.actions[0], Action::SetPreedit("e".into()));

        let o = s.key(up(Key::Char('e'), KC_E));
        assert!(o.captured);
        assert_eq!(commits(&o.actions), vec!["e"]);

        // the timer of that press must now do nothing
        assert!(s.timer_fired(1).is_empty());
    }

    #[test]
    fn non_accentable_keys_pass_through() {
        let mut s = HoldState::new(builtin());
        let o = s.key(down(Key::Char('t'), KC_T));
        assert!(!o.captured);
        assert!(o.actions.is_empty());
    }

    #[test]
    fn modifier_combos_pass_through() {
        let mut s = HoldState::new(builtin());
        let mut ev = down(Key::Char('e'), KC_E);
        ev.special_mods = true; // e.g. Ctrl+E
        let o = s.key(ev);
        assert!(!o.captured);
        assert!(o.actions.is_empty());
    }

    #[test]
    fn autorepeat_is_swallowed() {
        let mut s = HoldState::new(builtin());
        s.key(down(Key::Char('e'), KC_E));
        let o = s.key(down(Key::Char('e'), KC_E));
        assert!(o.captured);
        assert!(o.actions.is_empty());
    }

    #[test]
    fn hold_then_digit_picks_accent() {
        let mut s = open_popup();
        // autorepeat and release of the held key keep the popup open
        assert!(s.key(down(Key::Char('e'), KC_E)).captured);
        assert!(s.key(up(Key::Char('e'), KC_E)).captured);

        let o = s.key(down(Key::Digit(2), 11));
        assert!(o.captured);
        assert_eq!(commits(&o.actions), vec![builtin()[&'e'][1].clone()]);
        assert!(o.actions.contains(&Action::HideTable));
    }

    #[test]
    fn arrows_and_enter() {
        let mut s = open_popup();
        let o = s.key(down(Key::Right, 114));
        assert!(matches!(o.actions[0], Action::ShowTable { cursor: 1, .. }));
        let o = s.key(down(Key::Left, 113));
        assert!(matches!(o.actions[0], Action::ShowTable { cursor: 0, .. }));
        // wraps around
        let o = s.key(down(Key::Left, 113));
        let n = builtin()[&'e'].len();
        assert!(matches!(o.actions[0], Action::ShowTable { cursor, .. } if cursor == n - 1));

        let o = s.key(down(Key::Enter, 36));
        assert_eq!(commits(&o.actions), vec![builtin()[&'e'][n - 1].clone()]);
    }

    #[test]
    fn escape_keeps_plain_letter() {
        let mut s = open_popup();
        let o = s.key(down(Key::Escape, 9));
        assert!(o.captured);
        assert_eq!(commits(&o.actions), vec!["e"]);
    }

    #[test]
    fn backspace_drops_letter() {
        let mut s = open_popup();
        let o = s.key(down(Key::BackSpace, 22));
        assert!(o.captured);
        assert!(commits(&o.actions).is_empty());
    }

    #[test]
    fn other_key_in_popup_types_letter_and_passes_key() {
        let mut s = open_popup();
        let o = s.key(down(Key::Char('t'), KC_T));
        assert!(!o.captured);
        assert_eq!(commits(&o.actions), vec!["e"]);
    }

    #[test]
    fn rollover_commits_pending_letter_first() {
        let mut s = HoldState::new(builtin());
        s.key(down(Key::Char('e'), KC_E));
        // `t` pressed before `e` is released
        let o = s.key(down(Key::Char('t'), KC_T));
        assert!(!o.captured);
        assert_eq!(commits(&o.actions), vec!["e"]);
        // the late release of `e` passes through harmlessly
        assert!(!s.key(up(Key::Char('e'), KC_E)).captured);
        assert!(s.timer_fired(1).is_empty());
    }

    #[test]
    fn rollover_into_another_accentable_letter() {
        let mut s = HoldState::new(builtin());
        s.key(down(Key::Char('e'), KC_E));
        let o = s.key(down(Key::Char('a'), 38));
        assert!(o.captured);
        assert_eq!(commits(&o.actions), vec!["e"]);
        assert_eq!(o.actions.last(), Some(&Action::StartTimer(2)));
        // only the newest timer opens a popup
        assert!(s.timer_fired(1).is_empty());
        assert!(!s.timer_fired(2).is_empty());
    }

    #[test]
    fn shift_release_before_letter_release() {
        let mut s = HoldState::new(builtin());
        s.key(down(Key::Char('E'), KC_E));
        assert!(!s.key(up(Key::Other, KC_SHIFT)).captured);
        // the release now reports `e`, but the keycode still matches
        let o = s.key(up(Key::Char('e'), KC_E));
        assert_eq!(commits(&o.actions), vec!["E"]);
    }

    #[test]
    fn clear_drops_everything() {
        let mut s = open_popup();
        assert_eq!(s.clear(), vec![Action::HideTable, Action::ClearPreedit]);
        assert!(s.select(0).is_empty());
        assert!(!s.key(down(Key::Char('t'), KC_T)).captured);
    }

    #[test]
    fn click_selects_candidate() {
        let mut s = open_popup();
        let a = s.select(0);
        assert_eq!(commits(&a), vec![builtin()[&'e'][0].clone()]);
    }
}
