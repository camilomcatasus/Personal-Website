function switchKey(direction: number) {
    let directionField = document.getElementById("direction") as HTMLElement;
    directionField.setAttribute("value", direction.toString());
}

export function snakeKeyDownHandler(event: KeyboardEvent) {
    if(window.location.pathname == "/fun-nonsense/htmx-snake") {
        switch(event.key)
        {
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
