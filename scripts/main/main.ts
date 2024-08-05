let nav_toggle_button = document.getElementById("toggle-second-nav-button")

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

function primaryNavLoaded() {
    let secondary_nav = document.getElementById("secondary-nav");
    if (secondary_nav != null) {
        nav_toggle_button.classList.remove("hidden");
    }
    else {
        nav_toggle_button.classList.add("hidden");
    }
}

function secondaryNavLoaded(path:string) {
    let nav = document.getElementById("secondary-nav");
    let allNavItems = nav!.querySelectorAll("a");
    allNavItems.forEach((navItem) => {
        navItem.classList.remove("underline");
    })

    let selectedAnchor = nav!.querySelector(`a[href="${path}"]`) as HTMLElement;
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

nav_toggle_button.onclick = (event: Event) => {
    let secondaryNav = document.getElementById("secondary-nav") as HTMLElement;
    let parent = secondaryNav.parentElement as HTMLElement;
    toggleClass(secondaryNav, "w-0", "w-60");
    toggleClass(parent, "w-0", "w-60");
    
    document.getElementById("toggle-second-nav-button").classList.toggle("is-active");
}

document.addEventListener("htmx:afterSettle", (event: Event) => {
    primaryNavLoaded()
})

document.onclick = (event: Event) => {
    let target = event.target as HTMLElement;
    if (target.matches("#secondary-nav > a")) {
        secondaryNavLoaded(target.getAttribute("href")!);
    }
    else if (target.matches("#primary-nav > a")) {

    }
}

sectionSetup();
