const Actions = (function actions() {
    return {
        init: () => {

            function loadTemplate(tplName) {
                const url = "/" + (tplName.length > 0 ? `?template=${tplName}` : "");
                location.href = url;
            }


            function sendFile(file) {
                const uri = "/store/store_file";
                const xhr = new XMLHttpRequest();
                const fd = new FormData();

                xhr.open("POST", uri, true);
                xhr.onreadystatechange = function () {
                    if (xhr.readyState == 4 && xhr.status == 200) {
                        const responseText = xhr.responseText;
                        if (responseText.startsWith("Error:")) console.error(responseText);
                        else {
                            const [typeName, baseName] = responseText.split("/");
                            if (typeName == "template") {
                                loadTemplate(baseName)
                            }
                        }
                    }
                };
                fd.append('myFile', file);
                // Initiate a multipart/form-data upload
                xhr.send(fd);
            }

            // const reload = document.getElementById("reload");
            const dropzone = document.getElementById("dropzone");
            const templateSelect = document.getElementById("ol-template");

            templateSelect.addEventListener("change", () => {
                loadTemplate(templateSelect.value);

            })

            dropzone.ondragover = dropzone.ondragenter = function (event) {
                event.stopPropagation();
                event.preventDefault();
            }

            dropzone.ondrop = function (event) {
                event.stopPropagation();
                event.preventDefault();

                const filesArray = event.dataTransfer.files;
                for (let i = 0; i < filesArray.length; i++) {
                    sendFile(filesArray[i]);
                }
            }
            /*
                        reload.addEventListener("click", () => {
                            const xhr = new XMLHttpRequest();
            
                            xhr.open('GET', '/admin/refresh_templates');
            
            
                            // 4. This will be called after the response is received
                            xhr.onload = function () {
                                if (xhr.status != 200) { // analyze HTTP status of the response
                                    console.error(`Error ${xhr.status}: ${xhr.statusText}`); // e.g. 404: Not Found
                                } else { // show the result
                                    if (xhr.response != 'Ok') alert(xhr.response);
                                    else console.log("Handlebars templates were reloaded.")
                                }
                            };
            
                            xhr.send();
            
                        });
            */
        }
    };
})();
