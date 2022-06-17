const Collapsables = (function collapsables() {
    const COLLAPSED = 'collapsed';

    function clickHandle() {
        const handle = this;
        const collapsable = handle.parentElement;
        const classList = collapsable.classList;
        const collapsed = classList.contains("collapsed");

        if (collapsed) {
            classList.remove(COLLAPSED);
        } else {
            classList.add(COLLAPSED);
        }

    }
    class Collapsables {
        static #handles = [];

        static collect(elements) {
            elements.forEach(element => {
                element.addEventListener("click", clickHandle);
            })
        }
    }

    return Collapsables;
})();
