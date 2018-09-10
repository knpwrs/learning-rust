const http = require('http');
const fs = require('fs');

const server = http.createServer((req, res) => {
  if (req.method === 'GET' && req.url === '/') {
    fs.createReadStream('index.html').pipe(res);
  } else if (req.method === 'GET' && req.url === '/sleep') {
      setTimeout(() => {
        fs.createReadStream('index.html').pipe(res);
      }, 5000);
  } else {
    res.writeHead(404);
    fs.createReadStream('404.html').pipe(res);
  }
});
server.listen(7979);
