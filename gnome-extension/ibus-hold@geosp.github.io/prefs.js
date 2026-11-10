import Adw from 'gi://Adw';
import Gio from 'gi://Gio';
import GnomeDesktop from 'gi://GnomeDesktop?version=4.0';
import Gtk from 'gi://Gtk';

import {ExtensionPreferences} from 'resource:///org/gnome/Shell/Extensions/js/extensions/prefs.js';

import {
    DEFAULT_DELAY_MS, INSTALL_URL, findComponent, listEngines, readDelay, writeDelay,
} from './engines.js';

export default class IBusHoldPreferences extends ExtensionPreferences {
    fillPreferencesWindow(window) {
        const settings = this.getSettings();
        const page = new Adw.PreferencesPage();
        window.add(page);

        if (!findComponent())
            page.add(this._missingGroup());
        page.add(this._sourceGroup(settings));
        page.add(this._timingGroup(window));
    }

    _missingGroup() {
        const group = new Adw.PreferencesGroup();
        const row = new Adw.ActionRow({
            title: 'ibus-hold is not installed',
            subtitle: 'Install the package, then log out and back in.',
        });
        row.add_suffix(new Gtk.LinkButton({
            label: 'Install instructions',
            uri: INSTALL_URL,
            valign: Gtk.Align.CENTER,
        }));
        group.add(row);
        return group;
    }

    _sourceGroup(settings) {
        const group = new Adw.PreferencesGroup({title: 'Input Source'});

        const xkb = new GnomeDesktop.XkbInfo();
        const label = layout => {
            const [ok, name] = xkb.get_layout_info(layout);
            return ok ? name : layout;
        };
        const engines = listEngines()
            .map(e => ({name: e.name, label: label(e.layout)}))
            .sort((a, b) => a.label.localeCompare(b.label));
        const names = ['', ...engines.map(e => e.name)];

        const layoutRow = new Adw.ComboRow({
            title: 'Keyboard layout',
            subtitle: 'Layout used with Press and Hold',
            model: Gtk.StringList.new(['Automatic', ...engines.map(e => e.label)]),
            selected: Math.max(0, names.indexOf(settings.get_string('engine'))),
            sensitive: engines.length > 0,
        });
        layoutRow.connect('notify::selected', () => {
            settings.set_string('engine', names[layoutRow.selected] ?? '');
        });
        group.add(layoutRow);

        const autoRow = new Adw.SwitchRow({
            title: 'Add automatically',
            subtitle: 'Add Press and Hold to the input sources if it is missing',
        });
        settings.bind('auto-enable', autoRow, 'active', Gio.SettingsBindFlags.DEFAULT);
        group.add(autoRow);

        return group;
    }

    _timingGroup(window) {
        const group = new Adw.PreferencesGroup({
            title: 'Timing',
            description: 'Saved to ~/.config/ibus-hold/config',
        });

        const delayRow = Adw.SpinRow.new_with_range(100, 2000, 50);
        delayRow.title = 'Hold delay';
        delayRow.subtitle = `Milliseconds before the accent popup opens (default ${DEFAULT_DELAY_MS})`;
        delayRow.value = readDelay();
        delayRow.connect('notify::value', () => {
            try {
                writeDelay(delayRow.value);
            } catch (e) {
                window.add_toast(new Adw.Toast({title: `Could not save: ${e.message}`}));
            }
        });
        group.add(delayRow);

        const restartRow = new Adw.ActionRow({
            title: 'Apply changes',
            subtitle: 'The engine reads its settings when IBus starts',
        });
        const button = new Gtk.Button({label: 'Restart IBus', valign: Gtk.Align.CENTER});
        button.connect('clicked', () => {
            try {
                Gio.Subprocess.new(['ibus', 'restart'], Gio.SubprocessFlags.NONE);
                window.add_toast(new Adw.Toast({title: 'IBus restarted'}));
            } catch (e) {
                window.add_toast(new Adw.Toast({title: `Could not restart IBus: ${e.message}`}));
            }
        });
        restartRow.add_suffix(button);
        group.add(restartRow);

        return group;
    }
}
