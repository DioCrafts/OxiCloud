/**
 * Path tooltip — shows the full path of a hovered file/folder item
 * in an overlay at the bottom-left of the content area.
 *
 * Usage: call init(container) after rendering items, destroy(container) on teardown.
 * Only file-item elements with a data-path attribute trigger the tooltip.
 */

/** @type {HTMLElement|null} */
let _tooltip = null;

function _getOrCreateTooltip() {
    if (_tooltip) return _tooltip;
    _tooltip = document.getElementById('path-tooltip');
    if (!_tooltip) {
        _tooltip = document.createElement('div');
        _tooltip.id = 'path-tooltip';
        _tooltip.className = 'path-tooltip hidden';
        document.querySelector('.main-content')?.appendChild(_tooltip);
    }
    return _tooltip;
}

/**
 * @param {MouseEvent} e
 */
function _onEnter(e) {
    const item = /** @type {HTMLElement} */ (e.currentTarget);
    const path = item.dataset.path;
    if (!path) return;

    const tooltip = _getOrCreateTooltip();
    tooltip.textContent = path;
    tooltip.classList.remove('hidden');
}

function _onLeave() {
    _tooltip?.classList.add('hidden');
}

/**
 * @typedef {Object} EnterLeaveF
 * @property {(e: MouseEvent) => void} enter
 * @property {(e: MouseEvent) => void} leave
 *

/** @type {WeakMap<HTMLElement, EnterLeaveF>} */
const _listeners = new WeakMap();

/**
 * Attach path tooltip listeners to all file-item elements inside container.
 * @param {HTMLElement} container
 */
function init(container) {
    const items = container.querySelectorAll('.file-item[data-path]');
    items.forEach((item) => {
        const el = /** @type {HTMLElement} */ (item);

        /** @type {(e: MouseEvent) => void} */
        const enter = (e) => _onEnter(e);
        el.addEventListener('mouseenter', enter);

        /** @type {(e: MouseEvent) => void} */
        const leave = (_e) => _onLeave();
        el.addEventListener('mouseleave', leave);

        _listeners.set(el, { enter, leave });
    });
}

/**
 * Remove path tooltip listeners from all file-item elements inside container.
 * @param {HTMLElement} container
 */
function destroy(container) {
    const items = container.querySelectorAll('.file-item');
    items.forEach((item) => {
        const el = /** @type {HTMLElement} */ (item);
        const fns = _listeners.get(el);
        if (fns) {
            el.removeEventListener('mouseenter', fns.enter);
            el.removeEventListener('mouseleave', fns.leave);
            _listeners.delete(el);
        }
    });
    _onLeave();
}

export { destroy, init };
