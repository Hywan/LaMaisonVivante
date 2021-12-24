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

                const self = this;
                const base = self.getAttribute('data-base').replace(/\/+$/, '');
                const base_origin = new URL(base).origin;
                const primary_property_name = self.getAttribute('data-property');

                const primary_property_response = await http_get(base);
                const primary_property_json_response = await primary_property_response.json();

                const primary_property_description = primary_property_json_response.properties[primary_property_name];
                const primary_property_unit = primary_property_description.unit;
                const primary_property_link = primary_property_description.links[0].href;

                async function update_value(thing_value_element, property_name, property_link, property_unit, do_update_thing_meter_circle_element) {
                    const response = await http_get(base_origin + property_link);
                    const json_response = await response.json();
                    const value = json_response[property_name];
                    let formatted_value = value;

                    switch (property_unit) {
                    case 'percent':
                        formatted_value += '%';
                        break;

                    case 'watt':
                        formatted_value += 'W';
                        break;
                    }

                    thing_value_element.innerHTML = formatted_value;

                    if (do_update_thing_meter_circle_element) {
                        thing_meter_circle_element.style.strokeDasharray = value + ' 100';
                    }

                    window.setTimeout(
                        () => {
                            update_value(
                                thing_value_element,
                                property_name,
                                property_link,
                                property_unit,
                                do_update_thing_meter_circle_element
                            );
                        },
                        1000 * 30 /* in 30 secs */,
                        false
                    );
                }

                update_value(thing_primary_value_element, primary_property_name, primary_property_link, primary_property_unit, true);

                if (self.hasAttribute('data-secondary-property')) {
                    const secondary_property_name = self.getAttribute('data-secondary-property');

                    const secondary_property_response = await http_get(base);
                    const secondary_property_json_response = await secondary_property_response.json();

                    const secondary_property_description = secondary_property_json_response.properties[secondary_property_name];
                    const secondary_property_unit = secondary_property_description.unit;
                    const secondary_property_link = secondary_property_description.links[0].href;

                    update_value(thing_secondary_value_element, secondary_property_name, secondary_property_link, secondary_property_unit, false);
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
