const {
	app,
	BrowserWindow
} = require('electron')

app.on('ready', () => {
	window = new BrowserWindow({
		width: 1000,
		height: 600,
		frame: false,
		webPreferences: {
			nodeIntegration: true,
			enableRemoteModule: true
		},
	})

	window.loadFile('index.html')
})

app.on('window-all-closed', () => app.quit())