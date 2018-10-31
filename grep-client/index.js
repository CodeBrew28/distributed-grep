var WebSocket = require("ws");
ws = new WebSocket('ws://104.196.24.172:3000');
ws.onopen = function (evt) {
   ws.send("Hello!");
};