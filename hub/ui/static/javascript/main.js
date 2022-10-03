const HOME_LATITUDE = 46.78657339107215;
const HOME_LONGITUDE = 6.806581635522576;
const REFRESH_RATE = 1000 * 10; // 10 secs
const LONG_REFRESH_RATE = 1000 * 60; // 1 minute
const VERY_LONG_REFRESH_RATE = 1000 * 60 * 60; // 1 hour

function http_get(url) {
    return fetch(url, {
        method: 'GET',
        cache: 'no-cache',
    })
}
function http_put(url, body) {
    return fetch(url, {
        method: 'PUT',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: body,
    })
}

function http_post(url, body) {
    return fetch(url, {
        method: 'POST',
        cache: 'no-cache',
        headers: {
            'Content-Type': 'application/json',
        },
        body: body,
    })
}

function number_to_2_chars(number) {
    if (number < 10) {
        return "0" + number;
    }

    return number;
}

function seconds_to_duration(seconds) {
    const hours = Math.floor(seconds / 3600);
    seconds = seconds % 3600;
    const minutes = Math.floor(seconds / 60);
    seconds = seconds % 60;

    let output = '';

    if (hours > 0) {
        output = `${hours}h `;
    }

    output = `${minutes}min`;

    if (seconds > 0) {
        let output = ` ${seconds}s`;
    }

    return output;
}

// Fire a function:
// * immediately,
// * every `timeout`,
// * when the document is back to visible.
function fire(timeout, func, ...args) {
    let timeout_id = null;

    function next(...new_args) {
        if (timeout_id != null) {
            window.clearTimeout(timeout_id);
        }

        timeout_id = window.setTimeout(
            () => {
                func(next, ...new_args);
            },
            timeout,
            false,
        );
    }

    // Fire `func` if the document's visibility has changed to
    // `visible`. Otherwise, cancel the timeout set by `next`.
    document.addEventListener(
        'visibilitychange',
        () => {
            if (document.visibilityState == 'visible') {
                func(next, ...args)
            } else {
                if (timeout_id != null) {
                    window.clearTimeout(timeout_id);
                }
            }
        }
    );

    // Fire `func` immediately.
    func(next, ...args);
}

async function properties_of(element, property_name_of_base, ...attributes) {
    const names = read_data_attributes(element, ...attributes);
    const fetched = await fetch_properties(
        element.getAttribute(`data-${property_name_of_base}`),
        ...Object.values(names)
    );

    return {
        names,
        fetch_values: async function() {
            return await fetched.read();
        }
    };
}

function read_data_attributes(element, ...attributes) {
    const out = {};

    for (attribute of attributes) {
        out[attribute.replace(/-/g, '_')] = element.getAttribute(`data-${attribute}`);
    }

    return out;
}

const SCHEMAS_CACHE = {};

async function fetch_properties(base, ...property_names) {
    base = base.replace(/\/+$/, '');

    if (SCHEMAS_CACHE[base] == undefined) {
        const raw_schema = await http_get(base);
        const schema = await raw_schema.json();

        SCHEMAS_CACHE[base] = schema;
    }

    const schema = SCHEMAS_CACHE[base];

    let properties_values = {};

    async function refresh_properties_values() {
        const raw_properties_values = await http_get(base + '/properties');
        properties_values = await raw_properties_values.json();
    }

    let properties = {};

    for (const property_name of property_names) {
        if (undefined == property_name) {
            continue;
        }

        const property_description = schema.properties[property_name];

        let value_reader;
        const extra_values = {};

        switch (property_description.type) {
        case 'boolean': {
            value_reader = function () {
                const value = properties_values[property_name];

                return {value};
            };

            break;
        }

        case 'integer':
        case 'number': {
            const unit = property_description.unit;
            let min = 0;
            let max = null;

            if (property_description.minimum) {
                min = property_description.minimum;
            }

            if (property_description.maximum) {
                max = property_description.maximum;
            }

            extra_values.min = min;
            extra_values.max = max;

            value_reader = function () {
                const value = properties_values[property_name];
                let formatted_value = value.round(2);

                switch (unit) {
                case 'percent':
                    formatted_value += '%';
                    break;

                case 'watt':
                    formatted_value += 'W';
                    break;

                case 'ampere':
                    formatted_value += 'A';
                    break;

                case 'celsius':
                    formatted_value = Math.round(formatted_value) + '°C';
                    break;
                }

                return {
                    value,
                    formatted_value,
                };
            };

            break;
        }

        case 'string': {
            value_reader = function () {
                const value = properties_values[property_name];

                return {value};
            };

            break;
        }

        case 'object': {
            value_reader = function() {
                const value = properties_values[property_name];

                return {value};
            };

            break;
        }

        // Non-standard.
        case 'recurrence': {
            const regex = /RRULE:FREQ=(?<freq>[A-Z]+);INTERVAL=(?<interval>[0-9]+);(?<by>[A-Z]+)=(?<by_value>[A-Z]+);AT=(?<at>[0-9]+)/;

            value_reader = function() {
                const value = properties_values[property_name];
                let formatted_value;
                const matches = value.match(regex);

                if (matches) {
                    const m = matches.groups;

                    if ('OFF' == m.freq) {
                        formatted_value = 'jamais';
                    } else {
                        formatted_value = `chaque ${m.freq == 'MONTHLY' ? 'mois' : (m.freq == 'WEEKLY' ? 'semaine' : '(inconnu)')}`;

                        if ('BYDAY' == m.by) {
                            let day = '';

                            switch (m.by_value) {
                            case 'MO':
                                day = 'lundi';
                                break;

                            case 'TU':
                                day = 'mardi';
                                break;

                            case 'WE':
                                day = 'mercredi';
                                break;

                            case 'TH':
                                day = 'jeudi';
                                break;

                            case 'FR':
                                day = 'vendredi';
                                break;

                            case 'SA':
                                day = 'samedi';
                                break;

                            case 'SU':
                                day = 'dimanche';
                                break;

                            default:
                                day = '(inconnu)';
                            }

                            formatted_value += `, le ${day} à ${m.at}h`;
                        }
                    }

                    return {
                        value,
                        formatted_value,
                    };
                }
            }
        }
        }

        properties[property_name] = {
            value_reader,
            ...extra_values
        };
    }

    // Helper to replace:
    //     (properties[property_name].value_reader)()
    // by
    //     properties.$get(property_name)
    properties.$get = function (property_name) {
        return (properties[property_name].value_reader)();
    };

    return {
        async read() {
            await refresh_properties_values();

            return properties;
        }
    };
}

const render = new function() {
    const LOOP_REGEX = /^(?<item_name>[a-zA-Z_]+) in (?<set_name>[a-zA-Z_]+(\.[a-zA-Z_]+)?)$/;
    const ATTRIBUTE_PREFIX = 'data-bind:';
    const restore_data_bindings = new function () {
        const deferred = [];

        return {
            defer: function (element, binding_value, func) {
                deferred.push({element, binding_value, func});
            },

            now: function () {
                for (const {element, binding_value, func} of deferred) {
                    (func)(element, binding_value);
                }

                deferred.length = 0;
            },
        };
    };

    function remove_prefix(prefix, value) {
        if ('' === prefix) {
            return value;
        }

        return value.replace(new RegExp(`^${prefix}`), '');
    }

    function render_bind_loop(data, root, key_prefix) {
        let element;

        // Handle one loop at a time to allow proper embedded loops
        // computation.
        while (element = root.querySelector('[data-bind-loop]')) {
            let key = element.dataset.bindLoop;
            delete element.dataset.bindLoop;

            restore_data_bindings.defer(element, key, (element, key) => element.dataset.bindLoop = key);

            key = remove_prefix(key_prefix, key);

            let match = key.match(LOOP_REGEX);

            if (null === match) {
                console.error(`Loop format is invalid: \`${key}\``);
                restore_data_bindings.now();

                return;
            }

            let { item_name, set_name } = match.groups;
            set_name = remove_prefix(key_prefix, set_name);

            if (!(set_name in data)) {
                console.error(`Set key \`${set_name}\` is absent from the data`, data, element);
                restore_data_bindings.now();

                return;
            }

            if (!(Symbol.iterator in data[set_name])) {
                console.error(`Set \`${set_name}\` is not an iterable object`, data, element);
                restore_data_bindings.now();

                return;
            }

            const children = [];

            for (const datum of data[set_name]) {
                const next_root = element.cloneNode(true);
                const next_key_prefix = `${item_name}.`;

                render_all(datum, next_root, next_key_prefix);

                children.push(next_root);
            }

            element.replaceWith(...children);
        }
    }

    function render_bind(data, root, key_prefix) {
        const elements = [...root.querySelectorAll('[data-bind]')];

        if (root.dataset && root.dataset.bind) {
            elements.push(root);
        }

        for (const element of elements) {
            let key = element.dataset.bind;
            delete element.dataset.bind;

            restore_data_bindings.defer(element, key, (element, key) => element.dataset.bind = key);

            key = remove_prefix(key_prefix, key);

            if (!(key in data)) {
                console.error(`Key \`${key}\` is absent from the data`, data, element);
                restore_data_bindings.now();

                return;
            }

            element.innerHTML = data[key].toString();
        }
    }

    function render_bind_attribute(data, root, key_prefix) {
        const elements = [...root.querySelectorAll('[data-bind-attributes]')];

        if (root.dataset && undefined !== root.dataset.bindAttributes) {
            elements.push(root);
        }

        for (const element of elements) {
            delete element.dataset.bindAttributes;

            restore_data_bindings.defer(element, '', (element, key) => element.dataset.bindAttributes = key);

            const attributes = Array.from(element.attributes)
                  .filter(node => node.nodeName.startsWith(ATTRIBUTE_PREFIX))
                  .reduce(
                      (object, node) => ({
                          ...object,
                          [node.nodeName.slice(ATTRIBUTE_PREFIX.length)]: node.nodeValue
                      }),
                      {}
                  );

            for (let [attribute_name, key] of Object.entries(attributes)) {
                key = remove_prefix(key_prefix, key);

                if (!(key in data)) {
                    console.error(`Key \`${key}\` is absent from the data`, data, element);
                    restore_data_bindings.now();

                    return;
                }

                element.setAttribute(attribute_name, data[key].toString());
            }
        }
    }

    function render_all(data, root, key_prefix) {
        key_prefix = key_prefix || '';

        render_bind_loop(data, root, key_prefix);
        render_bind(data, root, key_prefix);
        render_bind_attribute(data, root, key_prefix);
    };

    return function(data, root) {
        restore_data_bindings.now();
        render_all(data, root);
    };
};

function value_into_range(value, from_range_min, from_range_max, to_range_min, to_range_max) {
    let new_value = Math.min(Math.max(value, from_range_min), from_range_max);

    new_value = (
        ((new_value - from_range_min) * (to_range_max - to_range_min)) / (from_range_max - from_range_min)
    ) + to_range_min;

    return new_value;
}

Number.prototype.round = function (precision) {
    precision = Math.pow(10, precision);

    return Math.round((this + Number.EPSILON) * precision) / precision;
};

window.customElements.define(
    'my-nav',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            let template = document.getElementById('template--nav');
            let template_content = template.content.cloneNode(true);

            this.attachShadow({mode: 'open'})
                .appendChild(template_content);
        }

        enter(name, onclick) {
            let root = this.shadowRoot;

            var link = document.createElement('a');
            link.setAttribute('href', '#');
            link.addEventListener('click', onclick);
            link.appendChild(document.createTextNode(name));

            var item = document.createElement('li');
            item.appendChild(link);

            let list = root.querySelector('ol');
            list.appendChild(item);
        }

        leave() {
            let last = this.shadowRoot.querySelector('li:last-child');

            if (last) {
                last.parentNode.removeChild(last);
            }
        }
    }
);

window.customElements.define(
    'my-things',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            let template = document.getElementById('template--things');
            let template_content = template.content.cloneNode(true);

            this.attachShadow({mode: 'closed'})
                .appendChild(template_content);
        }
    }
);

window.customElements.define(
    'my-unlocated-things',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            let template = document.getElementById('template--unlocated-things');
            let template_content = template.content.cloneNode(true);

            this.attachShadow({mode: 'closed'})
                .appendChild(template_content);
        }
    }
);

window.customElements.define(
    'my-thing',
    new function() {
        let thing_index = 0;

        return class extends HTMLElement {
            constructor() {
                super();
            }

            connectedCallback() {
                const template = document.getElementById('template--thing');
                const template_content = template.content.cloneNode(true);

                const thing = template_content.querySelector('.thing');
                thing.setAttribute('id', 'thing-' + thing_index);
                thing_index += 1;

                const shadow_root = this.attachShadow({mode: 'open'})
                      .appendChild(template_content);
            }
        };
    }
);

window.customElements.define(
    'my-expandable-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            const template = document.getElementById('template--expandable-thing');
            const template_content = template.content.cloneNode(true);

            const thing = template_content.querySelector('.thing--expandable');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const self = this;

            thing.querySelector('.thing--expandable-summary').addEventListener(
                'click',
                () => {
                    const nav = document.getElementById('nav');
                    const leaving = () => {
                        thing.setAttribute('aria-expanded', 'false')
                        nav.leave();
                    };

                    if (thing.getAttribute('aria-expanded') == 'false') {
                        thing.setAttribute('aria-expanded', 'true');
                        nav.enter('Tous les objets', leaving);
                    }
                }
            );
        }
    }
);

window.customElements.define(
    'my-text-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--text-thing');
            const template_content = template.content.cloneNode(true);

            this.attachShadow({mode: 'open'}).appendChild(template_content);
        }
    }
);

window.customElements.define(
    'my-meter-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--meter-thing');
            const template_content = template.content.cloneNode(true);

            const thing_primary_value_element = template_content.querySelector('.thing--meter-primary-value');
            const thing_secondary_value_element = template_content.querySelector('.thing--meter-secondary-value');
            const thing_meter_circle_element = template_content.querySelector('.thing--meter-meter .meter');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const circle_length = thing_meter_circle_element.getTotalLength();

            const props = await properties_of(this, 'base', 'primary', 'secondary');

            async function update(next) {
                // Read all fetched properties.
                const values = await props.fetch_values();

                async function subupdate(
                    property_name,
                    thing_value_element,
                    do_update_thing_meter_circle_element
                ) {
                    const prop = values[property_name];

                    const max = prop.max;
                    const {value, formatted_value} = (prop.value_reader)();
                    thing_value_element.innerHTML = formatted_value;

                    if (do_update_thing_meter_circle_element) {
                        if (null != max) {
                            const percent = (value * circle_length) / max;
                            thing_meter_circle_element.style.strokeDasharray = percent + ' 100';
                        } else {
                            thing_meter_circle_element.style.strokeDasharray = '100 100';
                        }
                    }
                }

                // Update values.

                subupdate(props.names.primary, thing_primary_value_element, true);

                if (undefined != props.names.secondary) {
                    subupdate(props.names.secondary, thing_secondary_value_element, false);
                }

                next();
            }

            fire(REFRESH_RATE, update);

            if (undefined == props.names.secondary) {
                thing_primary_value_element.classList.add('thing--meter-primary-value-large');
            }
        }
    }
);

window.customElements.define(
    'my-solar-pv-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--solar-pv-thing');
            const template_content = template.content.cloneNode(true);

            const thing_frame = template_content.querySelector('.thing--frame');
            const thing_meter_circle_element = template_content.querySelector('.thing--solar-pv-meter .meter');

            this.attachShadow({mode: 'open'}).appendChild(template_content);
            const root = this.shadowRoot;

            const props = await properties_of(this, 'base', 'power');

            let previous_now = new Date(0);
            let sunrise = null;
            let sunset = null;

            async function update(next) {
                // Read all fetched properties.
                const values = await props.fetch_values();
                const { formatted_value: power } = values.$get(props.names.power);

                let now = new Date();

                /// The day has changed.
                if (previous_now.getDate() != now.getDate() || sunrise == null || sunset == null) {
                    previous_now = now;

                    let {
                        sunrise: next_sunrise,
                        sunset: next_sunset
                    } = sunrise_sunset(
                        HOME_LATITUDE,
                        HOME_LONGITUDE,
                        now.getFullYear(),
                        now.getMonth() + 1,
                        now.getDate()
                    );

                    sunrise = next_sunrise;
                    sunset = next_sunset;
                }

                const data = {
                    power,
                    sunrise: sunrise.getHours() + ":" + number_to_2_chars(sunrise.getMinutes()),
                    sunset: sunset.getHours() + ":" + number_to_2_chars(sunset.getMinutes()),
                    sun_cx: 0,
                    sun_cy: 0,
                    sun_hidden: true,
                };

                /// No sun!
                if (now < sunrise || now > sunset) {
                    thing_frame.setAttribute('aria-disabled', true);
                }
                /// Position the sun.
                else {
                    thing_frame.setAttribute('aria-disabled', false);

                    let now_in_minutes = now.getHours() * 60 + now.getMinutes();
                    const min_sun = sunrise.getHours() * 60 + sunrise.getMinutes();
                    const max_sun = sunset.getHours() * 60 + sunset.getMinutes();
                    const circle_length = thing_meter_circle_element.getTotalLength();
                    const min_circle = circle_length / 2;
                    const max_circle = circle_length;

                    const pos = value_into_range(now_in_minutes, min_sun, max_sun, min_circle, max_circle);

                    const pos_point = thing_meter_circle_element.getPointAtLength(pos);
                    data.sun_hidden = false;
                    data.sun_cx = pos_point.x;
                    data.sun_cy = pos_point.y;
                }

                render(data, root);

                next();
            }

            fire(REFRESH_RATE, update);
        }
    }
);

window.customElements.define(
    'my-dhw-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--dhw-thing');
            const template_content = template.content.cloneNode(true);

            const thing_top_value_element = template_content.querySelector('.thing--dhw-top-value');
            const thing_bottom_value_element = template_content.querySelector('.thing--dhw-bottom-value');
            const thing_wanted_value_element = template_content.querySelector('.thing--dhw-wanted-value');
            const thing_anti_legionella_started_manually_value_element = template_content.querySelector('.thing--dhw-anti-legionella-started-manually-value');
            const thing_anti_legionella_schedule_value_element = template_content.querySelector('.thing--dhw-anti-legionella-schedule-value');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const props = await properties_of(
                this,
                'base',
                'top',
                'bottom',
                'wanted',
                'anti-legionella-started-manually',
                'anti-legionella-schedule',
            );

            async function update(next) {
                // Read all fetched properties.
                const values = await props.fetch_values();

                // Get formatted values.
                const { formatted_value: top_formatted } = values.$get(props.names.top);
                const { formatted_value: bottom_formatted } = values.$get(props.names.bottom);
                const { formatted_value: wanted_formatted } = values.$get(props.names.wanted);
                const { value: anti_legionella_started_manually } = values.$get(props.names.anti_legionella_started_manually);
                const { formatted_value: anti_legionella_schedule } = values.$get(props.names.anti_legionella_schedule);

                // Update values.
                thing_top_value_element.innerHTML = top_formatted;
                thing_bottom_value_element.innerHTML = bottom_formatted;
                thing_wanted_value_element.innerHTML = wanted_formatted;

                if (anti_legionella_started_manually) {
                    thing_anti_legionella_started_manually_value_element.innerHTML = 'oui';
                } else {
                    thing_anti_legionella_started_manually_value_element.innerHTML = 'non';
                }

                thing_anti_legionella_schedule_value_element.innerHTML = anti_legionella_schedule;

                next();
            }

            fire(LONG_REFRESH_RATE, update);
        }
    }
);

window.customElements.define(
    'my-ventilation-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--ventilation-thing');
            const template_content = template.content.cloneNode(true);

            const thing_frame = template_content.querySelector('.thing--frame');

            const thing_after_ground_coupled_heat_exchanger_element = template_content.querySelector('.thing--ventilation-after-ground-coupled-heat-exchanger');
            const thing_after_heat_recovery_exchanger_element = template_content.querySelector('.thing--ventilation-after-heat-recovery-exchanger');
            const thing_extracted_element = template_content.querySelector('.thing--ventilation-extracted');

            const thing_after_ground_coupled_heat_exchanger_meter_element = template_content.querySelector('.meter--ventilation-after-ground-coupled-heat-exchanger');
            const thing_after_heat_recovery_exchanger_meter_element = template_content.querySelector('.meter--ventilation-after-heat-recovery-exchanger');
            const thing_extracted_meter_element = template_content.querySelector('.meter--ventilation-extracted');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const props = await properties_of(
                this,
                'base',
                'state',
                'after-ground-coupled-heat-exchanger',
                'after-heat-recovery-exchanger',
                'extracted',
            );

            const MAX_TEMPERATURE = 25;
            const MARGIN = 0.75; // in percent

            async function update(next) {
                // Read all properties.
                const values = await props.fetch_values();

                async function subupdate(property_name, element, meter_element) {
                    let {value, formatted_value} = values.$get(property_name);
                    element.innerHTML = formatted_value;

                    value = Math.min(value, MAX_TEMPERATURE);
                    let max_length = meter_element.getTotalLength();

                    meter_element.style.strokeDasharray = (value * (max_length * MARGIN)) / MAX_TEMPERATURE + ' ' + max_length;
                }

                // Update values.

                subupdate(
                    props.names.after_ground_coupled_heat_exchanger,
                    thing_after_ground_coupled_heat_exchanger_element,
                    thing_after_ground_coupled_heat_exchanger_meter_element,
                );

                subupdate(
                    props.names.after_heat_recovery_exchanger,
                    thing_after_heat_recovery_exchanger_element,
                    thing_after_heat_recovery_exchanger_meter_element,
                );

                subupdate(
                    props.names.extracted,
                    thing_extracted_element,
                    thing_extracted_meter_element,
                );

                let { value: state } = values.$get(props.names.state);

                if ('paused' == state) {
                    thing_frame.setAttribute('aria-disabled', true);
                } else {
                    thing_frame.setAttribute('aria-disabled', false);
                }

                next();
            }

            fire(LONG_REFRESH_RATE, update);
        }
    }
);

window.customElements.define(
    'my-weather-thing',
    new function() {
        const WEATHER_CONDITIONS = {
            0: { text: '(inconnue)', icon: '' },

            200: { text: 'Orage avec légère pluie', icon: '11d' },
            201: { text: 'Orage avec pluie', icon: '11d' },
            202: { text: 'Orage avec pluie importante', icon: '11d' },
            210: { text: 'Orage léger', icon: '11d' },
            211: { text: 'Orage', icon: '11d' },
            212: { text: 'Orage important', icon: '11d' },
            221: { text: 'Orage violent', icon: '11d' },
            230: { text: 'Orage avec bruine légère', icon: '11d' },
            231: { text: 'Orange avec bruine', icon: '11d' },
            232: { text: 'Orage avec bruine importante', icon: '11d' },

            300: { text: 'Légère bruine', icon: '09d' },
            301: { text: 'Bruine', icon: '09d' },
            302: { text: 'Bruine dense', icon: '09d' },
            310: { text: 'Pluie légère', icon: '09d' },
            311: { text: 'Pluie légère', icon: '09d' },
            312: { text: 'Pluie légère', icon: '09d' },
            313: { text: 'Douche de bruine', icon: '09d' },
            314: { text: 'Douche de bruine', icon: '09d' },
            321: { text: 'Douche de bruine', icon: '09d' },

            500: { text: 'Légère pluie', icon: '10d' },
            501: { text: 'Pluie modérée', icon: '10d' },
            502: { text: 'Pluie intense', icon: '10d' },
            503: { text: 'La douche', icon: '10d' },
            504: { text: 'Pluie extrême', icon: '10d' },
            511: { text: 'Pluie glaçante', icon: '13d' },
            520: { text: 'Pluie légère', icon: '09d' },
            521: { text: 'Pluie dense', icon: '09d' },
            522: { text: 'Pluie dense', icon: '09d' },
            531: { text: 'Pluie éparse', icon: '09d' },

            600: { text: 'Légère neige', icon: '13d' },
            601: { text: 'Neige', icon: '13d' },
            602: { text: 'Neige intense', icon: '13d' },
            611: { text: 'Neige fondue', icon: '13d' },
            612: { text: 'Légère neige fondue', icon: '13d' },
            613: { text: 'Neige fondue intense', icon: '13d' },
            615: { text: 'Légère pluie et neige', icon: '13d' },
            616: { text: 'Pluie et neige', icon: '13d' },
            620: { text: 'Neige', icon: '13d' },
            621: { text: 'Neige', icon: '13d' },
            622: { text: 'Neige intense', icon: '13d' },

            701: { text: 'Brume', icon: '50d' },
            711: { text: 'Brume intense', icon: '50d' },
            721: { text: 'Brouillard', icon: '50d' },
            731: { text: 'Tourbillon de poussières', icon: '50d' },
            741: { text: 'Brouillard', icon: '50d' },
            751: { text: 'Sable', icon: '50d' },
            761: { text: 'Poussière', icon: '50d' },
            762: { text: 'Cendres volcanique', icon: '50d' },
            771: { text: 'Bourrasques', icon: '50d' },
            781: { text: 'Tonarde', icon: '50d' },

            800: { text: 'Ciel dégagé', icon: '01d' },
            801: { text: 'Partiellement nuageux', icon: '02d' },
            802: { text: 'Parsemé de nuages', icon: '03d' },
            803: { text: 'Nuageux', icon: '04d' },
            804: { text: 'Couvert', icon: '04d' },
        };

        return class extends HTMLElement {
            constructor() {
                super();
            }

            async connectedCallback() {
                let template = document.getElementById('template--weather-thing');
                let template_content = template.content.cloneNode(true);

                // Insert the OpenWeatherMap API key.
                {
                    const all_srcs = template_content.querySelectorAll('[src*="{openweathermap_api_key}"]');

                    all_srcs.forEach((element) => {
                        element.setAttribute(
                            'src',
                            element.getAttribute('src').replace(
                                /\{openweathermap_api_key\}/,
                                OPENWEATHERMAP_API_KEY,
                            )
                        );
                    });
                }

                const thing_frame = template_content.querySelector('.thing--frame');

                this.attachShadow({mode: 'open'}).appendChild(template_content);
                const root = this.shadowRoot;

                const props = await properties_of(
                    this,
                    'base',
                    'temperature',
                    'apparent-temperature',
                    'condition',
                    'wind-degree',
                    'wind-speed',
                    'wind-gust',
                    'rain',
                    'snow',
                    'uv-index',
                    'humidity',
                    'dew-point',
                );
                const forecast_props = await properties_of(this, 'forecast-base', 'forecast');

                async function update(next) {
                    // Read all fetched properties.
                    const values = await props.fetch_values();
                    const forecast_values = await forecast_props.fetch_values();

                    // Get values.
                    const { formatted_value: temperature } = values.$get(props.names.temperature);
                    const { formatted_value: apparent_temperature } = values.$get(props.names.apparent_temperature);
                    const { value: condition } = values.$get(props.names.condition);
                    const { value: wind_degree } = values.$get(props.names.wind_degree);
                    const { formatted_value: wind_speed } = values.$get(props.names.wind_speed);
                    const { formatted_value: wind_gust } = values.$get(props.names.wind_gust);
                    const { value: rain } = values.$get(props.names.rain);
                    const { value: snow } = values.$get(props.names.snow);
                    const { value: uv_index } = values.$get(props.names.uv_index);
                    const { value: humidity } = values.$get(props.names.humidity);
                    const { value: dew_point } = values.$get(props.names.dew_point);
                    const { value: forecast } = forecast_values.$get(forecast_props.names.forecast);

                    const weather_condition = WEATHER_CONDITIONS[condition] || WEATHER_CONDITIONS[0];

                    const today = new Date();
                    today.setHours(0);
                    today.setMinutes(0);
                    today.setSeconds(0);
                    today.setMilliseconds(0);

                    const forecast_view_data = [];

                    for (const f of forecast) {
                        const date = adjust_time_to_local(f.datetime * 1000);
                        const conditions = WEATHER_CONDITIONS[f.conditions[0].id] || WEATHER_CONDITIONS[0];

                        let date_extra = '';

                        if (today.getDate() != date.getDate()) {
                            date_extra = ` <small>(+${Math.floor((date - today) / (1000 * 60 * 60 * 24))}j)</small>`;
                        }

                        let octas = Math.floor(f.clouds / 12.5);
                        let formatted_octas = `${octas} octa`;

                        if (octas > 1) {
                            formatted_octas += 's';
                        }

                        const precipitations = (f.rain || f.snow || {one_hour: 0}).one_hour;

                        forecast_view_data.push({
                            temperature_category: Math.round(value_into_range(f.temperature, 0, 30, 0, 5)),
                            date_hour: date.getHours(),
                            date_extra,
                            condition_icon: `static/icons/weather/symbols.svg#${conditions.icon}`,
                            condition: conditions.text,
                            octas: formatted_octas,
                            precipitations: precipitations.round(2),
                            uv_index: f.uv_index.round(1),
                            temperature: f.temperature.round(1),
                            apparent_temperature: f.apparent_temperature.round(1),
                            humidity: f.humidity,
                            pressure: f.pressure,
                            dew_point: f.dew_point.round(1),
                            wind: f.wind_speed.round(1),
                            wind_gust: f.wind_gust.round(1),
                            wind_degree: `transform: rotate(${f.wind_degree}deg)`,
                        });
                    }

                    render(
                        {
                            temperature,
                            apparent_temperature,
                            condition: weather_condition.text,
                            condition_icon: `static/icons/weather/symbols.svg#${weather_condition.icon}`,
                            precipitation: (rain + snow).round(2),
                            uv_index: uv_index.round(1),
                            wind: wind_speed.round(1),
                            wind_gust: wind_gust.round(0),
                            wind_degree: `transform: rotate(${wind_degree + 180}deg) transform-origin: 50% 50%`,
                            humidity: humidity.round(0),
                            dew_point: dew_point.round(1),
                            forecasts: forecast_view_data,
                        },
                        root
                    );

                    next();
                }

                fire(VERY_LONG_REFRESH_RATE, update);
            }
        }
    }
);

window.customElements.define(
    'my-car-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--car-thing');
            const template_content = template.content.cloneNode(true);

            const short_thing = template_content.querySelector('[slot="short-thing"]');
            const long_thing = template_content.querySelector('[slot="long-thing"]');

            // Set properties of `my-meter-thing` before it's attached to the DOM.
            const data_properties = read_data_attributes(this, 'battery-base', 'battery-primary');
            render({...data_properties}, short_thing);

            this.attachShadow({mode: 'open'}).appendChild(template_content);
            const root = this.shadowRoot;

            const { json: json_url } = read_data_attributes(this, 'json');
            const raw_json = await http_get(json_url);
            const json = await raw_json.json();

            const { description, state } = json;
            const { status, odometer, location } = state;
            const { longitude, latitude } = location.coordinates;

            render(
                {
                    vehicle_nickname: description.nickname,
                    vehicle_vin: description.vin,
                    battery: status.battery.state_of_charge,
                    range: status.battery.remaining_range,
                    is_charging: status.battery.is_charging ? 'en charge' : 'débranché',
                    estimated_charging_duration: seconds_to_duration(state.status.battery.estimated_charging_duration.secs),
                    odometer: odometer.round(0),
                    location_static_map: `https://api.mapbox.com/styles/v1/mapbox/streets-v11/static/pin-l+a12b20(${longitude},${latitude})/${longitude},${latitude},16,0/300x300@2x?access_token=pk.eyJ1IjoiaHl3YW4iLCJhIjoiY2w4cG9sNDcwMTJ0cjNvbzVrYXMyd2VibCJ9.d2BSDWYAxe3w0-w7-tzBZQ`,
                    location_map: `https://www.openstreetmap.org/?mlat=${latitude}&mlon=${longitude}#map=14/${latitude}/${longitude}`,
                    targeted_temperature: `${status.targeted_temperature.round(1)}°C`,
                    is_defrost_enabled: status.is_defrost_enabled ? 'activé' : 'désactivé',
                    is_locked: status.is_locked ? 'fermée' : 'ouverte',
                    is_front_left_door_opened: status.doors.is_front_left_opened,
                    is_back_left_door_opened: status.doors.is_back_left_opened,
                    is_front_right_door_opened: status.doors.is_front_right_opened,
                    is_back_right_door_opened: status.doors.is_back_right_opened,
                    is_front_left_window_opened: status.windows.is_front_left_opened,
                    is_back_left_window_opened: status.windows.is_back_left_opened,
                    is_front_right_window_opened: status.windows.is_front_right_opened,
                    is_back_right_window_opened: status.windows.is_back_right_opened,
                    is_trunk_opened: status.is_trunk_opened,
                    is_frunk_opened: status.is_frunk_opened,
                    is_steer_wheel_heat_enabled: status.is_steer_wheel_heat_enabled ? 'chauffant' : 'normal',
                    is_air_climate_enabled: status.is_air_climate_enabled ? 'activée' : 'désactivée',
                    is_side_back_window_heat_enabled: status.is_side_back_window_heat_enabled ? 'chauffants' : 'normaux',
                },
                long_thing,
            );
        }
    }
);

window.customElements.define(
    'my-actionable-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--actionable-thing');
            const template_content = template.content.cloneNode(true);

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);
        }
    }
);

window.customElements.define(
    'my-thing--pulse',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            const template = document.getElementById('template--thing-pulse');
            const template_content = template.content.cloneNode(true);

            const button = template_content.querySelector('.thing--pulse');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const inactive_for = parseInt(self.getAttribute('data-inactive-for') || 0) * 1000;
            const url = base + '/properties/pulse';

            button.addEventListener(
                'click',
                () => {
                    button.setAttribute('disabled', true);
                    window.setTimeout(
                        () => {
                            button.removeAttribute('disabled');
                        },
                        inactive_for,
                        false
                    )

                    http_put(url, '{"pulse": true}')
                }
            );
        }
    }
);

window.customElements.define(
    'my-fancy-icon',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            const template = document.getElementById('template--fancy-icon');
            const template_content = template.content.cloneNode(true);

            this.attachShadow({mode: 'open'}).appendChild(template_content);
            const root = this.shadowRoot;

            const href = this.getAttribute('href');
            const filler_class = this.getAttribute('filler-class') || 'gradient gradient--linear__default';

            render({href, filler_class}, root);
        }
    }
);

window.customElements.define(
    'my-thing--blind',
    class extends HTMLElement {
        constructor() {
            super();
        }

        connectedCallback() {
            const template = document.getElementById('template--thing-blind');
            const template_content = template.content.cloneNode(true);

            const open_button = template_content.querySelector('.thing--blind-open');
            const stop_button = template_content.querySelector('.thing--blind-stop');
            const close_button = template_content.querySelector('.thing--blind-close');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const open_url = base + '/actions/open';
            const stop_url = base + '/actions/stop';
            const close_url = base + '/actions/close';

            open_button.addEventListener(
                'click',
                () => {
                    http_post(open_url, '{"open": {}}')
                }
            );
            stop_button.addEventListener(
                'click',
                () => {
                    http_post(stop_url, '{"stop": {}}')
                }
            );
            close_button.addEventListener(
                'click',
                () => {
                    http_post(close_url, '{"close": {}}')
                }
            );
        }
    }
);

// Once the DOM is ready.
window.addEventListener(
    'DOMContentLoaded',
    () => {
        // Implement tabs for the navigation.
        {
            const all_tablists = document.querySelectorAll('[role="tablist"]');

            all_tablists.forEach(
                (tablist) => {
                    const all_tabs = tablist.querySelectorAll('[role="tab"]');

                    all_tabs.forEach(
                        (tab) => {
                            tab.addEventListener(
                                'click',
                                () => {
                                    const is_not_selected = tab.getAttribute('aria-selected') == "false";

                                    if (is_not_selected) {
                                        all_tabs.forEach(
                                            (tab) => {
                                                tab.setAttribute('aria-selected', 'false');
                                                document.getElementById(tab.getAttribute('aria-controls')).setAttribute('aria-hidden', 'true');
                                            }
                                        );
                                        tab.setAttribute('aria-selected', 'true');
                                        document.getElementById(tab.getAttribute('aria-controls')).setAttribute('aria-hidden', 'false');
                                    }
                                }
                            );
                        }
                    );
                }
            );
        }
    }
);
