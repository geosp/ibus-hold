// Helpers shared by extension.js and prefs.js. Only GLib/Gio, so this module
// also loads in the preferences process.
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';

export const DEFAULT_ENGINE = 'hold';
export const DEFAULT_DELAY_MS = 500;
export const INSTALL_URL =
    'https://github.com/geosp/ibus-hold/blob/main/doc/ibus-install.md';

const COMPONENT = 'ibus-hold.xml';

export function isHoldEngine(id) {
    return id === DEFAULT_ENGINE || id.startsWith(`${DEFAULT_ENGINE}:`);
}

function readText(path) {
    try {
        const [, bytes] = GLib.file_get_contents(path);
        return new TextDecoder().decode(bytes);
    } catch {
        return null;
    }
}

// Same places IBus looks for components.
function componentDirs() {
    const dirs = (GLib.getenv('IBUS_COMPONENT_PATH') ?? '').split(':').filter(d => d);
    for (const dir of GLib.get_system_data_dirs())
        dirs.push(GLib.build_filenamev([dir, 'ibus', 'component']));
    return dirs;
}

/** Path of the installed ibus-hold component XML, or null if not installed. */
export function findComponent() {
    for (const dir of componentDirs()) {
        const path = GLib.build_filenamev([dir, COMPONENT]);
        if (GLib.file_test(path, GLib.FileTest.EXISTS))
            return path;
    }
    return null;
}

/** Engines declared in the component XML: [{name, layout, language}]. */
export function parseEngines(xml) {
    const tag = (block, name) =>
        block.match(new RegExp(`<${name}>([^<]*)</${name}>`))?.[1].trim() ?? '';
    return [...xml.matchAll(/<engine>([\s\S]*?)<\/engine>/g)]
        .map(([, block]) => ({
            name: tag(block, 'name'),
            layout: tag(block, 'layout'),
            language: tag(block, 'language'),
        }))
        .filter(e => isHoldEngine(e.name) && e.layout);
}

export function listEngines(path = findComponent()) {
    const xml = path ? readText(path) : null;
    return xml ? parseEngines(xml) : [];
}

/**
 * The engine to use: the configured one if it exists, otherwise ("automatic")
 * the one matching the first keyboard layout in `sources`, otherwise US.
 * `sources` is the org.gnome.desktop.input-sources list: [[type, id], ...].
 */
export function pickEngine(engines, configured, sources) {
    if (configured && engines.some(e => e.name === configured))
        return configured;
    const [, layout] = sources.find(([type]) => type === 'xkb') ?? [];
    return engines.find(e => e.layout === layout)?.name ?? DEFAULT_ENGINE;
}

export function layoutOf(engines, name) {
    return engines.find(e => e.name === name)?.layout ?? 'us';
}

const sameSource = (a, b) => a[0] === b[0] && a[1] === b[1];

/** `sources` with `entry` inserted at `index`, unless already present. */
export function withSource(sources, entry, index = sources.length) {
    if (sources.some(s => sameSource(s, entry)))
        return sources;
    const copy = [...sources];
    copy.splice(index, 0, entry);
    return copy;
}

/**
 * Switch the first Press and Hold source to `engine`, keeping its position.
 * Returns `sources` unchanged if there is none or `engine` is already listed.
 */
export function withEngineReplaced(sources, engine) {
    const i = sources.findIndex(([type, id]) => type === 'ibus' && isHoldEngine(id));
    if (i < 0 || sources.some(s => sameSource(s, ['ibus', engine])))
        return sources;
    const copy = [...sources];
    copy[i] = ['ibus', engine];
    return copy;
}

// ~/.config/ibus-hold/config, read by the engine at startup.

export function configPath() {
    return GLib.build_filenamev([GLib.get_user_config_dir(), 'ibus-hold', 'config']);
}

const DELAY_LINE = /^\s*delay(\s+\S*)?\s*$/;

/** The `delay` from config text (last one wins, like the engine). */
export function parseDelay(text) {
    let delay = DEFAULT_DELAY_MS;
    for (const line of text.split('\n')) {
        const m = line.match(/^\s*delay\s+(\d+)\s*$/);
        if (m)
            delay = Number(m[1]);
    }
    return delay;
}

/** Config text with its `delay` set to `ms`; other lines are kept. */
export function setDelay(text, ms) {
    const lines = text.split('\n');
    const first = lines.findIndex(l => DELAY_LINE.test(l));
    if (first < 0) {
        const rest = text.trim() ? `\n${text.replace(/\n*$/, '\n')}` : '';
        return `# delay before the popup opens, in milliseconds\ndelay ${ms}\n${rest}`;
    }
    lines[first] = `delay ${ms}`;
    return lines.filter((l, i) => i === first || !DELAY_LINE.test(l)).join('\n');
}

export function readDelay() {
    return parseDelay(readText(configPath()) ?? '');
}

export function writeDelay(ms) {
    const path = configPath();
    GLib.mkdir_with_parents(GLib.path_get_dirname(path), 0o755);
    const text = setDelay(readText(path) ?? '', ms);
    Gio.File.new_for_path(path).replace_contents(
        new TextEncoder().encode(text), null, false,
        Gio.FileCreateFlags.REPLACE_DESTINATION, null);
}
