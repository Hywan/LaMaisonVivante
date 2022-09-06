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
    const base_origin = new URL(base).origin;

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
    const loopRegex = /(?<item>[a-zA-Z\_]+) in (?<set>[a-zA-Z\_]+(\.[a-zA-Z\_]+)?)/;
    const removePrefix = function (prefix, value) {
        if ('' === prefix) {
            return value;
        }

        return value.replace(new RegExp(`^${prefix}`), '');
    };

    return function(data, root, keyPrefix) {
        keyPrefix = keyPrefix || '';

        let element;

        // Handle one loop at a time to allow proper embedded loops
        // computation.
        while (element = root.querySelector('[data-bind-loop]')) {
            let key = removePrefix(keyPrefix, element.dataset.bindLoop);
            delete element.dataset.bindLoop;

            let match = key.match(loopRegex);

            if (null === match) {
                console.error(`Loop format is invalid: \`${key}\``);

                return;
            }

            let { item: itemKey, set: setKey } = match.groups;
            setKey = removePrefix(keyPrefix, setKey);

            if (!(setKey in data)) {
                console.error(`Set key \`${setKey}\` is absent from the data`, data, element);

                return;
            }

            if (!(Symbol.iterator in data[setKey])) {
                console.error(`Set \`${setKey}\` is not an iterable object`, data, element);

                return;
            }

            const children = [];

            for (const datum of data[setKey]) {
                const newRoot = element.cloneNode(true);
                delete newRoot.dataset.bindLoop;

                render(datum, newRoot, `${itemKey}.`);
                children.push(newRoot);
            }

            element.replaceChildren(...children);
        }

        for (const element of root.querySelectorAll('[data-bind]')) {
            let key = removePrefix(keyPrefix, element.dataset.bind);
            delete element.dataset.bind;

            if (!(key in data)) {
                console.error(`Key \`${key}\` is absent from the data`, data, element);

                return;
            }

            element.innerHTML = data[key].toString();
        }
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

            const thing_primary_value_element = template_content.querySelector('.thing--solar-pv-primary-value');
            const thing_meter_circle_element = template_content.querySelector('.thing--solar-pv-meter .meter');
            const thing_sunrise_element = template_content.querySelector('.thing--solar-pv-sunrise');
            const thing_sunset_element = template_content.querySelector('.thing--solar-pv-sunset');
            const thing_sun_element = template_content.querySelector('.thing--solar-pv-sun');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const props = await properties_of(this, 'base', 'power');

            let previous_now = new Date(0);
            let sunrise = null;
            let sunset = null;

            async function update(next) {
                // Read all fetched properties.
                const values = await props.fetch_values();

                // Update `thing_primary_value_element`.
                const { formatted_value: power } = values.$get(props.names.power);
                thing_primary_value_element.innerHTML = power;

                // Update `thing_sunrise_element` + `thing_sunset_element`.
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

                thing_sunrise_element.innerHTML = sunrise.getHours() + ":" + number_to_2_chars(sunrise.getMinutes());
                thing_sunset_element.innerHTML = sunset.getHours() + ":" + number_to_2_chars(sunset.getMinutes());

                // Update `thing_sun_element`.

                /// No sun!
                if (now < sunrise || now > sunset) {
                    thing_sun_element.setAttribute('aria-hidden', true);
                    thing_frame.setAttribute('aria-disabled', true);
                }
                /// Position the sun.
                else {
                    thing_sun_element.setAttribute('aria-hidden', false);
                    thing_frame.setAttribute('aria-disabled', false);

                    let now_in_minutes = now.getHours() * 60 + now.getMinutes();
                    const min_sun = sunrise.getHours() * 60 + sunrise.getMinutes();
                    const max_sun = sunset.getHours() * 60 + sunset.getMinutes();
                    const circle_length = thing_meter_circle_element.getTotalLength();
                    const min_circle = circle_length / 2;
                    const max_circle = circle_length;

                    const pos = value_into_range(now_in_minutes, min_sun, max_sun, min_circle, max_circle);

                    const pos_point = thing_meter_circle_element.getPointAtLength(pos);
                    thing_sun_element.setAttributeNS(null, "cx", pos_point.x);
                    thing_sun_element.setAttributeNS(null, "cy", pos_point.y);
                }

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

                const thing_temperature_element = template_content.querySelector('.thing--weather-temperature');
                const thing_apparent_temperature_element = template_content.querySelector('.thing--weather-apparent-temperature > span');
                const thing_condition_element = template_content.querySelector('.thing--weather-condition');
                const thing_condition_icon_element = template_content.querySelector('.thing--weather-condition-icon > img');
                const thing_now_temperature_element = template_content.querySelector('.thing--weather-now-temperature');
                const thing_now_apparent_temperature_element = template_content.querySelector('.thing--weather-now-apparent-temperature');
                const thing_now_condition_element = template_content.querySelector('.thing--weather-now-condition');
                const thing_now_condition_icon_element = template_content.querySelector('.thing--weather-now-condition-icon');
                const thing_now_precipitation_element = template_content.querySelector('.thing--weather-now-precipitation');
                const thing_now_uv_index_element = template_content.querySelector('.thing--weather-now-uv-index');
                const thing_now_humidity_element = template_content.querySelector('.thing--weather-now-humidity');
                const thing_now_dew_point_element = template_content.querySelector('.thing--weather-now-dew-point');
                const thing_wind_degree_element = template_content.querySelector('.thing--weather-wind-degree');
                const thing_wind_text_element = template_content.querySelector('.thing--weather-wind-text');
                const thing_forecast_element = template_content.querySelector('.thing--weather-forecast');

                this.attachShadow({mode: 'closed'})
                    .appendChild(template_content);

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
                    thing_temperature_element.innerHTML = temperature;
                    thing_apparent_temperature_element.innerHTML = apparent_temperature;
                    thing_condition_element.innerHTML = weather_condition.text;
                    thing_condition_icon_element.setAttribute('src', `static/icons/weather/${weather_condition.icon}.svg`);
                    thing_now_temperature_element.innerHTML = `Mesurée ${temperature}`;
                    thing_now_apparent_temperature_element.innerHTML = `Ressentie ${apparent_temperature}`;
                    thing_now_condition_element.innerHTML = weather_condition.text;
                    thing_now_condition_icon_element.setAttribute('src', `static/icons/weather/${weather_condition.icon}.svg`);
                    thing_now_precipitation_element.innerHTML = `${(rain + snow).round(2)}mm`;
                    thing_now_uv_index_element.innerHTML = uv_index.round(1);
                    thing_now_humidity_element.innerHTML = `${humidity.round(0)}%`;
                    thing_now_dew_point_element.innerHTML = `${dew_point.round(1)}°C`;
                    thing_wind_degree_element.style.transform = `rotate(${wind_degree + 180}deg)`;
                    thing_wind_text_element.innerHTML = `${wind_speed.round(1)}m/s<br /><abbr title="rafales">raf.</abbr> ${wind_gust.round(0)}m/s`;

                    let formatted_forecast = '';

                    const today = new Date();
                    today.setHours(0);
                    today.setMinutes(0);
                    today.setSeconds(0);
                    today.setMilliseconds(0);

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

                        formatted_forecast += `<div class="thing--weather-one-forecast" data-temperature-category="${Math.round(value_into_range(f.temperature, 0, 30, 0, 5))}">
  <h5 class="thing--weather-one-forecast--datetime">${date.getHours()}h${date_extra}</h5>
  <h6 class="thing--weather-one-forecast--title"><span>Ciel</span></h6>
  <div class="thing--weather-one-forecast--condition-icon"><img src="static/icons/weather/${conditions.icon}.svg" alt="condition icon" /></div>
  <div class="thing--weather-one-forecast--condition">${conditions.text}</div>
  <div class="thing--weather-one-forecast--cloudiness">${formatted_octas}</div>
  <div class="thing--weather-one-forecast--precipitations">${precipitations.round(2)}mm</div>

  <div class="thing--weather-one-forecast--uv-index">${f.uv_index.round(1)}UV<sub>ix</sub></div>
  <h6 class="thing--weather-one-forecast--title"><span>Températures</span></h6>
  <div class="thing--weather-one-forecast--temperature">${f.temperature.round(1)}°C</div>
  <div class="thing--weather-one-forecast--apparent-temperature">(${f.apparent_temperature.round(1)}°C)</div>
  <h6 class="thing--weather-one-forecast--title"><span>Air</span></h6>
  <div class="thing--weather-one-forecast--humidity">${f.humidity}%H</div>
  <div class="thing--weather-one-forecast--pressure">${f.pressure}hPa</div>
  <div class="thing--weather-one-forecast--dew-point">${f.dew_point.round(1)}°C</div>
  <h6 class="thing--weather-one-forecast--title"><span>Vent</span></h6>
  <div class="thing--weather-one-forecast--wind-speed">${f.wind_speed.round(1)}m/s</div>
  <div class="thing--weather-one-forecast--wind-gust">(${f.wind_gust.round(1)}m/s)</div>
  <div class="thing--weather-one-forecast--wind-degree"><svg class="icon" style="transform: rotate(${f.wind_degree + 180}deg)"><use href="#icon-compass" /></div>
</div>`;
                    }

                    thing_forecast_element.innerHTML = formatted_forecast;

                    next();
                }

                fire(VERY_LONG_REFRESH_RATE, update);
            }
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
