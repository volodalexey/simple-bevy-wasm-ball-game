import init from './simple-bevy-wasm-ball-game.js'

async function run() {
    const loader = document.querySelector('.loader')
    if (loader != null) {
        loader.parentElement?.removeChild(loader)
    }
    init()

    // disable touch context menu
    document.addEventListener('contextmenu', (e) => {
        if (e.pointerType === "touch") {
            e.preventDefault();
        }
    });
}

run().catch((err) => {
    console.error(err)
    const errorMessageDiv = document.querySelector('.error-message')
    if (errorMessageDiv != null) {
        errorMessageDiv.classList.remove('hidden')
        errorMessageDiv.innerText = ((Boolean(err)) && (Boolean(err.message))) ? err.message : err
    }
    const errorStackDiv = document.querySelector('.error-stack')
    if (errorStackDiv != null) {
        errorStackDiv.classList.remove('hidden')
        errorStackDiv.innerText = ((Boolean(err)) && (Boolean(err.stack))) ? err.stack : ''
    }
    const canvas = document.querySelector('canvas')
    if (canvas != null) {
        canvas.parentElement?.removeChild(canvas)
    }
})
