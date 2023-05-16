"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.createNonce = void 0;
function createNonce() {
    var nonceConst = Math.random() * 100000;
    var nonceBuffer = Buffer.alloc(4);
    nonceBuffer.writeUInt32LE(nonceConst, 0);
    return nonceBuffer;
}
exports.createNonce = createNonce;