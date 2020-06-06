var AudioContext = window.AudioContext || window.webkitAudioContext;
var audioCtx = new AudioContext();

const track = document.getElementById('test-track');

const otherTrack = new Audio('./assets/teleport.aif');

console.log(otherTrack)
console.log(track)

document.getElementById('controls').appendChild(otherTrack)

// audioCtx.destination

otherTrack.play()
