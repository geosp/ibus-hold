//! IBus glue: feeds key events into [`HoldState`] and turns its
//! [`Action`]s into ibus signals.
use std::sync::Arc;
use std::time::Duration;

use ibus_engine::ibus::{
    IBusEngine, IBusEngineBackend, IBusFactory, IBusModifierState, IBusPreeditFocusMode,
    LookupTable,
};
use log::{debug, warn};
use tokio::sync::Mutex;
use tokio::time::sleep;
use xkeysym::{KeyCode, Keysym};
use zbus::{ObjectServer, fdo, object_server::SignalEmitter};

use crate::accents::{Config, MAX_CANDIDATES};
use crate::state::{Action, HoldState, Key, KeyEvent};

#[derive(Debug, Clone)]
pub struct HoldEngine {
    state: Arc<Mutex<HoldState>>,
    delay: Duration,
}

impl HoldEngine {
    pub fn new(config: &Config) -> Self {
        Self {
            state: Arc::new(Mutex::new(HoldState::new(config.accents.clone()))),
            delay: config.delay,
        }
    }

    /// Apply actions produced by a key event; may start the hold timer.
    async fn apply(&self, se: &SignalEmitter<'_>, actions: Vec<Action>) {
        for a in actions {
            if let Action::StartTimer(generation) = a {
                self.start_timer(se, generation);
            } else {
                emit(se, a).await;
            }
        }
    }

    fn start_timer(&self, se: &SignalEmitter<'_>, generation: u64) {
        let se = se.to_owned();
        let state = self.state.clone();
        let delay = self.delay;
        tokio::spawn(async move {
            sleep(delay).await;
            // hold the lock while emitting, so signals stay in order with key events
            let mut s = state.lock().await;
            for a in s.timer_fired(generation) {
                emit(&se, a).await;
            }
        });
    }

    async fn clear(&self, se: &SignalEmitter<'_>) {
        let mut s = self.state.lock().await;
        let actions = s.clear();
        self.apply(se, actions).await;
    }
}

/// Emit one UI action as an ibus signal. Errors are logged, not returned:
/// a failed UI update must not break typing.
async fn emit(se: &SignalEmitter<'_>, action: Action) {
    let r = match action {
        Action::SetPreedit(text) => {
            let cursor = text.chars().count() as u32;
            // `Commit`: if focus is lost, ibus types the pending letter
            HoldEngine::update_preedit_text(se, text, cursor, true, IBusPreeditFocusMode::Commit)
                .await
        }
        Action::ClearPreedit => {
            HoldEngine::update_preedit_text(
                se,
                String::new(),
                0,
                false,
                IBusPreeditFocusMode::Commit,
            )
            .await
        }
        Action::Commit(text) => {
            debug!("commit {text:?}");
            HoldEngine::commit_text(se, text).await
        }
        Action::ShowTable { candidates, cursor } => {
            match LookupTable::new(candidates, MAX_CANDIDATES as u32, true, true) {
                Ok(mut table) => {
                    table.set_cursor_pos(cursor as i64);
                    HoldEngine::update_lookup_table(se, &table, true).await
                }
                Err(()) => Ok(()),
            }
        }
        Action::HideTable => match LookupTable::new(vec![], 1, false, false) {
            Ok(table) => HoldEngine::update_lookup_table(se, &table, false).await,
            Err(()) => Ok(()),
        },
        Action::StartTimer(_) => Ok(()),
    };
    if let Err(e) = r {
        warn!("ibus signal failed: {e}");
    }
}

fn classify(keyval: Keysym) -> Key {
    match keyval {
        Keysym::Left => Key::Left,
        Keysym::Right => Key::Right,
        Keysym::Up => Key::Up,
        Keysym::Down => Key::Down,
        Keysym::Return | Keysym::KP_Enter => Key::Enter,
        Keysym::Escape => Key::Escape,
        Keysym::BackSpace => Key::BackSpace,
        _ => match keyval.key_char() {
            Some(c) if c.is_ascii_digit() => Key::Digit(c as u8 - b'0'),
            Some(c) if !c.is_control() => Key::Char(c),
            _ => Key::Other,
        },
    }
}

impl IBusEngine for HoldEngine {
    async fn process_key_event(
        &mut self,
        se: SignalEmitter<'_>,
        _server: &ObjectServer,
        keyval: Keysym,
        keycode: KeyCode,
        state: IBusModifierState,
    ) -> fdo::Result<bool> {
        let ev = KeyEvent {
            key: classify(keyval),
            keycode: keycode.raw(),
            down: state.is_keydown(),
            special_mods: state.has_special_modifiers(),
        };
        let mut s = self.state.lock().await;
        let out = s.key(ev);
        self.apply(&se, out.actions).await;
        Ok(out.captured)
    }

    async fn focus_out(
        &mut self,
        se: SignalEmitter<'_>,
        _server: &ObjectServer,
    ) -> fdo::Result<()> {
        self.clear(&se).await;
        Ok(())
    }

    async fn reset(&mut self, se: SignalEmitter<'_>, _server: &ObjectServer) -> fdo::Result<()> {
        self.clear(&se).await;
        Ok(())
    }

    async fn disable(&mut self, se: SignalEmitter<'_>, _server: &ObjectServer) -> fdo::Result<()> {
        self.clear(&se).await;
        Ok(())
    }

    async fn candidate_clicked(
        &mut self,
        se: SignalEmitter<'_>,
        _server: &ObjectServer,
        index: u32,
        _button: u32,
        _state: u32,
    ) -> fdo::Result<()> {
        let mut s = self.state.lock().await;
        let actions = s.select(index as usize);
        self.apply(&se, actions).await;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct HoldFactory {
    config: Config,
}

impl HoldFactory {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl IBusFactory<HoldEngine> for HoldFactory {
    fn create_engine(&mut self, name: String) -> Result<HoldEngine, String> {
        // every layout variant behaves the same; only GNOME's keymap differs
        if crate::layouts::is_engine(&name) {
            Ok(HoldEngine::new(&self.config))
        } else {
            Err(format!("unknown engine: {name}"))
        }
    }
}
