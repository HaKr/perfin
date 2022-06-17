const Sources = (function sources() {
    const HAS_SOURCE = 'has-source';
    const CLASS_HIDDEN = "hidden";

    function showJSSource(tab, showSourceElement, sourceElement) {
        return () => {
            Sources.showTab(tab);
            tab.labelElement.textContent = `Javascript`;
            showSourceElement.textContent = js_beautify(sourceElement.textContent, { indent_size: 2 });
            Prism.highlightElement(showSourceElement);
        };
    }


    function loadHtml(resourceType, resourceId) {
        const uri = "/store/resource_content";
        const xhr = new XMLHttpRequest();

        return new Promise((resolve, reject) => {
            xhr.open("POST", uri, true);
            xhr.setRequestHeader("Content-Type", "application/json");

            xhr.onreadystatechange = function () {
                if (xhr.readyState == 4) {
                    if (xhr.status == 200) {
                        resolve(xhr.responseText);
                    } else {
                        reject(new Error(xhr.responseText))
                    }
                }
            };
            xhr.send(JSON.stringify({ template: Sources.template, resourceType, resourceId }));

        })

    }

    function showHtmlSource(tab, showSourceElement, sourceElement) {
        return () => {
            Sources.showTab(tab);
            tab.labelElement.textContent = `HTML`;
            loadHtml(sourceElement.dataset.resourceType, sourceElement.dataset.id)
                .then(html => {
                    showSourceElement.textContent = html_beautify(html, { indent_size: 2 });
                    Prism.highlightElement(showSourceElement);
                }).catch(console.error);

        };
    }

    function showHandlebarsSource(tab, showSourceElement, sourceElement) {
        return () => {
            Sources.showTab(tab);
            tab.labelElement.textContent = `Handlebars`;
            loadHtml(sourceElement.dataset.resourceType, sourceElement.dataset.id)
                .then(hbs => {
                    showSourceElement.textContent = hbs;
                    Prism.highlightElement(showSourceElement);
                }).catch(console.error);

        };
    }

    function showCssSource(tab, showSourceElement, sourceElement) {
        return () => {
            Sources.showTab(tab);
            tab.labelElement.textContent = `Cascading Stylesheet`;
            loadHtml(sourceElement.dataset.resourceType, sourceElement.dataset.id)
                .then(html => {
                    showSourceElement.textContent = css_beautify(html, { indent_size: 2 });
                    Prism.highlightElement(showSourceElement);
                }).catch(console.error);

        };
    }

    function showImage(tab, showSourceElement, sourceElement) {
        return () => {
            Sources.showTab(tab);
            tab.labelElement.textContent = `Image`;
            showSourceElement.src = `/template/${Sources.template}/image/${sourceElement.dataset.id}`
        };
    }

    function js_handler(tab, element, showSourceElement) {
        const sourceElement = (element.querySelector("code"));
        const sourceNode = sourceElement ? sourceElement.childNodes[0] : null;
        if (sourceNode && sourceNode.nodeType == Node.COMMENT_NODE) {
            element.classList.add(HAS_SOURCE);
            element.addEventListener("click", showJSSource(tab, showSourceElement, sourceNode));
        }
    }

    function html_handler(tab, element, showSourceElement) {
        element.classList.add(HAS_SOURCE);
        element.addEventListener("click", showHtmlSource(tab, showSourceElement, element));
    }

    function handlebars_handler(tab, element, showSourceElement) {
        element.classList.add(HAS_SOURCE);
        element.addEventListener("click", showHandlebarsSource(tab, showSourceElement, element));
    }

    function css_handler(tab, element, showSourceElement) {
        element.classList.add(HAS_SOURCE);
        element.addEventListener("click", showCssSource(tab, showSourceElement, element));
    }

    function image_handler(tab, element, showSourceElement) {
        element.classList.add(HAS_SOURCE);
        element.addEventListener("click", showImage(tab, showSourceElement, element));
    }

    class SourceTab {
        #domElement;

        constructor(domElement, labelElement) {
            this.#domElement = domElement;
            this.labelElement = labelElement;
        }

        show() {
            this.#domElement.classList.remove(CLASS_HIDDEN);
        }

        hide() {
            this.#domElement.classList.add(CLASS_HIDDEN);
        }
    }

    class Sources {
        static #tabsMap = new Map([
            ["html", { tab: null, selector: "ul.resources li.html", handler: html_handler }],
            ["css", { tab: null, selector: "ul.resources li.css", handler: css_handler }],
            ["js", { tab: null, selector: "ul.scripts li.script_node.script", handler: js_handler }],
            ["image", { tab: null, selector: "ul.resources li.image", handler: image_handler }],
            ["hbs", { tab: null, selector: "ul.resources li.hbs", handler: handlebars_handler }]
        ]);

        static showTab(tabToShow) {
            for (let info of Sources.#tabsMap.values()) {
                if (info.tab == tabToShow) {
                    info.tab.show()
                } else {
                    info.tab.hide()
                }
            }
        }

        static collect(container) {
            const label = container.querySelector(".resource-label");
            for (let entry of Sources.#tabsMap.entries()) {
                const [sourceType, info] = entry;
                container.querySelectorAll(`.source-tab.${sourceType}`).forEach(tabElement => {
                    const tab = new SourceTab(tabElement, label);
                    info.tab = tab;
                    const codeElement = tabElement.querySelector("code");
                    const imgElement = tabElement.querySelector("img");
                    const showSourceElement = codeElement ? codeElement : imgElement;
                    document.body.querySelectorAll(info.selector).forEach(element => info.handler(tab, element, showSourceElement));
                })
            }

        }
    }

    return Sources;
})();
