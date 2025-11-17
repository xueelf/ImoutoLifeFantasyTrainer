local config = require('config')

local width, height = 180, config.window.height
local file_cover = 'assets/images/card/card0002.jpg'

local function create(owner)
    local Cover = createImage(owner)

    local function loadImage()
        Cover.loadImageFromFile(file_cover)
    end

    Cover.Width, Cover.Height = width, height
    Cover.Left = 0

    loadImage()
    return Cover
end

return {
    width = width,
    create = create,
}
