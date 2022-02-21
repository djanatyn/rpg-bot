local socket = require("socket.core")
local rapidjson = require("rapidjson")

local sock, err = socket.tcp()
sock:setoption("reuseaddr", true)
local res, err = sock:bind("127.0.0.1", 8080)
sock:listen(-1)
emu.message("server up")
local client = sock:accept()

emu.speedmode("normal")
while true do

   -- send memory to client
   local msg = {
      x = memory.readbyteunsigned(0x0064),
      y = memory.readbyteunsigned(0x0065)
   }
   client:send(rapidjson.encode(msg) .. "\n")

   -- advance to next frame
   emu.frameadvance() -- This essentially tells FCEUX to keep running

   -- wait for response from client
   local response = client:receive()

   -- display response from client
   emu.message(response)
end
