<script>
  import { getNotificationsContext } from "svelte-notifications";
  import MediaQuery from "./MediaQuery.svelte";

  const { addNotification } = getNotificationsContext();
  const { remote } = require("electron");
  import Bluehood from "./bluehood.js";
  const win = remote.getCurrentWindow();

  let is_encrypt = true;
  let key_value = "test";

  win.setResizable(false);

  function max_windows() {
    if (win.isMaximized()) {
      win.unmaximize();
    } else {
      win.maximize();
    }
  }

  function error_msg(msg) {
    addNotification({
      text: msg,
      position: "bottom-right",
      removeAfter: 3500,
      type: "danger",
    });
  }

  function success_msg(msg) {
    addNotification({
      text: msg,
      position: "bottom-right",
      removeAfter: 3500,
      type: "success",
    });
  }

  function use_function(func_name) {
    if (key_value.length > 0) {
      Bluehood.command(func_name, key_value, error_msg, success_msg);
    } else {
      error_msg("Key not provided");
    }
  }
</script>

<main>
  <div id="top-fream">
    <table id="top-fream-btn-bar">
      <tr>
        <td on:click={() => window.close()} class="top-fream-btn" id="exit-btn"
          >x</td>
        <td on:click={() => max_windows()} class="top-fream-btn" id="max-btn"
          >+</td>
        <td on:click={() => win.minimize()} class="top-fream-btn" id="min-btn"
          >-</td>
      </tr>
    </table>
  </div>
  <div id="header">
    <MediaQuery query="(max-width: 999px)" let:matches>
      {#if matches}
        <img alt="logo" src=".\\graphic package\\logo.svg" id="logo" />
      {:else}
        <img alt="logo" src=".\\graphic package\\banner.svg" id="logo" />
      {/if}
    </MediaQuery>
  </div>
  <div id="main-content">
    <h1 class="main-title">Convert your files into encrypted image</h1>
    <h2 class="key-input-label">Enter your key</h2>
    <input type="password" bind:value={key_value} />
    <h2 class="key-input-label">Choose function</h2>
    <input
      type="button"
      value="Decrypt"
      on:click={() => use_function("decrypt")} />
    <input
      type="button"
      value="Encrypt"
      on:click={() => use_function("encrypt")} />
  </div>
</main>

<style>
  @import url("https://fonts.googleapis.com/css2?family=Spartan:wght@100;200;300;400;500;600;700;800;900&display=swap");
  @import url("https://fonts.googleapis.com/css2?family=Montserrat:wght@500&display=swap");

  main {
    width: 100vw;
    height: 100vh;
    background-color: #171747;
    -webkit-user-select: none;
    -webkit-app-region: drag;
  }
  #top-fream {
    font-weight: 100;
    font-family: "Spartan", sans-serif;
    background-color: #171747;
    width: 100vw;
    height: 30px;
    font-size: 35px;
    -webkit-user-select: none;
    -webkit-app-region: drag;
  }
  #top-fream-btn-bar {
    position: absolute;
    float: left;
    left: -2px;
    top: -4px;
  }
  .top-fream-btn {
    -webkit-user-select: select;
    -webkit-app-region: no-drag;
    display: inline-block;
    text-align: center;
    left: 0;
    line-height: 35px;
    width: 40px;
    height: 40px;
    color: #f0f0f0;
  }
  #max-btn {
    line-height: 40px;
    background-color: #2069ad00;
  }
  #exit-btn:hover {
    background-color: #d31f1f;
  }
  #min-btn:hover {
    background-color: #ddb01d;
  }
  #max-btn:hover {
    background-color: #1e8cf3;
  }
  #logo {
    -webkit-user-select: none;
    -webkit-app-region: drag;
  }
  #main-content {
    text-align: center;
  }
  #main-content h1 {
    font-family: "Montserrat", sans-serif;
    color: #f0f0f0;
    user-select: none;
    font-size: 5vh;
    font-weight: 500;
  }
  .key-input-label {
    font-family: "Montserrat", sans-serif;
    color: #f0f0f0;
    user-select: none;
    font-size: 4vh;
  }
  #main-content input {
    background-color: #f0f0f0;
    color: #171747;
    font-size: 3vh;
    text-align: center;
    -webkit-app-region: no-drag;
  }
  #main-content input[type="password"] {
    margin-top: 0;
    border: none;
    border-radius: 0.25rem;
    height: 5.5vh;
    width: 25vw;
    font-size: 6vh;
  }
  #main-content input[type="button"] {
    font-family: "Spartan", sans-serif;
    font-weight: 200;
    margin-top: 0;
    width: 12.5vw;
    height: 5vh;
    border: none;
    border-radius: 0.25rem;
    height: 5.5vh;
  }
  @media (max-width: 999px) {
    #logo {
      width: 60vw;
      display: block;
      margin-left: auto;
      margin-right: auto;
    }
    #main-content input[type="button"] {
      width: 40%;
    }
    #main-content input[type="password"] {
      width: 80vw;
    }
  }
</style>
