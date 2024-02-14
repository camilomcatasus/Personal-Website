import { snakeKeyDownHandler } from "./htmx_snake";


function toggleHidden(event: Event) {
    let target = event.target as HTMLElement;
    if (target.classList.contains("hidden")) 
    {
        target.classList.remove("hidden");
        target.classList.add("flex");
    }
    else 
    {
        target.classList.add("hidden");
        target.classList.remove("flex");
    }
}

window.onload = () => {
    document.onkeydown = (event: KeyboardEvent) => {
        snakeKeyDownHandler(event);
    }

    document.onclick = (event: Event) => {
        let target = event.target as HTMLElement;
        if(target.matches("#toggle-nav")) toggleHidden(event);
    }
}
