function sendFile(file) {
    const uri = "/upload/bank_transactions";
    const xhr = new XMLHttpRequest();
    const fd = new FormData();

    xhr.open("POST", uri, true);
    xhr.onreadystatechange = function () {
        if (xhr.readyState == 4 && xhr.status == 200) {
            const responseText = xhr.responseText;
            if (responseText.startsWith("Error:")) console.error(responseText);
            else {
                console.log("UPLOAD response", responseText);
                // const [typeName, baseName] = responseText.split("/");


            }
        }
    };

    fd.append('bank_file', file);
    // Initiate a multipart/form-data upload
    console.dir(fd);
    for (var pair of fd.entries()) {
        console.log(pair[0] + ', ' + pair[1]);
    }
    xhr.send(fd);
}


const transferForm = document.getElementById("transfer");
const formFileName = document.getElementById('filename');
const dropzone = document.getElementById("dropzone");
let formFile = null;

transferForm.addEventListener("formdata", (e) => {
    const formData = e.formData;
    formData.append('transactions_file', formFile);
});

dropzone.addEventListener("dragover", event => {
    event.stopPropagation();
    event.preventDefault();
});

dropzone.addEventListener("dragenter", event => {
    if (typeof event == "object" && typeof event.dataTransfer == "object" && typeof event.dataTransfer.items == "object" && event.dataTransfer.items.length == 1) {
        dropzone.dataset.dropStatus = "accept";
    } else {
        if (typeof event == "object" && typeof event.dataTransfer == "object") console.log("Reject: ", event.dataTransfer);
        dropzone.dataset.dropStatus = "reject";
    }
    event.stopPropagation();
    event.preventDefault();
});


dropzone.addEventListener("dragleave", event => {
    dropzone.dataset.dropStatus = "neutral";
    event.stopPropagation();
    event.preventDefault();
});

dropzone.addEventListener("drop", event => {
    dropzone.dataset.dropStatus = "send";
    const filesArray = event.dataTransfer.files;
    for (let i = 0; i < filesArray.length && i < 1; i++) {
        formFile = filesArray[i];
        transferForm.submit();
    }
    event.stopPropagation();
    event.preventDefault();
});

