import * as wand_app from "wand-example";

window.$mobile = /mobile/i.test(window.navigator.userAgent) || typeof window.orientation !== 'undefined'

const app = wand_app.Application.new();
const canvas = document.getElementById("canvas");
const resize = () => {
    canvas.width = document.documentElement.clientWidth;
    canvas.height = document.documentElement.clientHeight;
    app.on_size_change();
    app.draw();
}

resize();

window.addEventListener("resize", resize);

