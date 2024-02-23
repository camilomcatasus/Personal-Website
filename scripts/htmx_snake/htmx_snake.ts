function switchKey(direction: number) {
    let directionField = document.getElementById("direction") as HTMLElement;
    directionField.setAttribute("value", direction.toString());
}

function inFocus() {
}

function snakeKeyDownHandler(event: KeyboardEvent) {
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

window.onload = () => {
    document.onkeydown = (event: KeyboardEvent) => {
        snakeKeyDownHandler(event);
    }

    let focusBlurDiv = document.getElementById("focus-blur");


    window.addEventListener("focusout", (event) => {
        console.log(event);
        if(!document.hasFocus()) {
            focusBlurDiv!.classList.remove("hidden");
        }
    });

    window.addEventListener("focusin", (event) => {
        console.log(event);
        if(document.hasFocus()) {
            focusBlurDiv!.classList.add("hidden");
        }
    });


    setInterval(() => {
        if(document.hasFocus()) {
            document.body.dispatchEvent(new CustomEvent("snake-step", {}));
        }
    }, 500);
}