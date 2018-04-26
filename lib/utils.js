export function memcopy(b1, b2, offset) {
    new Uint8Array(b2, offset, b1.byteLength).set(new Uint8Array(b1));
};