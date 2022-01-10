const HOME_LATITUDE = 46.78657339107215;
const HOME_LONGITUDE = 6.806581635522576;
const REFRESH_RATE = 1000 * 10; // 10 secs

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

async function read_property(base, property_name) {
    const base_origin = new URL(base).origin;
    const property_response = await http_get(base);
    const property_json_response = await property_response.json();

    const property_description = property_json_response.properties[property_name];
    const unit = property_description.unit;
    const link = property_description.links[0].href;
    let min = 0;
    let max = null;

    if (property_description.minimum) {
        min = property_description.minimum;
    }

    if (property_description.maximum) {
        max = property_description.maximum;
    }

    return {
        link,
        unit,
        min,
        max,
        value_reader: async function () {
            const response = await http_get(base_origin + link);
            const json_response = await response.json();
            const value = json_response[property_name];
            let formatted_value = Math.round((value + Number.EPSILON) * 100) / 100;

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
                formatted_value += '°C';
                break;
            }

            return {
                value,
                formatted_value,
            };
        },
    };
}

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

            thing.querySelector('.thing--expander').addEventListener(
                'click',
                () => {
                    const nav = document.getElementById('nav');
                    const leaving = () => {
                        thing.setAttribute('aria-expanded', 'false')
                        nav.leave();
                    };

                    if (thing.getAttribute('aria-expanded') == 'false') {
                        const thing_name = self.querySelector('span[slot="name"]').textContent;

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

            async function update(
                next,
                thing_value_element,
                property_value_reader,
                property_link,
                property_min,
                property_max,
                do_update_thing_meter_circle_element
            ) {
                const {value, formatted_value} = await property_value_reader();

                thing_value_element.innerHTML = formatted_value;

                if (do_update_thing_meter_circle_element) {
                    if (property_max != null) {
                        const percent = (value * 100) / property_max;
                        thing_meter_circle_element.style.strokeDasharray = percent + ' 100';
                    } else {
                        thing_meter_circle_element.style.strokeDasharray = '100 100';
                    }
                }

                next(
                    thing_value_element,
                    property_value_reader, 
                    property_link,
                    property_min,
                    property_max,
                    do_update_thing_meter_circle_element,
                );
            }

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const primary_property = await read_property(base, self.getAttribute('data-property'));

            fire(
                REFRESH_RATE,
                update,
                thing_primary_value_element,
                primary_property.value_reader,
                primary_property.link,
                primary_property.min,
                primary_property.max,
                true,
            );

            if (self.hasAttribute('data-secondary-property')) {
                const secondary_property = await read_property(base, self.getAttribute('data-secondary-property'));

                fire(
                    REFRESH_RATE,
                    update,
                    thing_secondary_value_element,
                    secondary_property.value_reader,
                    secondary_property.link,
                    secondary_property.min,
                    secondary_property.max,
                    false,
                );
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

            const thing_primary_value_element = template_content.querySelector('.thing--solar-pv-primary-value');
            const thing_meter_circle_element = template_content.querySelector('.thing--solar-pv-meter .meter');
            const thing_sunrise_element = template_content.querySelector('.thing--solar-pv-sunrise');
            const thing_sunset_element = template_content.querySelector('.thing--solar-pv-sunset');
            const thing_sun_element = template_content.querySelector('.thing--solar-pv-sun');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            async function update(
                next,
                thing_value_element,
                property_value_reader,
                property_link,
                property_min,
                property_max,
                thing_sunrise_element,
                thing_sunset_element,
                thing_sun_element,
            ) {
                // `thing_value_element`.
                const {value, formatted_value} = await property_value_reader();

                thing_value_element.innerHTML = formatted_value;

                // `thing_sunrise_element` + `thing_sunset_element`.
                let now = new Date();
                let { sunrise, sunset } = sunrise_sunset(
                    HOME_LATITUDE,
                    HOME_LONGITUDE,
                    now.getFullYear(),
                    now.getMonth() + 1,
                    now.getDate()
                );

                thing_sunrise_element.innerHTML = sunrise.getHours() + ":" + number_to_2_chars(sunrise.getMinutes());
                thing_sunset_element.innerHTML = sunset.getHours() + ":" + number_to_2_chars(sunset.getMinutes());

                // `thing_sun_element`.
                let now_in_minutes = now.getHours() * 60 + now.getMinutes();
                const min_sun = sunrise.getHours() * 60 + sunrise.getMinutes();
                const max_sun = sunset.getHours() * 60 + sunset.getMinutes();
                const min_circle = 50;
                const max_circle = 100;

                const pos = ((now_in_minutes - min_sun) / (max_sun - min_sun)) * (max_circle - min_circle) + min_circle;

                const pos_point = thing_meter_circle_element.getPointAtLength(pos);
                thing_sun_element.setAttributeNS(null, "cx", pos_point.x);
                thing_sun_element.setAttributeNS(null, "cy", pos_point.y);

                next(
                    thing_value_element,
                    property_value_reader,
                    property_link,
                    property_min,
                    property_max,
                    thing_sunrise_element,
                    thing_sunset_element,
                    thing_sun_element,
                );
            }

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const primary_property = await read_property(base, self.getAttribute('data-property'));

            fire(
                REFRESH_RATE,
                update,
                thing_primary_value_element,
                primary_property.value_reader,
                primary_property.link,
                primary_property.min,
                primary_property.max,
                thing_sunrise_element,
                thing_sunset_element,
                thing_sun_element,
            );
        }
    }
);

window.customElements.define(
    'my-temperature-thing',
    class extends HTMLElement {
        constructor() {
            super();
        }

        async connectedCallback() {
            const template = document.getElementById('template--temperature-thing');
            const template_content = template.content.cloneNode(true);

            const thing_value_element = template_content.querySelector('.thing--meter-primary-value');
            const thing_meter_circle_element = template_content.querySelector('.thing--meter-meter .meter');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            async function update(
                next,
                thing_value_element,
                property_value_reader,
                property_link,
                property_max,
            ) {
                const {value, formatted_value} = await property_value_reader();

                thing_value_element.innerHTML = formatted_value;

                const percent = (value * 100) / property_max;
                thing_meter_circle_element.style.strokeDasharray = percent + ' 100';

                next(
                    thing_value_element,
                    property_value_reader,
                    property_link,
                    property_max,
                );
            }

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const current_property = await read_property(base, self.getAttribute('data-current-value'));
            const target_property = await read_property(base, self.getAttribute('data-target-value'));
            const target_value = (await target_property.value_reader()).value;

            fire(
                REFRESH_RATE,
                update,
                thing_value_element,
                current_property.value_reader,
                current_property.link,
                target_value,
            );
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

            const button = template_content.querySelector('.thing--type-pulse button');

            const shadow_root = this.attachShadow({mode: 'open'})
                  .appendChild(template_content);

            const self = this;
            const base = self.getAttribute('data-base').replace(/\/+$/, '');
            const url = base + '/properties/pulse';

            button.addEventListener(
                'click',
                () => {
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

            const open_button = template_content.querySelector('.thing--type-blind__open');
            const stop_button = template_content.querySelector('.thing--type-blind__stop');
            const close_button = template_content.querySelector('.thing--type-blind__close');

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
        new function() {
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
        };
    }
);
