function on(name) {
    var elements = document.getElementsByClassName(name);
    for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        elem.style.backgroundColor = "pink";
    }
}
function off(name) {
    var elements = document.getElementsByClassName(name);
    for (var i = 0; i < elements.length; i++) {
        var elem = elements[i];
        elem.style.backgroundColor = "white";
    }
}

var display_menu = false;
function toggle() {
    if (display_menu == true) {
        display_menu = false;
        document.getElementById("nav").style.left = "-21%";
        document.getElementById("content").style.marginLeft = "0%";
        document.getElementById("menu").style.left = "1em";
/*        if(document.getElementById("top")) {
            document.getElementById("top").style.left = "0";
        }
        if(document.getElementById("footer")) {
            document.getElementById("footer").style.marginLeft = "0%";
        }*/
    } else {
        display_menu = true;
        document.getElementById("nav").style.left = "0";
        document.getElementById("content").style.marginLeft = "20%";
        document.getElementById("menu").style.left = "20%";
/*        if(document.getElementById("top")) {
            document.getElementById("top").style.left = "20%";
        }
        if(document.getElementById("footer")) {
            document.getElementById("footer").style.marginLeft = "20%";
        }*/
    }
}
