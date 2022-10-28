import {store} from '../../main.js';
const { invoke } = window.__TAURI__.tauri;

const cookie = await store.get('cookie');
const roomEl = document.querySelectorAll(".salle");
const room = await invoke("get_room", {cookie : cookie.value});

roomEl.forEach((el) =>{
    el.textContent = room;
})
