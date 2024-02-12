import { snakeKeyDownHandler } from "./htmx_snake";

window.onload = () => {
    document.onkeydown = (event: KeyboardEvent) => {
        snakeKeyDownHandler(event);
    }
}
