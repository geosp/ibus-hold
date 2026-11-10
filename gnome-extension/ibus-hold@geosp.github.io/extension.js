import Gio from 'gi://Gio';
import GLib from 'gi://GLib';
import GObject from 'gi://GObject';

import {Extension} from 'resource:///org/gnome/shell/extensions/extension.js';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import * as MessageTray from 'resource:///org/gnome/shell/ui/messageTray.js';
import * as QuickSettings from 'resource:///org/gnome/shell/ui/quickSettings.js';
import * as Keyboard from 'resource:///org/gnome/shell/ui/status/keyboard.js';

import {
    INSTALL_URL, findComponent, isHoldEngine, layoutOf, listEngines,
    pickEngine, withEngineReplaced, withSource,
} from './engines.js';

const HoldToggle = GObject.registerClass(
class HoldToggle extends QuickSettings.QuickToggle {
    _init() {
        super._init({
            title: 'Press and Hold',
            iconName: 'input-keyboard-symbolic',
            toggleMode: false,
        });
    }
});

// Shown once per session, not on every unlock.
let notifiedMissing = false;

export default class IBusHoldExtension extends Extension {
    enable() {
        this._settings = this.getSettings();
        this._inputSettings = new Gio.Settings({schema_id: 'org.gnome.desktop.input-sources'});
        this._manager = Keyboard.getInputSourceManager();
        this._pending = null;
        this._loadEngines();

        this._toggle = new HoldToggle();
        this._toggle.connect('clicked', () => this._onClicked());
        this._indicator = new QuickSettings.SystemIndicator();
        this._indicator.quickSettingsItems.push(this._toggle);
        Main.panel.statusArea.quickSettings.addExternalIndicator(this._indicator);

        this._manager.connectObject(
            'current-source-changed', () => this._sync(),
            'sources-changed', () => this._onSourcesChanged(),
            this);
        this._settings.connectObject('changed::engine', () => this._onEngineChanged(), this);

        if (!this._installed)
            this._notifyMissing(false);
        else if (this._settings.get_boolean('auto-enable'))
            this._autoEnable();
        this._sync();
    }

    disable() {
        this._manager.disconnectObject(this);
        this._settings.disconnectObject(this);
        this._cancelPending();
        this._indicator.quickSettingsItems.forEach(item => item.destroy());
        this._indicator.destroy();
        this._indicator = null;
        this._toggle = null;
        this._manager = null;
        this._settings = null;
        this._inputSettings = null;
        this._engines = null;
    }

    _loadEngines() {
        this._installed = findComponent() !== null;
        this._engines = this._installed ? listEngines() : [];
    }

    get _sources() {
        return this._inputSettings.get_value('sources').deepUnpack();
    }

    set _sources(sources) {
        this._inputSettings.set_value('sources', new GLib.Variant('a(ss)', sources));
    }

    get _engine() {
        return pickEngine(this._engines, this._settings.get_string('engine'), this._sources);
    }

    get _activeHold() {
        const current = this._manager.currentSource;
        return current?.type === 'ibus' && isHoldEngine(current.id) ? current.id : null;
    }

    _findSource(type, id) {
        return Object.values(this._manager.inputSources)
            .find(s => s.type === type && s.id === id) ?? null;
    }

    _cancelPending() {
        if (this._pending?.idleId)
            GLib.source_remove(this._pending.idleId);
        this._pending = null;
    }

    // Switch to a source. After the sources list changes GNOME first
    // reactivates the most recent source, so wait for that, then switch.
    _activate(type, id) {
        this._cancelPending();
        const source = this._findSource(type, id);
        if (source)
            source.activate(true);
        else
            this._pending = {type, id, idleId: 0};
    }

    _onSourcesChanged() {
        const pending = this._pending;
        if (pending && !pending.idleId && this._findSource(pending.type, pending.id)) {
            pending.idleId = GLib.idle_add(GLib.PRIORITY_DEFAULT, () => {
                this._pending = null;
                this._findSource(pending.type, pending.id)?.activate(true);
                return GLib.SOURCE_REMOVE;
            });
        }
        this._sync();
    }

    _autoEnable() {
        const sources = this._sources;
        if (sources.some(([type, id]) => type === 'ibus' && isHoldEngine(id)))
            return;
        const engine = this._engine;
        this._sources = withSource(sources, ['ibus', engine], 0);
        this._activate('ibus', engine);
    }

    _onClicked() {
        this._loadEngines();
        if (!this._installed) {
            Main.panel.closeQuickSettings();
            this._notifyMissing(true);
            return;
        }

        const active = this._activeHold;
        const sources = this._sources;
        if (active) {
            // Off: the plain keyboard layout this engine is based on.
            const xkb = ['xkb', layoutOf(this._engines, active)];
            const at = sources.findIndex(([type, id]) => type === 'ibus' && id === active);
            this._sources = withSource(sources, xkb, at + 1);
            this._activate(...xkb);
        } else {
            const engine = this._engine;
            this._sources = withSource(sources, ['ibus', engine]);
            this._activate('ibus', engine);
        }
    }

    _onEngineChanged() {
        if (!this._installed)
            return;
        const engine = this._engine;
        const wasActive = this._activeHold !== null;
        this._sources = withEngineReplaced(this._sources, engine);
        if (wasActive)
            this._activate('ibus', engine);
        this._sync();
    }

    _notifyMissing(force) {
        if (notifiedMissing && !force)
            return;
        notifiedMissing = true;

        const source = MessageTray.getSystemSource();
        const notification = new MessageTray.Notification({
            source,
            title: 'Press and Hold is not installed',
            body: 'Install the ibus-hold package, then log out and back in.',
        });
        notification.addAction('Install instructions', () => {
            Gio.AppInfo.launch_default_for_uri(INSTALL_URL, null);
        });
        source.addNotification(notification);
    }

    _sync() {
        if (!this._toggle)
            return;
        const active = this._activeHold;
        this._toggle.checked = active !== null;
        this._toggle.subtitle = this._installed
            ? layoutOf(this._engines, active ?? this._engine).toUpperCase()
            : 'Not installed';
    }
}
