const js = import("./pkg/surf.js");
var wasm_module;
js.then(js => wasm_module = js);

var video = document.getElementById('videoElement');
var c1 = document.getElementById("canvasElement");
var ctx1 = c1.getContext("2d");
var height = 375;
var width = 500;

playVideo = () => {
  if(navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
    navigator.mediaDevices.getUserMedia({ video: true }).then(function(stream) {
      video.srcObject = stream;
      video.play();
    });
  };
}

timerCallback = () => {
  if (video.paused || video.ended)
    return;  
  computeFrame();
  setTimeout(timerCallback, 16);
};

doLoad = () => video.addEventListener("play", timerCallback, false);

computeFrame = () => {
  ctx1.drawImage(video, 0, 0, width, height);
  var frame = ctx1.getImageData(0, 0, width, height);            

  frame.data.set(wasm_module.transform_to_black_and_white(frame.data));

  ctx1.putImageData(frame, 0, 0);
  return;
};
doLoad();
setTimeout(playVideo, 2000);