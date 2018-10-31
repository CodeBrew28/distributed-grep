var WebSocket = require("ws");
ws = new WebSocket('ws://localhost:8080');
ws.onopen = function (evt) {
   ws.send("Hello!");
};