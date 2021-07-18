const dialog = require("electron").remote.dialog;
const fs = require("fs");
const PNG = require("pngjs").PNG;

import init, {
    decrypt_data,
    encrypt_data
} from "./pkg/app.js";

export class Bluehood {
    static command(func_name, key, error_msg, success_msg) {

        open_file_dialog().then((open_response) => {
            init().then(() => {
                if (!open_response.canceled) {
                    const open_file_path = open_response.filePaths[0];
                    switch (func_name) {
                        case "encrypt":
                            save_image_dialog().then((save_response) => {
                                if (!save_response.canceled) {
                                    const save_file_path = save_response.filePath;
                                    let encrypt_data_list = Array.from(encrypt_data(fs.readFileSync(open_file_path), key, open_file_path.split('\\').slice(-1)[0]));

                                    const file_size = Math.ceil((Math.sqrt((encrypt_data_list.length) / 3)));

                                    for (let i = 0; file_size ** 2 * 3 != encrypt_data_list.length; i++) {
                                        encrypt_data_list.push(0);
                                    }

                                    let image = new PNG({
                                        width: file_size,
                                        height: file_size,
                                        colorType: 2,
                                        inputColorType: 2,
                                        bitDepth: 8,
                                    });

                                    image.data = encrypt_data_list

                                    image.pack().pipe(fs.createWriteStream(save_file_path));

                                    console.log(image);
                                }
                            });
                            break;
                        default:
                            break;
                    }

                }
            });
        });
    }
}



function save_image_dialog() {
    return dialog.showSaveDialog({
        properties: ["createDirectory"],
        title: "Save encrypt image - BlueHood",
        defaultPath: "Desktop\\output.png",
        buttonLabel: "Save encrypt image",
        filters: [{
            name: "Image",
            extensions: ["png"],
        }, ],
    });
}

function save_file_dialog(filename) {
    return dialog.showSaveDialog({
        properties: ["createDirectory"],
        title: "Save decrypt file - BlueHood",
        defaultPath: `Desktop\\${filename}`,
        buttonLabel: "Save decrypt file",
        filters: [{
            name: "Original",
            extensions: [filename.split(".")[1]],
        }, ],
    });
}

function open_file_dialog() {
    return dialog.showOpenDialog({
        properties: ["openFile"],
    });
}

export default Bluehood;