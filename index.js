const js = import("./pkg/surf.js");
let wasm_module;

let video = document.getElementById('videoElement');
let canvas = document.getElementById("canvasElement");
let context = canvas.getContext("2d");
let height = 375;
let width = 500;

playVideo = () => {
  if(navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
    navigator.mediaDevices.getUserMedia({ video: true }).then(function(stream) {
      video.srcObject = stream;
      video.play();
    });
  };
}

computeFrame = () => {
  context.drawImage(video, 0, 0, width, height);
  let frame = context.getImageData(0, 0, width, height);

  frame.data.set(wasm_module.transform_to_black_and_white(frame.data));
  
  context.putImageData(frame, 0, 0);
};

video.addEventListener("play", () => setInterval(computeFrame, 16), false);


js.then(js => {
  wasm_module = js;
  playVideo();
});