const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;
// const { confirm } = window.__TAURI__.dialog;
import { Store } from './index.mjs';

let elLoginButton = document.querySelector("#login");
const store = new Store('.settings.dat');

async function login() {
  let username = document.querySelector("#username").value;
  let password = document.querySelector("#password").value;
  let is_correct = await invoke("get_name", {username : username, password : password});
  if (is_correct === "") {
    console.log("incorrect login");
    return;
  }else {
    await store.set('username', {value: username});
    await store.set('password', {value: password});
    await store.set('name', { value: is_correct });
    window.location.href = "./content/home/home.html"; 
    return;
  }
}

if (elLoginButton){
  elLoginButton.addEventListener("click", login); 
}

export {store};
