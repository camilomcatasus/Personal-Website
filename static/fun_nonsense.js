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

window.onload = function () {
    document.onkeydown = function (event) {
        (0,_htmx_snake__WEBPACK_IMPORTED_MODULE_0__.snakeKeyDownHandler)(event);
    };
};

})();

/******/ })()
;
//# sourceMappingURL=data:application/json;charset=utf-8;base64,eyJ2ZXJzaW9uIjozLCJmaWxlIjoiZnVuX25vbnNlbnNlLmpzIiwibWFwcGluZ3MiOiI7Ozs7Ozs7Ozs7Ozs7O0FBQUEsU0FBUyxTQUFTLENBQUMsU0FBaUI7SUFDaEMsSUFBSSxjQUFjLEdBQUcsUUFBUSxDQUFDLGNBQWMsQ0FBQyxXQUFXLENBQWdCLENBQUM7SUFDekUsY0FBYyxDQUFDLFlBQVksQ0FBQyxPQUFPLEVBQUUsU0FBUyxDQUFDLFFBQVEsRUFBRSxDQUFDLENBQUM7QUFDL0QsQ0FBQztBQUVNLFNBQVMsbUJBQW1CLENBQUMsS0FBb0I7SUFDcEQsSUFBRyxNQUFNLENBQUMsUUFBUSxDQUFDLFFBQVEsSUFBSSwwQkFBMEIsRUFBRSxDQUFDO1FBQ3hELFFBQU8sS0FBSyxDQUFDLEdBQUcsRUFDaEIsQ0FBQztZQUNHLEtBQUssV0FBVyxDQUFDO1lBQ2pCLEtBQUssR0FBRztnQkFDSixTQUFTLENBQUMsQ0FBQyxDQUFDLENBQUM7Z0JBQ2IsTUFBTTtZQUNWLEtBQUssU0FBUyxDQUFDO1lBQ2YsS0FBSyxHQUFHO2dCQUNKLFNBQVMsQ0FBQyxDQUFDLENBQUMsQ0FBQztnQkFDYixNQUFNO1lBQ1YsS0FBSyxZQUFZLENBQUM7WUFDbEIsS0FBSyxHQUFHO2dCQUNKLFNBQVMsQ0FBQyxDQUFDLENBQUMsQ0FBQztnQkFDYixNQUFNO1lBQ1YsS0FBSyxXQUFXLENBQUM7WUFDakIsS0FBSyxHQUFHO2dCQUNKLFNBQVMsQ0FBQyxDQUFDLENBQUMsQ0FBQztnQkFDYixNQUFNO1FBQ2QsQ0FBQztJQUNMLENBQUM7QUFDTCxDQUFDOzs7Ozs7O1VDM0JEO1VBQ0E7O1VBRUE7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7VUFDQTtVQUNBO1VBQ0E7O1VBRUE7VUFDQTs7VUFFQTtVQUNBO1VBQ0E7Ozs7O1dDdEJBO1dBQ0E7V0FDQTtXQUNBO1dBQ0EseUNBQXlDLHdDQUF3QztXQUNqRjtXQUNBO1dBQ0E7Ozs7O1dDUEE7Ozs7O1dDQUE7V0FDQTtXQUNBO1dBQ0EsdURBQXVELGlCQUFpQjtXQUN4RTtXQUNBLGdEQUFnRCxhQUFhO1dBQzdEOzs7Ozs7Ozs7Ozs7QUNObUQ7QUFFbkQsTUFBTSxDQUFDLE1BQU0sR0FBRztJQUNaLFFBQVEsQ0FBQyxTQUFTLEdBQUcsVUFBQyxLQUFvQjtRQUN0QyxnRUFBbUIsQ0FBQyxLQUFLLENBQUMsQ0FBQztJQUMvQixDQUFDO0FBQ0wsQ0FBQyIsInNvdXJjZXMiOlsid2VicGFjazovL3NjcmlwdHMvLi9odG14X3NuYWtlLnRzIiwid2VicGFjazovL3NjcmlwdHMvd2VicGFjay9ib290c3RyYXAiLCJ3ZWJwYWNrOi8vc2NyaXB0cy93ZWJwYWNrL3J1bnRpbWUvZGVmaW5lIHByb3BlcnR5IGdldHRlcnMiLCJ3ZWJwYWNrOi8vc2NyaXB0cy93ZWJwYWNrL3J1bnRpbWUvaGFzT3duUHJvcGVydHkgc2hvcnRoYW5kIiwid2VicGFjazovL3NjcmlwdHMvd2VicGFjay9ydW50aW1lL21ha2UgbmFtZXNwYWNlIG9iamVjdCIsIndlYnBhY2s6Ly9zY3JpcHRzLy4vbWFpbi50cyJdLCJzb3VyY2VzQ29udGVudCI6WyJmdW5jdGlvbiBzd2l0Y2hLZXkoZGlyZWN0aW9uOiBudW1iZXIpIHtcclxuICAgIGxldCBkaXJlY3Rpb25GaWVsZCA9IGRvY3VtZW50LmdldEVsZW1lbnRCeUlkKFwiZGlyZWN0aW9uXCIpIGFzIEhUTUxFbGVtZW50O1xyXG4gICAgZGlyZWN0aW9uRmllbGQuc2V0QXR0cmlidXRlKFwidmFsdWVcIiwgZGlyZWN0aW9uLnRvU3RyaW5nKCkpO1xyXG59XHJcblxyXG5leHBvcnQgZnVuY3Rpb24gc25ha2VLZXlEb3duSGFuZGxlcihldmVudDogS2V5Ym9hcmRFdmVudCkge1xyXG4gICAgaWYod2luZG93LmxvY2F0aW9uLnBhdGhuYW1lID09IFwiL2Z1bi1ub25zZW5zZS9odG14LXNuYWtlXCIpIHtcclxuICAgICAgICBzd2l0Y2goZXZlbnQua2V5KVxyXG4gICAgICAgIHtcclxuICAgICAgICAgICAgY2FzZSBcIkFycm93TGVmdFwiOiBcclxuICAgICAgICAgICAgY2FzZSBcIkFcIjpcclxuICAgICAgICAgICAgICAgIHN3aXRjaEtleSgxKTtcclxuICAgICAgICAgICAgICAgIGJyZWFrO1xyXG4gICAgICAgICAgICBjYXNlIFwiQXJyb3dVcFwiOlxyXG4gICAgICAgICAgICBjYXNlIFwiV1wiOlxyXG4gICAgICAgICAgICAgICAgc3dpdGNoS2V5KDIpO1xyXG4gICAgICAgICAgICAgICAgYnJlYWs7XHJcbiAgICAgICAgICAgIGNhc2UgXCJBcnJvd1JpZ2h0XCI6XHJcbiAgICAgICAgICAgIGNhc2UgXCJEXCI6XHJcbiAgICAgICAgICAgICAgICBzd2l0Y2hLZXkoMyk7XHJcbiAgICAgICAgICAgICAgICBicmVhaztcclxuICAgICAgICAgICAgY2FzZSBcIkFycm93RG93blwiOlxyXG4gICAgICAgICAgICBjYXNlIFwiU1wiOlxyXG4gICAgICAgICAgICAgICAgc3dpdGNoS2V5KDQpO1xyXG4gICAgICAgICAgICAgICAgYnJlYWs7XHJcbiAgICAgICAgfVxyXG4gICAgfVxyXG59XHJcbiIsIi8vIFRoZSBtb2R1bGUgY2FjaGVcbnZhciBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX18gPSB7fTtcblxuLy8gVGhlIHJlcXVpcmUgZnVuY3Rpb25cbmZ1bmN0aW9uIF9fd2VicGFja19yZXF1aXJlX18obW9kdWxlSWQpIHtcblx0Ly8gQ2hlY2sgaWYgbW9kdWxlIGlzIGluIGNhY2hlXG5cdHZhciBjYWNoZWRNb2R1bGUgPSBfX3dlYnBhY2tfbW9kdWxlX2NhY2hlX19bbW9kdWxlSWRdO1xuXHRpZiAoY2FjaGVkTW9kdWxlICE9PSB1bmRlZmluZWQpIHtcblx0XHRyZXR1cm4gY2FjaGVkTW9kdWxlLmV4cG9ydHM7XG5cdH1cblx0Ly8gQ3JlYXRlIGEgbmV3IG1vZHVsZSAoYW5kIHB1dCBpdCBpbnRvIHRoZSBjYWNoZSlcblx0dmFyIG1vZHVsZSA9IF9fd2VicGFja19tb2R1bGVfY2FjaGVfX1ttb2R1bGVJZF0gPSB7XG5cdFx0Ly8gbm8gbW9kdWxlLmlkIG5lZWRlZFxuXHRcdC8vIG5vIG1vZHVsZS5sb2FkZWQgbmVlZGVkXG5cdFx0ZXhwb3J0czoge31cblx0fTtcblxuXHQvLyBFeGVjdXRlIHRoZSBtb2R1bGUgZnVuY3Rpb25cblx0X193ZWJwYWNrX21vZHVsZXNfX1ttb2R1bGVJZF0obW9kdWxlLCBtb2R1bGUuZXhwb3J0cywgX193ZWJwYWNrX3JlcXVpcmVfXyk7XG5cblx0Ly8gUmV0dXJuIHRoZSBleHBvcnRzIG9mIHRoZSBtb2R1bGVcblx0cmV0dXJuIG1vZHVsZS5leHBvcnRzO1xufVxuXG4iLCIvLyBkZWZpbmUgZ2V0dGVyIGZ1bmN0aW9ucyBmb3IgaGFybW9ueSBleHBvcnRzXG5fX3dlYnBhY2tfcmVxdWlyZV9fLmQgPSAoZXhwb3J0cywgZGVmaW5pdGlvbikgPT4ge1xuXHRmb3IodmFyIGtleSBpbiBkZWZpbml0aW9uKSB7XG5cdFx0aWYoX193ZWJwYWNrX3JlcXVpcmVfXy5vKGRlZmluaXRpb24sIGtleSkgJiYgIV9fd2VicGFja19yZXF1aXJlX18ubyhleHBvcnRzLCBrZXkpKSB7XG5cdFx0XHRPYmplY3QuZGVmaW5lUHJvcGVydHkoZXhwb3J0cywga2V5LCB7IGVudW1lcmFibGU6IHRydWUsIGdldDogZGVmaW5pdGlvbltrZXldIH0pO1xuXHRcdH1cblx0fVxufTsiLCJfX3dlYnBhY2tfcmVxdWlyZV9fLm8gPSAob2JqLCBwcm9wKSA9PiAoT2JqZWN0LnByb3RvdHlwZS5oYXNPd25Qcm9wZXJ0eS5jYWxsKG9iaiwgcHJvcCkpIiwiLy8gZGVmaW5lIF9fZXNNb2R1bGUgb24gZXhwb3J0c1xuX193ZWJwYWNrX3JlcXVpcmVfXy5yID0gKGV4cG9ydHMpID0+IHtcblx0aWYodHlwZW9mIFN5bWJvbCAhPT0gJ3VuZGVmaW5lZCcgJiYgU3ltYm9sLnRvU3RyaW5nVGFnKSB7XG5cdFx0T2JqZWN0LmRlZmluZVByb3BlcnR5KGV4cG9ydHMsIFN5bWJvbC50b1N0cmluZ1RhZywgeyB2YWx1ZTogJ01vZHVsZScgfSk7XG5cdH1cblx0T2JqZWN0LmRlZmluZVByb3BlcnR5KGV4cG9ydHMsICdfX2VzTW9kdWxlJywgeyB2YWx1ZTogdHJ1ZSB9KTtcbn07IiwiaW1wb3J0IHsgc25ha2VLZXlEb3duSGFuZGxlciB9IGZyb20gXCIuL2h0bXhfc25ha2VcIjtcclxuXHJcbndpbmRvdy5vbmxvYWQgPSAoKSA9PiB7XHJcbiAgICBkb2N1bWVudC5vbmtleWRvd24gPSAoZXZlbnQ6IEtleWJvYXJkRXZlbnQpID0+IHtcclxuICAgICAgICBzbmFrZUtleURvd25IYW5kbGVyKGV2ZW50KTtcclxuICAgIH1cclxufVxyXG4iXSwibmFtZXMiOltdLCJzb3VyY2VSb290IjoiIn0=