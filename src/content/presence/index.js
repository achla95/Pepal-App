import {store} from '../../main.js';
const { invoke } = window.__TAURI__.tauri;

let username = await store.get('username');
let password = await store.get('password');
const roomEl = document.querySelectorAll(".salle");
const room = await invoke("get_room", {username : username.value, password : password.value});

roomEl.forEach((el) =>{
    el.textContent = room;
})
