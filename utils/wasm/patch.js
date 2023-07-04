let audioContext;

function fnPatch(obj, key, log) {
    obj[key] = new Proxy(obj[key], {
        construct(target, args) {
            console.log(`new ${log}`);
            audioContext = new target(...args);
            return audioContext;
        },
        apply(target, thisArg, args) {
            console.log(log);
            return Reflect.apply(target, thisArg, args);
        },
    });
}

fnPatch(globalThis, 'AudioContext', 'AudioContext()');

function initAudioContext() {
    audioContext.resume();
    document.body.removeEventListener('mousedown', initAudioContext);
    document.body.removeEventListener('pointerdown', initAudioContext);
    document.body.removeEventListener('touchdown', initAudioContext);
}

document.body.addEventListener('mousedown', initAudioContext);
document.body.addEventListener('pointerdown', initAudioContext);
document.body.addEventListener('touchdown', initAudioContext);