local config = require('config')
local util = require('util')

Window = createForm(false)

local WINDOW_FONT = 'Lolita'
local WINDOW_ICON = 'assets/imouto.ico'

local function draw()
    util.loadFont(WINDOW_FONT)

    Window.setCaption(config.window.title)
    Window.setWidth(config.window.width)
    Window.setHeight(config.window.height)
    Window.setColor(clWhite)
    Window.setOnClose(closeCE)
    Window.Font.setName(WINDOW_FONT)
    Window.Icon.loadFromFile(WINDOW_ICON)
end

local function show()
    Window.centerScreen()
    Window.show()
end

return {
    draw = draw,
    show = show,
}
