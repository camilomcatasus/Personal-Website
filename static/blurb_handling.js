

function displayText(sender, sentText) {
    var text = sender.getAttribute("api-text");
    if (!text)
    {
        text = sentText;
    }
    var index = 1;
    var consoleTyper = setInterval(function() {
        
        sender.firstElementChild.innerHTML = text.substring(0, index);
        if(text.length <= index)
        {
            sender.classList.add("opacity-0");
            sender.addEventListener('transitionend', () => handleTransitionEnd(sender));
            clearInterval(consoleTyper);
            return;
        }

        let indexChange = 1;
        if (text.charAt(index + indexChange) == "<")
        {
            while (text.charAt(index + indexChange) != ">")
            {
                indexChange += 1;
            }
        }

        index += indexChange;
    }, 50);
}

function handleTransitionEnd(sender) {
    let firstComputedStatus = window.getComputedStyle(sender);
    if (firstComputedStatus.getPropertyValue("opacity") == 0) {
        setTimeout(() => {
            let currentComputedStatus = window.getComputedStyle(sender);
            if(currentComputedStatus.getPropertyValue("opacity") == 0) {
                sender.remove();
            }
        }, 1000);
    }
}

function getGridHeight() {
    return document.getElementById('landing-page-grid').clientHeight;
}

function getGridWidth() {
    return document.getElementById('landing-page-grid').clientWidth;
}

function getRects() {
    let grid = document.getElementById('landing-page-grid');
    let childRects = grid.children;
    let rectList = [];
    for (let childRect of childRects) {
        let col = parseInt(childRect.getAttribute("x"));
        let row = parseInt(childRect.getAttribute("y"));
        
        if(col && row ) {
            console.log(col, row);
            rectList.push({
                "col": col,
                "row": row
            });
        }
    }
    return rectList;
}

htmx.onLoad((elt) => {
    if(elt.getAttribute("api-text")) {
        displayText(elt);
    }
});
