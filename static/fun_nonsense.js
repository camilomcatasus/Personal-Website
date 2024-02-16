/******/ (() => { // webpackBootstrap
/******/ 	"use strict";
/******/ 	var __webpack_modules__ = ({

/***/ "./htmx_snake.ts":
/*!***********************!*\
  !*** ./htmx_snake.ts ***!
  \***********************/
/***/ ((__unused_webpack_module, __webpack_exports__, __webpack_require__) => {

__webpack_require__.r(__webpack_exports__);
/* harmony export */ __webpack_require__.d(__webpack_exports__, {
/* harmony export */   snakeKeyDownHandler: () => (/* binding */ snakeKeyDownHandler)
/* harmony export */ });
function switchKey(direction) {
    var directionField = document.getElementById("direction");
    directionField.setAttribute("value", direction.toString());
}
function snakeKeyDownHandler(event) {
    if (window.location.pathname == "/fun-nonsense/htmx-snake") {
        switch (event.key) {
            case "ArrowLeft":
            case "A":
                switchKey(1);
                break;
            case "ArrowUp":
            case "W":
                switchKey(2);
                break;
            case "ArrowRight":
            case "D":
                switchKey(3);
                break;
            case "ArrowDown":
            case "S":
                switchKey(4);
                break;
        }
    }
}


/***/ })

/******/ 	});
/************************************************************************/
/******/ 	// The module cache
/******/ 	var __webpack_module_cache__ = {};
/******/ 	
/******/ 	// The require function
/******/ 	function __webpack_require__(moduleId) {
/******/ 		// Check if module is in cache
/******/ 		var cachedModule = __webpack_module_cache__[moduleId];
/******/ 		if (cachedModule !== undefined) {
/******/ 			return cachedModule.exports;
/******/ 		}
/******/ 		// Create a new module (and put it into the cache)
/******/ 		var module = __webpack_module_cache__[moduleId] = {
/******/ 			// no module.id needed
/******/ 			// no module.loaded needed
/******/ 			exports: {}
/******/ 		};
/******/ 	
/******/ 		// Execute the module function
/******/ 		__webpack_modules__[moduleId](module, module.exports, __webpack_require__);
/******/ 	
/******/ 		// Return the exports of the module
/******/ 		return module.exports;
/******/ 	}
/******/ 	
/************************************************************************/
/******/ 	/* webpack/runtime/define property getters */
/******/ 	(() => {
/******/ 		// define getter functions for harmony exports
/******/ 		__webpack_require__.d = (exports, definition) => {
/******/ 			for(var key in definition) {
/******/ 				if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
/******/ 					Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
/******/ 				}
/******/ 			}
/******/ 		};
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/hasOwnProperty shorthand */
/******/ 	(() => {
/******/ 		__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
/******/ 	})();
/******/ 	
/******/ 	/* webpack/runtime/make namespace object */
/******/ 	(() => {
/******/ 		// define __esModule on exports
/******/ 		__webpack_require__.r = (exports) => {
/******/ 			if(typeof Symbol !== 'undefined' && Symbol.toStringTag) {
/******/ 				Object.defineProperty(exports, Symbol.toStringTag, { value: 'Module' });
/******/ 			}
/******/ 			Object.defineProperty(exports, '__esModule', { value: true });
/******/ 		};
/******/ 	})();
/******/ 	
/************************************************************************/
var __webpack_exports__ = {};
// This entry need to be wrapped in an IIFE because it need to be isolated against other modules in the chunk.
(() => {
/*!*****************!*\
  !*** ./main.ts ***!
  \*****************/
__webpack_require__.r(__webpack_exports__);
/* harmony import */ var _htmx_snake__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./htmx_snake */ "./htmx_snake.ts");

function toggleClass(elem, class1, class2) {
    if (elem.classList.contains(class1)) {
        elem.classList.remove(class1);
        elem.classList.add(class2);
    }
    else {
        elem.classList.add(class1);
        elem.classList.remove(class2);
    }
}
function secondaryNavLoaded(path) {
    var nav = document.getElementById("secondary-nav");
    var allNavItems = nav.querySelectorAll("a");
    allNavItems.forEach(function (navItem) {
        navItem.classList.remove("underline");
    });
    var selectedAnchor = nav.querySelector("a[href=\"".concat(path, "\"]"));
    selectedAnchor.classList.add("underline");
}
function sectionSetup() {
    var path_part = window.location.pathname.split("/");
    console.log(path_part);
    if (path_part.length >= 2) {
        switch (path_part[1]) {
            case "fun-nonsense":
                secondaryNavLoaded(window.location.pathname);
                break;
        }
    }
}
window.onload = function () {
    document.onkeydown = function (event) {
        (0,_htmx_snake__WEBPACK_IMPORTED_MODULE_0__.snakeKeyDownHandler)(event);
    };
    document.onclick = function (event) {
        var target = event.target;
        if (target.matches(":is(#toggle-nav-button, #toggle-nav-button *)")) {
            var secondaryNav = document.getElementById("secondary-nav");
            var parent_1 = secondaryNav.parentElement;
            toggleClass(secondaryNav, "w-0", "w-60");
            toggleClass(parent_1, "w-0", "w-60");
            document.getElementById("toggle-nav-button").classList.toggle("is-active");
        }
        else if (target.matches("#secondary-nav > a")) {
            secondaryNavLoaded(target.getAttribute("href"));
        }
    };
    sectionSetup();
};

})();

/******/ })()
;
//# sourceMappingURL=data:application/json;charset=utf-8;base64,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cyI6WyJ3ZWJwYWNrOi8vc2NyaXB0cy8uL2h0bXhfc25ha2UudHMiLCJ3ZWJwYWNrOi8vc2NyaXB0cy93ZWJwYWNrL2Jvb3RzdHJhcCIsIndlYnBhY2s6Ly9zY3JpcHRzL3dlYnBhY2svcnVudGltZS9kZWZpbmUgcHJvcGVydHkgZ2V0dGVycyIsIndlYnBhY2s6Ly9zY3JpcHRzL3dlYnBhY2svcnVudGltZS9oYXNPd25Qcm9wZXJ0eSBzaG9ydGhhbmQiLCJ3ZWJwYWNrOi8vc2NyaXB0cy93ZWJwYWNrL3J1bnRpbWUvbWFrZSBuYW1lc3BhY2Ugb2JqZWN0Iiwid2VicGFjazovL3NjcmlwdHMvLi9tYWluLnRzIl0sInNvdXJjZXNDb250ZW50IjpbImZ1bmN0aW9uIHN3aXRjaEtleShkaXJlY3Rpb246IG51bWJlcikge1xyXG4gICAgbGV0IGRpcmVjdGlvbkZpZWxkID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJkaXJlY3Rpb25cIikgYXMgSFRNTEVsZW1lbnQ7XHJcbiAgICBkaXJlY3Rpb25GaWVsZC5zZXRBdHRyaWJ1dGUoXCJ2YWx1ZVwiLCBkaXJlY3Rpb24udG9TdHJpbmcoKSk7XHJcbn1cclxuXHJcbmV4cG9ydCBmdW5jdGlvbiBzbmFrZUtleURvd25IYW5kbGVyKGV2ZW50OiBLZXlib2FyZEV2ZW50KSB7XHJcbiAgICBpZih3aW5kb3cubG9jYXRpb24ucGF0aG5hbWUgPT0gXCIvZnVuLW5vbnNlbnNlL2h0bXgtc25ha2VcIikge1xyXG4gICAgICAgIHN3aXRjaChldmVudC5rZXkpXHJcbiAgICAgICAge1xyXG4gICAgICAgICAgICBjYXNlIFwiQXJyb3dMZWZ0XCI6IFxyXG4gICAgICAgICAgICBjYXNlIFwiQVwiOlxyXG4gICAgICAgICAgICAgICAgc3dpdGNoS2V5KDEpO1xyXG4gICAgICAgICAgICAgICAgYnJlYWs7XHJcbiAgICAgICAgICAgIGNhc2UgXCJBcnJvd1VwXCI6XHJcbiAgICAgICAgICAgIGNhc2UgXCJXXCI6XHJcbiAgICAgICAgICAgICAgICBzd2l0Y2hLZXkoMik7XHJcbiAgICAgICAgICAgICAgICBicmVhaztcclxuICAgICAgICAgICAgY2FzZSBcIkFycm93UmlnaHRcIjpcclxuICAgICAgICAgICAgY2FzZSBcIkRcIjpcclxuICAgICAgICAgICAgICAgIHN3aXRjaEtleSgzKTtcclxuICAgICAgICAgICAgICAgIGJyZWFrO1xyXG4gICAgICAgICAgICBjYXNlIFwiQXJyb3dEb3duXCI6XHJcbiAgICAgICAgICAgIGNhc2UgXCJTXCI6XHJcbiAgICAgICAgICAgICAgICBzd2l0Y2hLZXkoNCk7XHJcbiAgICAgICAgICAgICAgICBicmVhaztcclxuICAgICAgICB9XHJcbiAgICB9XHJcbn1cclxuIiwiLy8gVGhlIG1vZHVsZSBjYWNoZVxudmFyIF9fd2VicGFja19tb2R1bGVfY2FjaGVfXyA9IHt9O1xuXG4vLyBUaGUgcmVxdWlyZSBmdW5jdGlvblxuZnVuY3Rpb24gX193ZWJwYWNrX3JlcXVpcmVfXyhtb2R1bGVJZCkge1xuXHQvLyBDaGVjayBpZiBtb2R1bGUgaXMgaW4gY2FjaGVcblx0dmFyIGNhY2hlZE1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF07XG5cdGlmIChjYWNoZWRNb2R1bGUgIT09IHVuZGVmaW5lZCkge1xuXHRcdHJldHVybiBjYWNoZWRNb2R1bGUuZXhwb3J0cztcblx0fVxuXHQvLyBDcmVhdGUgYSBuZXcgbW9kdWxlIChhbmQgcHV0IGl0IGludG8gdGhlIGNhY2hlKVxuXHR2YXIgbW9kdWxlID0gX193ZWJwYWNrX21vZHVsZV9jYWNoZV9fW21vZHVsZUlkXSA9IHtcblx0XHQvLyBubyBtb2R1bGUuaWQgbmVlZGVkXG5cdFx0Ly8gbm8gbW9kdWxlLmxvYWRlZCBuZWVkZWRcblx0XHRleHBvcnRzOiB7fVxuXHR9O1xuXG5cdC8vIEV4ZWN1dGUgdGhlIG1vZHVsZSBmdW5jdGlvblxuXHRfX3dlYnBhY2tfbW9kdWxlc19fW21vZHVsZUlkXShtb2R1bGUsIG1vZHVsZS5leHBvcnRzLCBfX3dlYnBhY2tfcmVxdWlyZV9fKTtcblxuXHQvLyBSZXR1cm4gdGhlIGV4cG9ydHMgb2YgdGhlIG1vZHVsZVxuXHRyZXR1cm4gbW9kdWxlLmV4cG9ydHM7XG59XG5cbiIsIi8vIGRlZmluZSBnZXR0ZXIgZnVuY3Rpb25zIGZvciBoYXJtb255IGV4cG9ydHNcbl9fd2VicGFja19yZXF1aXJlX18uZCA9IChleHBvcnRzLCBkZWZpbml0aW9uKSA9PiB7XG5cdGZvcih2YXIga2V5IGluIGRlZmluaXRpb24pIHtcblx0XHRpZihfX3dlYnBhY2tfcmVxdWlyZV9fLm8oZGVmaW5pdGlvbiwga2V5KSAmJiAhX193ZWJwYWNrX3JlcXVpcmVfXy5vKGV4cG9ydHMsIGtleSkpIHtcblx0XHRcdE9iamVjdC5kZWZpbmVQcm9wZXJ0eShleHBvcnRzLCBrZXksIHsgZW51bWVyYWJsZTogdHJ1ZSwgZ2V0OiBkZWZpbml0aW9uW2tleV0gfSk7XG5cdFx0fVxuXHR9XG59OyIsIl9fd2VicGFja19yZXF1aXJlX18ubyA9IChvYmosIHByb3ApID0+IChPYmplY3QucHJvdG90eXBlLmhhc093blByb3BlcnR5LmNhbGwob2JqLCBwcm9wKSkiLCIvLyBkZWZpbmUgX19lc01vZHVsZSBvbiBleHBvcnRzXG5fX3dlYnBhY2tfcmVxdWlyZV9fLnIgPSAoZXhwb3J0cykgPT4ge1xuXHRpZih0eXBlb2YgU3ltYm9sICE9PSAndW5kZWZpbmVkJyAmJiBTeW1ib2wudG9TdHJpbmdUYWcpIHtcblx0XHRPYmplY3QuZGVmaW5lUHJvcGVydHkoZXhwb3J0cywgU3ltYm9sLnRvU3RyaW5nVGFnLCB7IHZhbHVlOiAnTW9kdWxlJyB9KTtcblx0fVxuXHRPYmplY3QuZGVmaW5lUHJvcGVydHkoZXhwb3J0cywgJ19fZXNNb2R1bGUnLCB7IHZhbHVlOiB0cnVlIH0pO1xufTsiLCJpbXBvcnQgeyBzbmFrZUtleURvd25IYW5kbGVyIH0gZnJvbSBcIi4vaHRteF9zbmFrZVwiO1xyXG5cclxuXHJcbmZ1bmN0aW9uIHRvZ2dsZUNsYXNzKGVsZW06IEhUTUxFbGVtZW50LCBjbGFzczE6IHN0cmluZywgY2xhc3MyOiBzdHJpbmcpIHtcclxuICAgIGlmIChlbGVtLmNsYXNzTGlzdC5jb250YWlucyhjbGFzczEpKVxyXG4gICAge1xyXG4gICAgICAgIGVsZW0uY2xhc3NMaXN0LnJlbW92ZShjbGFzczEpO1xyXG4gICAgICAgIGVsZW0uY2xhc3NMaXN0LmFkZChjbGFzczIpO1xyXG4gICAgfVxyXG4gICAgZWxzZSBcclxuICAgIHtcclxuICAgICAgICBlbGVtLmNsYXNzTGlzdC5hZGQoY2xhc3MxKTtcclxuICAgICAgICBlbGVtLmNsYXNzTGlzdC5yZW1vdmUoY2xhc3MyKTtcclxuICAgIH1cclxufVxyXG5cclxuZnVuY3Rpb24gc2Vjb25kYXJ5TmF2TG9hZGVkKHBhdGg6c3RyaW5nKSB7XHJcbiAgICBsZXQgbmF2ID0gZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJzZWNvbmRhcnktbmF2XCIpO1xyXG4gICAgbGV0IGFsbE5hdkl0ZW1zID0gbmF2LnF1ZXJ5U2VsZWN0b3JBbGwoXCJhXCIpO1xyXG4gICAgYWxsTmF2SXRlbXMuZm9yRWFjaCgobmF2SXRlbSkgPT4ge1xyXG4gICAgICAgIG5hdkl0ZW0uY2xhc3NMaXN0LnJlbW92ZShcInVuZGVybGluZVwiKTtcclxuICAgIH0pXHJcblxyXG4gICAgbGV0IHNlbGVjdGVkQW5jaG9yID0gbmF2LnF1ZXJ5U2VsZWN0b3IoYGFbaHJlZj1cIiR7cGF0aH1cIl1gKSBhcyBIVE1MRWxlbWVudDtcclxuICAgIHNlbGVjdGVkQW5jaG9yLmNsYXNzTGlzdC5hZGQoXCJ1bmRlcmxpbmVcIik7XHJcbn1cclxuXHJcbmZ1bmN0aW9uIHNlY3Rpb25TZXR1cCgpIHtcclxuICAgIGxldCBwYXRoX3BhcnQgPSB3aW5kb3cubG9jYXRpb24ucGF0aG5hbWUuc3BsaXQoXCIvXCIpO1xyXG4gICAgY29uc29sZS5sb2cocGF0aF9wYXJ0KTtcclxuICAgIGlmKHBhdGhfcGFydC5sZW5ndGggPj0gMikge1xyXG4gICAgICAgIHN3aXRjaChwYXRoX3BhcnRbMV0pIHtcclxuICAgICAgICAgICAgY2FzZSBcImZ1bi1ub25zZW5zZVwiOiBcclxuICAgICAgICAgICAgICAgIHNlY29uZGFyeU5hdkxvYWRlZCh3aW5kb3cubG9jYXRpb24ucGF0aG5hbWUpOyBcclxuICAgICAgICAgICAgICAgIGJyZWFrO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxufVxyXG5cclxuXHJcbndpbmRvdy5vbmxvYWQgPSAoKSA9PiB7XHJcbiAgICBkb2N1bWVudC5vbmtleWRvd24gPSAoZXZlbnQ6IEtleWJvYXJkRXZlbnQpID0+IHtcclxuICAgICAgICBzbmFrZUtleURvd25IYW5kbGVyKGV2ZW50KTtcclxuICAgIH1cclxuXHJcbiAgICBkb2N1bWVudC5vbmNsaWNrID0gKGV2ZW50OiBFdmVudCkgPT4ge1xyXG4gICAgICAgIGxldCB0YXJnZXQgPSBldmVudC50YXJnZXQgYXMgSFRNTEVsZW1lbnQ7XHJcbiAgICAgICAgaWYgKHRhcmdldC5tYXRjaGVzKFwiOmlzKCN0b2dnbGUtbmF2LWJ1dHRvbiwgI3RvZ2dsZS1uYXYtYnV0dG9uICopXCIpKSB7XHJcbiAgICAgICAgICAgIGxldCBzZWNvbmRhcnlOYXYgPSBkb2N1bWVudC5nZXRFbGVtZW50QnlJZChcInNlY29uZGFyeS1uYXZcIik7XHJcbiAgICAgICAgICAgIGxldCBwYXJlbnQgPSBzZWNvbmRhcnlOYXYucGFyZW50RWxlbWVudDtcclxuICAgICAgICAgICAgdG9nZ2xlQ2xhc3Moc2Vjb25kYXJ5TmF2LCBcInctMFwiLCBcInctNjBcIik7XHJcbiAgICAgICAgICAgIHRvZ2dsZUNsYXNzKHBhcmVudCwgXCJ3LTBcIiwgXCJ3LTYwXCIpO1xyXG4gICAgICAgICAgICBcclxuICAgICAgICAgICAgZG9jdW1lbnQuZ2V0RWxlbWVudEJ5SWQoXCJ0b2dnbGUtbmF2LWJ1dHRvblwiKS5jbGFzc0xpc3QudG9nZ2xlKFwiaXMtYWN0aXZlXCIpO1xyXG4gICAgICAgIH1cclxuICAgICAgICBlbHNlIGlmICh0YXJnZXQubWF0Y2hlcyhcIiNzZWNvbmRhcnktbmF2ID4gYVwiKSkge1xyXG4gICAgICAgICAgICBzZWNvbmRhcnlOYXZMb2FkZWQodGFyZ2V0LmdldEF0dHJpYnV0ZShcImhyZWZcIikpO1xyXG4gICAgICAgIH1cclxuICAgIH1cclxuXHJcbiAgICBzZWN0aW9uU2V0dXAoKTtcclxufVxyXG5cclxuXHJcbiJdLCJuYW1lcyI6W10sInNvdXJjZVJvb3QiOiIifQ==