const net = require('net')

net.createServer(socket => {
  console.log("got a socket", socket.localAddress);
  socket.write("Hello Socket!");
  socket.on('data', function(data){
    console.log('Echoing: %s', data.toString())
    socket.write(data.toString())
  });
  socket.on("close", () => {
    console.log("socket gone...");
  })
  socket.on("error", () => {
    console.error("oh no!!!");
  })
}).listen(3001, () => console.log("listening..."));