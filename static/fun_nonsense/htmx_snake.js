const LEFT_KEY = 37;
const UP_KEY = 38;
const RIGHT_KEY = 39;
const DOWN_KEY = 40;
const W_KEY = 87;
const S_KEY = 83;
const D_KEY = 68;
const A_KEY = 65;

function switchKey(direction) {
    let directionField = document.getElementById("direction");
    directionField.setAttribute("value", direction);
}

window.onload = () => {
        console.log("WHATS UP");
    document.addEventListener("keydown", (event) => {
        console.log(event);
        switch(event.keyCode)
        {
            case LEFT_KEY: 
            case A_KEY:
                switchKey(1);
                break;
            case UP_KEY:
            case W_KEY:
                switchKey(2);
                break;
            case RIGHT_KEY:
            case D_KEY:
                switchKey(3);
                break;
            case DOWN_KEY:
            case S_KEY:
                switchKey(4);
                break;
        }

    });
};

