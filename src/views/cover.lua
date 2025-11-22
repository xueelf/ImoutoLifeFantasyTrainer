local config = require('config')
local util = require('util')

Cover = createImage(Window)

local COVER_WIDTH = 180
local COVER_DETACH_CARD = 'card0001.jpg'

local function draw()
    Cover.setWidth(COVER_WIDTH)
    Cover.setHeight(config.window.height)
end

local function loadImage(filename)
    Cover.loadImageFromFile(filename)
end

local function attach()
    loadImage(util.getRandomFileByFolder('assets/images/card', COVER_DETACH_CARD))
end

local function detach()
    loadImage('assets/images/card/' .. COVER_DETACH_CARD)
end

return {
    draw = draw,
    attach = attach,
    detach = detach,
}
