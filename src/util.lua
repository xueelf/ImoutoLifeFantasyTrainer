local ROOT = 'assets/'

local function loadFile(filename)
    local stream = createMemoryStream()

    stream.loadFromFile(ROOT .. filename)
    return stream
end

local function loadFont(filename)
    local base = 'fonts/'
    local extension = '.ttf'
    local path = base .. filename .. extension
    local stream = loadFile(path)

    loadFontFromStream(stream)
end

local function playVoice(filename)
    local base = 'sounds/'
    local extension = '.wav'
    local path = base .. filename .. extension
    local stream = loadFile(path)

    playSound(stream)
end

return {
    loadFont = loadFont,
    playVoice = playVoice,
}
