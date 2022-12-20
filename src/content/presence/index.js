import {store} from '../../main.js';
const { invoke } = window.__TAURI__.tauri;

const cookie = await store.get('cookie');
const roomEl = document.querySelectorAll(".salle");
const room = await invoke("get_room", {cookie : cookie.value});

roomEl.forEach((el) =>{
    el.textContent = room;
})

const checkIfCourse = () => {
    if (room === "Pas de cours !") {
        let cancel_morning = document.querySelector("#matin");
        cancel_morning.style.pointerEvents = "none";
        let cancel_noon = document.querySelector("#midi");
        // it's not really clear (i'm cheating a little bit but fix coming soon)
        cancel_noon.style.pointerEvents = "none";

        return false;
    }
    return true;
}
checkIfCourse();


let morning = document.querySelector("#matin");
let noon = document.querySelector("#midi");

if (morning) {
    morning.addEventListener("click", async () => {
        if (checkIfCourse()) {
            let presence = await invoke("presence", {cookie : cookie.value});
            console.log("presence set");    
        }else {
            console.log("pas de cours");
        }

    });
}
if (noon) {
    noon.addEventListener("click", async () => {
        if (checkIfCourse()) {
            let presence = await invoke("presence", {cookie : cookie.value});
            console.log("presence set");
        }else {
            console.log("pas de cours");
        }

    });
}



