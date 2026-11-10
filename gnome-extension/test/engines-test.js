// Run with: gjs -m test/engines-test.js (or `make test`)
import GLib from 'gi://GLib';
import System from 'system';

import {
    isHoldEngine, layoutOf, parseDelay, parseEngines, pickEngine, setDelay,
    withEngineReplaced, withSource,
} from '../ibus-hold@geosp.github.io/engines.js';

let failures = 0;
function check(name, actual, expected) {
    const a = JSON.stringify(actual), e = JSON.stringify(expected);
    if (a !== e) {
        failures++;
        print(`FAIL ${name}\n  got:      ${a}\n  expected: ${e}`);
    }
}

const here = GLib.path_get_dirname(GLib.filename_from_uri(import.meta.url)[0]);
const [, bytes] = GLib.file_get_contents(
    GLib.build_filenamev([here, '..', '..', 'ibus-hold', 'data', 'ibus-hold.xml']));
const engines = parseEngines(new TextDecoder().decode(bytes));

check('engine count', engines.length, 17);
check('first engine', engines[0], {name: 'hold', layout: 'us', language: 'en'});
check('german engine', engines.find(e => e.layout === 'de')?.name, 'hold:de');

check('isHoldEngine hold', isHoldEngine('hold'), true);
check('isHoldEngine hold:de', isHoldEngine('hold:de'), true);
check('isHoldEngine other', isHoldEngine('holdx'), false);

check('pick configured', pickEngine(engines, 'hold:fr', []), 'hold:fr');
check('pick automatic', pickEngine(engines, '', [['xkb', 'de'], ['xkb', 'us']]), 'hold:de');
check('pick unknown configured', pickEngine(engines, 'hold:xx', [['xkb', 'gb']]), 'hold:gb');
check('pick fallback', pickEngine(engines, '', [['xkb', 'jp']]), 'hold');
check('pick no xkb', pickEngine(engines, '', []), 'hold');

check('layoutOf', layoutOf(engines, 'hold:latam'), 'latam');
check('layoutOf unknown', layoutOf(engines, 'nope'), 'us');

const us = [['xkb', 'us']];
check('withSource front', withSource(us, ['ibus', 'hold'], 0), [['ibus', 'hold'], ['xkb', 'us']]);
check('withSource end', withSource(us, ['ibus', 'hold']), [['xkb', 'us'], ['ibus', 'hold']]);
check('withSource present', withSource(us, ['xkb', 'us'], 0), us);

const withHold = [['xkb', 'us'], ['ibus', 'hold']];
check('replace', withEngineReplaced(withHold, 'hold:de'), [['xkb', 'us'], ['ibus', 'hold:de']]);
check('replace present', withEngineReplaced(withHold, 'hold'), withHold);
check('replace none', withEngineReplaced(us, 'hold:de'), us);

check('delay default', parseDelay(''), 500);
check('delay set', parseDelay('# c\ndelay 300\ne é\n'), 300);
check('delay last wins', parseDelay('delay 300\ndelay 700\n'), 700);
check('delay invalid', parseDelay('delay soon\n'), 500);

check('setDelay empty', setDelay('', 400),
    '# delay before the popup opens, in milliseconds\ndelay 400\n');
check('setDelay replace', setDelay('# c\ndelay 300\ne é\n', 400), '# c\ndelay 400\ne é\n');
check('setDelay dedupe', setDelay('delay 1\nx\ndelay 2\n', 400), 'delay 400\nx\n');
check('setDelay append', setDelay('e é è', 400),
    '# delay before the popup opens, in milliseconds\ndelay 400\n\ne é è\n');
check('setDelay roundtrip', parseDelay(setDelay('e é\ndelay 900', 650)), 650);

if (failures) {
    print(`${failures} failure(s)`);
    System.exit(1);
}
print('all engines.js tests passed');
