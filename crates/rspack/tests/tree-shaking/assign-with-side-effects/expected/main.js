(self['webpackChunkwebpack'] = self['webpackChunkwebpack'] || []).push([["main"], {
"./app.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "app", {
    enumerable: true,
    get: ()=>app
});
const _lib = __webpack_require__("./lib.js");
function app() {}
app.prototype.result = _lib.result;
},
"./index.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
const _app = __webpack_require__("./app.js");
(0, _app.app)();
},
"./lib.js": function (module, exports, __webpack_require__) {
"use strict";
Object.defineProperty(exports, "__esModule", {
    value: true
});
Object.defineProperty(exports, "result", {
    enumerable: true,
    get: ()=>result
});
const result = 20000;
},

},function(__webpack_require__) {
__webpack_require__("./index.js");
}
]);