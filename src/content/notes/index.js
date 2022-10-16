import {store} from '../../main.js';
const { invoke } = window.__TAURI__.tauri;

let username = await store.get('username');
let password = await store.get('password');
const notesEl = document.querySelector("#notes");
const room = await invoke("get_notes", {username : username.value, password : password.value}); 

const elTbody = document.querySelector("tbody");
room.forEach((element) => {
    const elTr = document.createElement("tr");
    elTbody.appendChild(elTr);
    const eltd1 = document.createElement("td");
    const eltd2 = document.createElement("td");
    const eltd3 = document.createElement("td");
    eltd1.textContent = element[0];
    eltd2.textContent = element[1];
    eltd3.textContent = element[2];
    elTr.appendChild(eltd1);
    elTr.appendChild(eltd2);
    elTr.appendChild(eltd3);
});