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
            let formatted_value = value;

            switch (unit) {
            case 'percent':
                formatted_value += '%';
                break;

            case 'watt':
                formatted_value += 'W';
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
        };
    }
);

window.customElements.define(
    'my-meter-thing',
    new function() {
        let thing_index = 0;

        return class extends HTMLElement {
            constructor() {
                super();
            }

            async connectedCallback() {
                const template = document.getElementById('template--meter-thing');
                const template_content = template.content.cloneNode(true);

                const thing = template_content.querySelector('.thing');
                thing.setAttribute('id', 'meter-thing-' + thing_index);
                thing_index += 1;

                const thing_primary_value_element = template_content.querySelector('.thing--meter-primary-value');
                const thing_secondary_value_element = template_content.querySelector('.thing--meter-secondary-value');
                const thing_meter_circle_element = template_content.querySelector('.thing--meter-meter > .meter--blend > circle');

                const shadow_root = this.attachShadow({mode: 'open'})
                      .appendChild(template_content);

                async function update_value(
                    thing_value_element,
                    property_value_reader,
                    property_link,
                    property_unit,
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

                    window.setTimeout(
                        () => {
                            update_value(
                                thing_value_element,
                                property_value_reader,
                                property_link,
                                property_unit,
                                property_min,
                                property_max,
                                do_update_thing_meter_circle_element
                            );
                        },
                        1000 * 10 /* in 10 secs */,
                        false
                    );
                }

                const self = this;
                const base = self.getAttribute('data-base').replace(/\/+$/, '');
                const primary_property = await read_property(base, self.getAttribute('data-property'));

                update_value(
                    thing_primary_value_element,
                    primary_property.value_reader,
                    primary_property.link,
                    primary_property.unit,
                    primary_property.min,
                    primary_property.max,
                    true,
                );

                if (self.hasAttribute('data-secondary-property')) {
                    const secondary_property = await read_property(base, self.getAttribute('data-secondary-property'));

                    update_value(
                        thing_secondary_value_element,
                        secondary_property.value_reader,
                        secondary_property.link,
                        secondary_property.unit,
                        secondary_property.min,
                        secondary_property.max,
                        false,
                    );
                }
            }
        };
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
