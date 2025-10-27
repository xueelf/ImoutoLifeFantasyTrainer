local config = require("config")

local cover_width, cover_height = 180, config.window.height

local function create(owner)
    local Cover = createImage(owner)

    Cover.Width, Cover.Height = cover_width, cover_height
    Cover.Left = 0

    return Cover
end

return {
    width = cover_width,
    create = create,
}
