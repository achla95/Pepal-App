import {store} from '../../main.js';
const { invoke } = window.__TAURI__.tauri;

const replaceToLong = (element,len) => {
    if (len > element.lenght){
        return element;
    }
    return element.substring(0,len) + "...";
}
const moyenneGenerale = (sum,len) => {
    if (len === 0) {
        return 0;
    }
    return sum/len;
}
let username = await store.get('username');
let password = await store.get('password');
const room = await invoke("get_notes", {username : username.value, password : password.value}); 

let sum = 0;
let len = 0;
const elTbody = document.querySelector("tbody");
room.forEach((element) => {
    const elTr = document.createElement("tr");
    elTbody.appendChild(elTr);
    const elDateTd = document.createElement("td");
    const elMatiereTD = document.createElement("td");
    const elNoteTd = document.createElement("td");
    elDateTd.textContent = element[0];
    elMatiereTD.textContent = replaceToLong(element[1],10);
    elNoteTd.textContent = element[2];
    sum += parseInt(element[2]);
    len+=1;
    elTr.appendChild(elDateTd);
    elTr.appendChild(elMatiereTD);
    elTr.appendChild(elNoteTd);
});

const elMgTr = document.createElement("tr");
elTbody.appendChild(elMgTr);
const elTtileTd = document.createElement("td");
elTtileTd.textContent = "Moyenne Générale";
const elEmptyTd = document.createElement("td");
const elMgTd = document.createElement("td");
elMgTd.textContent = moyenneGenerale(sum,len);
elMgTr.appendChild(elTtileTd);
elMgTr.appendChild(elEmptyTd);
elMgTr.appendChild(elMgTd);
