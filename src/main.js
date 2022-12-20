const { invoke } = window.__TAURI__.tauri;
const { appWindow } = window.__TAURI__.window;
import { Store } from './index.mjs';

let elLoginButton = document.querySelector("#login");
const store = new Store('.settings.dat');

async function login() {
  let username = document.querySelector("#username").value;
  let password = document.querySelector("#password").value;
  let is_correct = await invoke("get_name", {username : username, password : password});
  if (is_correct === "") {
    return;
  }else {
    const cookie =  await invoke("get_cookie", {username : username, password : password});
    await store.set('username', {value: username});
    await store.set('password', {value: password});
    await store.set('name', { value: is_correct });
    await store.set('cookie', {value : cookie});
    window.location.href = "./content/home/home.html"; 
    return;
  }
}

if (elLoginButton){
  elLoginButton.addEventListener("click", login); 
}

export {store};

//need to update cookie when expired (1hour)