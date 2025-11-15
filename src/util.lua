local assets_path = "assets/"

local function loadFont(filename)
    local stream = createMemoryStream()
    local path = assets_path .. "fonts/" .. filename .. ".ttf"

    stream.loadFromFile(path)
    loadFontFromStream(stream)
end

local function playVoice(filename)
    local path = assets_path .. "sounds/" .. filename .. ".wav"
    local stream = createMemoryStream()

    stream.loadFromFile(path)
    playSound(stream)
end

return {
    loadFont = loadFont,
    playVoice = playVoice,
}
