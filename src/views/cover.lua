local config = require("config")

local width, height = 180, config.window.height

local function create(owner)
    local Cover = createImage(owner)

    Cover.Width, Cover.Height = width, height
    Cover.Left = 0

    return Cover
end

return {
    width = width,
    create = create,
}
