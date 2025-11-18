local config = require('config')

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

local function getRandomFileByFolder(path, exclude)
    local files = getFileList(path)

    if not exclude then
        return files[math.random(1, #files)]
    end
    local candidate = {}

    for _, file in ipairs(files) do
        if not string.endsWith(file, exclude) then
            table.insert(candidate, file)
        end
    end
    return candidate[math.random(1, #candidate)]
end

return {
    loadFont = loadFont,
    playVoice = playVoice,
    getRandomFileByFolder = getRandomFileByFolder,
}
