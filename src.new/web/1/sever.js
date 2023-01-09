var http = require("http");
var url = require("url");
var Sever = http.createServer(function(req, res){
    var queryObj = url.parse(req.url, true).query;
    var name = queryObj.name;
    res.writeHead(200, {"Content-Type":"text/html;charset=UTF-8"});
    res.end("你好！"+ name + "!");
});
Sever.listen(8000, "127.0.0.1");
console.log("Sever is running at 127.0.0.1:8000");
