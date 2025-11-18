local config = require('config')
local util = require('util')

Cover = createImage(Window)

local COVER_WIDTH = 180
local COVER_UNLINK = 'card0001.jpg'

local function draw()
    Cover.setWidth(COVER_WIDTH)
    Cover.setHeight(config.window.height)
end

local function loadImage(filename)
    Cover.loadImageFromFile(filename)
end

local function link()
    loadImage(util.getRandomFileByFolder('assets/images/card', COVER_UNLINK))
end

local function unlink()
    loadImage('assets/images/card/' .. COVER_UNLINK)
end

return {
    draw = draw,
    link = link,
    unlink = unlink,
}
