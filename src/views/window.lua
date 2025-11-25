local util = require('util')
local app = require('config.app')

Window = createForm(false)

local WINDOW_FONT = 'Lolita'
local WINDOW_ICON = 'assets/imouto.ico'

local function draw()
    util.loadFont(WINDOW_FONT)

    Window.setCaption(app.window.title)
    Window.setWidth(app.window.width)
    Window.setHeight(app.window.height)
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
