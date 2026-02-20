console.log("welcome to glorp.io")

let glorpClicked = 0;

function clickGlorp() {
    glorpClicked++;
    const click_el = document.getElementById('click_count');
    click_el.innerText = glorpClicked.toString();
}