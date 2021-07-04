<script>
  import { getNotificationsContext } from "svelte-notifications";

  const { addNotification } = getNotificationsContext();
  const { remote } = require("electron");
  const dialog = require("electron").remote.dialog;
  const win = remote.getCurrentWindow();

  let is_encrypt = true;
  let key_value = "";

  win.setResizable(false);

  function max_windows() {
    if (win.isMaximized()) {
      win.unmaximize();
    } else {
      win.maximize();
    }
  }

  function use_function(func_name) {
    if (key_value.length > 0) {
      dialog
        .showOpenDialog({ properties: ["openFile"] })
        .then(function (response) {
          if (!response.canceled) {
            // handle fully qualified file name
            console.log(response.filePaths[0]);
          } else {
            error_msg("no file selected");
          }
        });
    } else {
      error_msg("Key not provided");
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
    <img alt="logo" src=".\graphic package\banner.svg" id="logo" />
  </div>
  <div id="main-content">
    <h1>Convert your files into encrypted image</h1>
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
    width: 12.5vw;
    height: 5vh;
  }
  #main-content input[type="button"] {
    margin-top: 0;
    width: 12.5vw;
    height: 5vh;
    border: 0px;
  }
</style>
