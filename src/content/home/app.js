import {store} from '../../main.js';


let elTitleEl = document.querySelector("#display-name");
let name =  await store.get("name");
elTitleEl.textContent = `Welcome ${name.value} !`;

