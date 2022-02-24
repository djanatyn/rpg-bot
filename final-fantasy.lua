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
      -- menu selection on new game screen (seems to represent cursor height)
      -- 0x58 = continue, 0x80 = new game
      { tags = {"menu selection"}, value = memory.readbyteunsigned(0x0041) },
      -- CHARACTER CREATION
      -- which character are we creating?
      -- 0x00 = 1, 0x10 = 2, 0x20 = 3, 0x30 = 4
      { tags = {"character selection"}, value = memory.readbyteunsigned(0x0067) },
      -- character classes
      -- 0x00: fighter        0x20: thief
      -- 0x40: black belt     0x60: red mage
      -- 0x80: white mage     0xA0: black mage
      { tags = {"slot 1 class"}, value = memory.readbyteunsigned(0x0201) },
      { tags = {"slot 2 class"}, value = memory.readbyteunsigned(0x0219) },
      { tags = {"slot 3 class"}, value = memory.readbyteunsigned(0x0231) },
      { tags = {"slot 4 class"}, value = memory.readbyteunsigned(0x0249) },
      -- coordinates for cursor in name selection
      { tags = {"menu cursor x"}, value = memory.readbyteunsigned(0x0064) },
      { tags = {"menu cursor y"}, value = memory.readbyteunsigned(0x0065) },
      -- world map coordinates
      { tags = {"world map x"}, value = memory.readbyteunsigned(0x0027) },
      { tags = {"world map y"}, value = memory.readbyteunsigned(0x0028) },
      -- battle cursor
      -- x: 0x0200, 0x0204, 0x0208, 0x020C
      -- y: 0x0203, 0x0207, 0x020B, 0x020F
      -- { x = 0x9E, y = 0x60 } = FIGHT
      -- { x = 0xAE, y = 0x60 } = MAGIC
      -- { x = 0xBE, y = 0x60 } = DRINK
      -- { x = 0xCE, y = 0x60 } = ITEM
      -- { x = 0x9E, y = 0x90 } = RUN
      { tags = {"battle cursor x"}, value = memory.readbyteunsigned(0x0200) },
      { tags = {"battle cursor y"}, value = memory.readbyteunsigned(0x0203) }
   }
   client:send(rapidjson.encode(msg) .. "\n")

   -- advance to next frame
   emu.frameadvance() -- This essentially tells FCEUX to keep running

   -- wait for response from client
   local response = client:receive()

   -- set joypad input
   joypad.set(1, rapidjson.decode(response))

   -- display response from client
   emu.message(response)
end
