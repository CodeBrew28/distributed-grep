var WebSocketServer = require("ws").Server;
var wss = new WebSocketServer({ port: 8080 });
wss.on("connection", function (ws, req) {
    // const ip = req.headers['x-forwarded-for'].split(/\s*,\s*/)[0];
   console.info("websocket connection open on " + String(req.headers['x-forwarded-for']));

   ws.on('message', function incoming(data) {
    console.log(data);
  });
});