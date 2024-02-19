import { snakeKeyDownHandler } from "./htmx_snake";


function toggleClass(elem: HTMLElement, class1: string, class2: string) {
    if (elem.classList.contains(class1))
    {
        elem.classList.remove(class1);
        elem.classList.add(class2);
    }
    else 
    {
        elem.classList.add(class1);
        elem.classList.remove(class2);
    }
}

function secondaryNavLoaded(path:string) {
    let nav = document.getElementById("secondary-nav");
    let allNavItems = nav.querySelectorAll("a");
    allNavItems.forEach((navItem) => {
        navItem.classList.remove("underline");
    })

    let selectedAnchor = nav.querySelector(`a[href="${path}"]`) as HTMLElement;
    selectedAnchor.classList.add("underline");
}

function sectionSetup() {
    let path_part = window.location.pathname.split("/");
    console.log(path_part);
    if(path_part.length >= 2) {
        switch(path_part[1]) {
            case "fun-nonsense": 
                secondaryNavLoaded(window.location.pathname); 
                break;
        }
    }

}

window.onload = () => {
    document.onkeydown = (event: KeyboardEvent) => {
        snakeKeyDownHandler(event);
    }

    document.onclick = (event: Event) => {
        let target = event.target as HTMLElement;
        if (target.matches(":is(#toggle-nav-button, #toggle-nav-button *)")) {
            let secondaryNav = document.getElementById("secondary-nav");
            let parent = secondaryNav.parentElement;
            toggleClass(secondaryNav, "w-0", "w-60");
            toggleClass(parent, "w-0", "w-60");
            
            document.getElementById("toggle-nav-button").classList.toggle("is-active");
        }
        else if (target.matches("#secondary-nav > a")) {
            secondaryNavLoaded(target.getAttribute("href"));
        }
    }

    sectionSetup();
}


