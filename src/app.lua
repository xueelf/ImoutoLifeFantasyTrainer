local util = require("util")
local window = require("views.window")
local cover = require("views.cover")
local feature = require("views.feature")
local menu = require("views.menu")

local font_name = "Lolita"
local file_cover = "assets/images/card0002.jpg"
local file_icon = "assets/imouto.ico"

local function create()
    util.loadFont(font_name)

    local Window = window.create()
    local Cover = cover.create(Window)
    local Feature = feature.create(Window)
    local Menu = menu.create(Window)

    Cover.loadImageFromFile(file_cover)
    Window.Font.setName(font_name)
    Window.Icon.loadFromFile(file_icon)
    Window.centerScreen()
    Window.show()

    util.playVoice("onj001")
end

return {
    create = create,
}
