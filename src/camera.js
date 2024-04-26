document.addEventListener("DOMContentLoaded", function () {
    let camera = document.getElementById('camera');
    if (!camera) {
        return;
    }

    if (!navigator.mediaDevices) {
        return;
    }

    if (!navigator.mediaDevices.getUserMedia) {
        return;
    }

    navigator.mediaDevices.getUserMedia({ video: true }).then(function (stream) {
        camera.srcObject = stream;
        camera.play();
    }).catch(function (error) {
        console.log(error);
    });
});